mod tests;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{VecDeque, HashMap};
use std::error::Error;
use std::fmt;

// Custom error type for graph-related errors
#[derive(Debug)]
struct GraphError {
    details: String,
}

impl GraphError {
    fn new(msg: &str) -> GraphError {
        GraphError { details: msg.to_string() }
    }
}

impl fmt::Display for GraphError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for GraphError {}

fn read_file_safe(filename: &str) -> Result<Vec<(u32, u32)>, GraphError> {
    let file = File::open(filename).map_err(|_| GraphError::new("Could not open file"))?;
    let reader = BufReader::new(file);
    let mut edges = Vec::new();

    for line in reader.lines() {
        let line = line.map_err(|_| GraphError::new("Could not read line"))?;
        if line.starts_with('#') {
            continue;
        }
        let nodes: Vec<&str> = line.split('\t').collect();
        if nodes.len() < 2 {
            return Err(GraphError::new("Line format is incorrect"));
        }
        let from_node: u32 = nodes[0].parse().map_err(|_| GraphError::new("Invalid node format"))?;
        let to_node: u32 = nodes[1].parse().map_err(|_| GraphError::new("Invalid node format"))?;
        edges.push((from_node, to_node));
    }

    Ok(edges)
}

fn adjacency_list(edges: &[(u32, u32)]) -> HashMap<u32, Vec<u32>> {
    let mut adj_list = HashMap::new();
    for (from, to) in edges {
        adj_list.entry(*from).or_insert_with(Vec::new).push(*to);
    }
    adj_list
}

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

fn avg_degrees_of_separation(filename: &str) -> f64 {
    match read_file_safe(filename) {
        Ok(edges) => {
            let adj_list = adjacency_list(&edges);
            let avg_dists = bfs_all_nodes(&adj_list);
            let mut total_dist = 0.0;
            let mut num_nodes_with_valid_dist = 0;

            for (_, avg_dist) in &avg_dists {
                if !avg_dist.is_nan() {
                    total_dist += avg_dist;
                    num_nodes_with_valid_dist += 1;
                }
            }

            total_dist / num_nodes_with_valid_dist as f64
        },
        Err(_) => 0.0, // Return 0 if file could not be read or any error occurred
    }
}

fn main() {
    let filename = "email-net.txt";
    match read_file_safe(filename) {
        Ok(edges) => {
            let adj_list = adjacency_list(&edges);

            // Summarize graph information at the end
            let avg_degrees = avg_degrees_of_separation(filename);
            let graph_density = graph_density(&adj_list);
            let num_edges = edges.len();
            let num_nodes = adj_list.len();
            let max_deg_node = adj_list.keys().max().unwrap_or(&0);
            let min_deg_node = adj_list.keys().min().unwrap_or(&0);
            let self_loops = edges.iter().filter(|&&(from, to)| from == to).count();
            let single_connection_nodes: Vec<u32> = adj_list.iter()
                .filter(|(_, neighbors)| neighbors.len() == 1)
                .map(|(&node, _)| node)
                .collect();
            let complete_graph = adj_list.iter()
                .all(|(node, neighbors)| neighbors.len() == adj_list.len() - 1);

            println!("Graph Summary:");
            println!("Total nodes: {}", num_nodes);
            println!("Total edges: {}", num_edges);
            println!("Graph density: {:.4}", graph_density);
            println!("Average degrees of separation: {}", avg_degrees);
            println!("Node with the highest identifier: {}", max_deg_node);
            println!("Node with the lowest identifier: {}", min_deg_node);
            println!("Number of self-loops: {}", self_loops);
            println!("Nodes with exactly one connection: {:?}", single_connection_nodes);
            println!("The graph is{}a complete graph.", if complete_graph { " " } else { " not " });
        },
        Err(e) => {
            println!("Failed to read or process the file: {}", e);
        }
    }
}

// Additional helper function to calculate graph density
fn graph_density(adj_list: &HashMap<u32, Vec<u32>>) -> f64 {
    if adj_list.is_empty() {
        return 0.0;
    }
    let total_possible_connections = (adj_list.len() * (adj_list.len() - 1)) as f64;
    let actual_connections = adj_list.values().map(|neighbors| neighbors.len()).sum::<usize>() as f64 / 2.0; // Considering undirected graph
    actual_connections / total_possible_connections
}
