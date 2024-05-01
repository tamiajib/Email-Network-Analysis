use std::collections::{HashMap, HashSet};
use std::ops::AddAssign; // Ensuring we can use add_assign for f64

// Function to calculate degree centrality
pub fn calculate_degree_centrality(graph: &HashMap<usize, Vec<usize>>) -> HashMap<usize, f64> {
    let mut degree_centrality: HashMap<usize, f64> = HashMap::new();
    for (node, neighbors) in graph {
        let degree = neighbors.len() as f64 / (graph.len() - 1).max(1) as f64; // Ensure no division by zero
        degree_centrality.insert(*node, degree);
    }
    degree_centrality
}

// Function to calculate betweenness centrality
pub fn calculate_betweenness_centrality(graph: &HashMap<usize, Vec<usize>>) -> HashMap<usize, f64> {
    let mut betweenness_centrality: HashMap<usize, f64> = HashMap::new();
    for node in graph.keys() {
        let mut num_paths: HashMap<usize, usize> = HashMap::new();
        let mut paths: HashMap<usize, f64> = HashMap::new();
        let mut queue = Vec::new();
        let mut visited = HashSet::new();

        num_paths.insert(*node, 1);
        queue.push(*node);

        // Breadth-First Search (BFS)
        while !queue.is_empty() {
            let current_node = queue.pop().unwrap();
            if let Some(&current_paths) = num_paths.get(&current_node) {
                visited.insert(current_node);
                for &neighbor in &graph[&current_node] {
                    if visited.insert(neighbor) {
                        queue.push(neighbor);
                        let entry = num_paths.entry(neighbor).or_insert(0);
                        *entry = entry.checked_add(current_paths).unwrap_or(usize::MAX); // Handle overflow
                    }
                }
            }
        }

        // Calculate betweenness centrality based on paths
        for (node, &num_path) in num_paths.iter() {
            let entry = paths.entry(*node).or_insert(0.0);
            *entry += num_path as f64;
        }

        for (node, &path_count) in paths.iter() {
            betweenness_centrality.entry(*node).or_insert(0.0).add_assign(path_count);
        }
    }

    // Normalize the Betweenness Centrality Values
    for value in betweenness_centrality.values_mut() {
        *value /= 2.0; // Normalize to account for paths counted twice
    }

    betweenness_centrality
}

// Function to calculate average path length
pub fn calculate_average_path_length(graph: &HashMap<usize, Vec<usize>>) -> f64 {
    let mut total_path_length: usize = 0;
    let mut total_paths: usize = 0;

    for &node in graph.keys() {
        let mut distances: HashMap<usize, usize> = HashMap::new();
        let mut visited = HashSet::new();
        let mut queue = vec![node];
        distances.insert(node, 0);

        while let Some(current_node) = queue.pop() {
            for &neighbor in &graph[&current_node] {
                if visited.insert(neighbor) {
                    let new_distance = distances.get(&current_node).unwrap().checked_add(1).unwrap_or(usize::MAX);
                    distances.insert(neighbor, new_distance);
                    queue.push(neighbor);
                }
            }
        }

        for distance in distances.values() {
            total_path_length = total_path_length.checked_add(*distance).unwrap_or_else(|| {
                eprintln!("Overflow occurred in total path length calculation.");
                return usize::MAX; // Signal of error state
            });
            total_paths += 1;
        }
    }

    if total_paths > 0 {
        total_path_length as f64 / total_paths as f64
    } else {
        0.0 // Avoid division by zero if no paths are found
    }
}

// Function to validate the "six degrees of separation" theory
pub fn validate_six_degrees(average_path_length: f64) -> bool {
    average_path_length <= 6.0
}
