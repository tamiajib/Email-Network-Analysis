#[cfg(test)]
mod tests;

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::{VecDeque, HashMap};

// Function to read edges from a file
fn read_file(filename: &str) -> Vec<(u32, u32)> {
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);

    let mut edges = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        if line.starts_with('#') {
            continue;
        }
        let nodes: Vec<&str> = line.split('\t').collect();
        let from_node: u32 = nodes[0].parse().expect("Invalid from node");
        let to_node: u32 = nodes[1].parse().expect("Invalid to node");
        edges.push((from_node, to_node));
    }

    edges
}

// Function to create an adjacency list from edges
fn adjacency_list(edges: &[(u32, u32)]) -> HashMap<u32, Vec<u32>> {
    let mut adj_list = HashMap::new();
    for (from, to) in edges {
        adj_list.entry(*from).or_insert_with(Vec::new).push(*to);
        adj_list.entry(*to).or_insert_with(Vec::new).push(*from); // Assuming an undirected graph
    }
    adj_list
}

// Function to perform Breadth-First Search (BFS) from a given node
fn bfs(node: u32, adj_list: &HashMap<u32, Vec<u32>>) -> HashMap<u32, u32> {
    let mut dist = HashMap::new();
    let mut visited = HashMap::new();
    let mut q = VecDeque::new();

    dist.insert(node, 0);
    visited.insert(node, true);
    q.push_back(node);

    while !q.is_empty() {
        let curr_node = q.pop_front().unwrap();

        for &neigh in adj_list.get(&curr_node).unwrap_or(&vec![]) {
            if !visited.contains_key(&neigh) {
                visited.insert(neigh, true);
                dist.insert(neigh, dist[&curr_node] + 1);
                q.push_back(neigh);
            }
        }
    }

    dist
}

// Function to calculate the average distances from all nodes to other nodes
fn bfs_all_nodes(adj_list: &HashMap<u32, Vec<u32>>) -> HashMap<u32, f64> {
    let mut avg_dists = HashMap::new();

    for node in adj_list.keys() {
        let dist = bfs(*node, adj_list);
        let sum_dists = dist.values().sum::<u32>() as f64;
        let num_nodes = dist.len() as f64 - 1.0; // subtract 1 for the starting node
        let avg_dist = sum_dists / num_nodes;
        avg_dists.insert(*node, avg_dist);
    }

    avg_dists
}

// Function to find the node with the minimum degree
fn min_degree_node(adj_list: &HashMap<u32, Vec<u32>>) -> Option<u32> {
    adj_list.iter().map(|(node, neighbors)| (neighbors.len(), *node)).min().map(|(_, node)| node)
}

// Function to find the node with the maximum degree
fn max_degree_node(adj_list: &HashMap<u32, Vec<u32>>) -> Option<u32> {
    adj_list.iter().map(|(node, neighbors)| (neighbors.len(), *node)).max().map(|(_, node)| node)
}

// Function to calculate the average degrees of separation
fn avg_degrees_of_separation(filename: &str) -> f64 {
    let edges = read_file(filename);
    let adj_list = adjacency_list(&edges);
    let avg_dists = bfs_all_nodes(&adj_list);
    let mut total_dist = 0.0;
    let mut num_nodes_with_valid_dist = 0;

    for (node, avg_dist) in &avg_dists {
        if !avg_dist.is_nan() {
            total_dist += avg_dist;
            num_nodes_with_valid_dist += 1;
        }
    }

    total_dist / num_nodes_with_valid_dist as f64
}
// Function to calculate the density of the graph
fn graph_density(adj_list: &HashMap<u32, Vec<u32>>) -> f64 {
    let num_nodes = adj_list.len() as f64;
    let num_edges = adj_list.values().map(|neighbors| neighbors.len() as f64).sum::<f64>() / 2.0; // Assuming an undirected graph
    num_edges / (num_nodes * (num_nodes - 1.0) / 2.0)
}

// Function to print the adjacency list
fn print_adjacency_list(adj_list: &HashMap<u32, Vec<u32>>) {
    println!("Adjacency List:");
    for (node, neighbors) in adj_list {
        println!("Node {}: {:?}", node, neighbors);
    }
}

fn main() {
    let filename = "email-net.txt";
    let edges = read_file(filename);
    let adj_list = adjacency_list(&edges);

    println!("Graph built with {} nodes and {} edges.", adj_list.len(), edges.len());

    let avg_dists = bfs_all_nodes(&adj_list);
    for (node, avg_dist) in &avg_dists {
        println!("Node {}: average distance {}", node, avg_dist);
    }

    let avg_degrees = avg_degrees_of_separation(filename);
    println!("Average degrees of separation: {}", avg_degrees);

    if let Some(min_node) = min_degree_node(&adj_list) {
        println!("Node {} has the minimum degree.", min_node);
    } else {
        println!("Graph is empty.");
    }

    if let Some(max_node) = max_degree_node(&adj_list) {
        println!("Node {} has the maximum degree.", max_node);
    } else {
        println!("Graph is empty.");
    } 
}