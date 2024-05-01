use crate::graph::Graph;
use std::collections::{HashMap, VecDeque, HashSet};

pub fn bfs(node: u32, adj_list: &Graph) -> HashMap<u32, u32> {
    let mut distances = HashMap::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    distances.insert(node, 0);
    visited.insert(node);
    queue.push_back(node);

    while let Some(curr_node) = queue.pop_front() {
        for &neigh in adj_list.get(&curr_node).unwrap_or(&Vec::new()) {
            if visited.insert(neigh) {
                distances.insert(neigh, distances[&curr_node] + 1);
                queue.push_back(neigh);
            }
        }
    }

    distances
}

pub fn bfs_all_nodes(adj_list: &Graph) -> HashMap<u32, f64> {
    let mut avg_dists = HashMap::new();
    for &node in adj_list.keys() {
        let distances = bfs(node, adj_list);
        let sum_distances = distances.values().sum::<u32>() as f64;
        let num_nodes = distances.len() as f64;
        avg_dists.insert(node, sum_distances / num_nodes);
    }
    avg_dists
}

pub fn avg_degrees_of_separation(adj_list: &Graph) -> f64 {
    let avg_dists = bfs_all_nodes(adj_list);
    let total_dist = avg_dists.values().sum::<f64>();
    let num_nodes = avg_dists.len() as f64;
    total_dist / num_nodes
}
