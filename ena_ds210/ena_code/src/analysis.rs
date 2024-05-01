use std::collections::{HashMap, VecDeque, HashSet};
use rayon::prelude::*;
use std::sync::{Mutex, Arc};

type Graph = HashMap<usize, Vec<usize>>;

pub fn calculate_degree_centrality(graph: &Graph) -> HashMap<usize, f64> {
    let mut degree_centrality: HashMap<usize, f64> = HashMap::new();
    let node_count = graph.len() as f64;
    for (node, neighbors) in graph {
        degree_centrality.insert(*node, neighbors.len() as f64 / (node_count - 1.0)); // Ensure no division by zero
    }
    degree_centrality
}

pub fn calculate_betweenness_centrality(graph: &Graph) -> HashMap<usize, f64> {
    let betweenness_centrality = Arc::new(Mutex::new(HashMap::new()));
    graph.keys().par_bridge().for_each(|&node| {
        let mut local_paths: HashMap<usize, usize> = HashMap::new();
        let mut queue = VecDeque::new();
        let visited = Arc::new(Mutex::new(HashSet::new()));

        local_paths.insert(node, 1);
        queue.push_back(node);

        while let Some(current_node) = queue.pop_front() {
            let mut visited = visited.lock().unwrap();
            visited.insert(current_node);
            
            if let Some(&current_paths) = local_paths.get(&current_node) {
                for &neighbor in &graph[&current_node] {
                    if visited.insert(neighbor) {
                        queue.push_back(neighbor);
                        let entry = local_paths.entry(neighbor).or_insert(0);
                        *entry += current_paths;
                    }
                }
            }
        }

        let mut bc = betweenness_centrality.lock().unwrap();
        for (&node, &count) in local_paths.iter() {
            *bc.entry(node).or_insert(0.0) += count as f64;
        }
    });
    
    let mut final_centrality = HashMap::new();
    for (&k, &v) in betweenness_centrality.lock().unwrap().iter() {
        final_centrality.insert(k, v / 2.0);  // normalization step
    }
    final_centrality
}

pub fn calculate_average_path_length(graph: &Graph) -> f64 {
    let mut total_path_length: usize = 0;
    let mut total_paths: usize = 0;

    for &node in graph.keys() {
        let mut distances: HashMap<usize, usize> = HashMap::new();
        let visited = Arc::new(Mutex::new(HashSet::new()));
        let mut queue = VecDeque::new();
        queue.push_back(node);
        distances.insert(node, 0);

        while let Some(current_node) = queue.pop_front() {
            let mut visited = visited.lock().unwrap();
            for &neighbor in &graph[current_node] {
                if visited.insert(neighbor) {
                    let new_distance = distances.get(&current_node).unwrap().checked_add(1).unwrap_or(usize::MAX);
                    distances.insert(neighbor, new_distance);
                    queue.push_back(neighbor);
                }
            }
        }

        for distance in distances.values() {
            total_path_length = total_path_length.checked_add(*distance).unwrap_or_else(|| usize::MAX);
            total_paths += 1;
        }
    }

    if total_paths > 0 {
        total_path_length as f64 / total_paths as f64
    } else {
        0.0 // Avoid division by zero if no paths are found
    }
}

pub fn validate_six_degrees(average_path_length: f64) -> bool {
    average_path_length <= 6.0
}
