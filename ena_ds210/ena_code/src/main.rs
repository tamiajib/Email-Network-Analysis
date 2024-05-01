mod graph;
use crate::graph::build_graph;

mod analysis;
use crate::analysis::{calculate_degree_centrality, calculate_betweenness_centrality, calculate_average_path_length, validate_six_degrees};

fn main() {
    println!("Loading edges...");
    let edges = load_edges_from_file("email-Enron.txt"); // Ensure this file path is correct
    println!("Edges loaded: {}", edges.len());

    println!("Building graph...");
    let graph = build_graph(&edges);
    println!("Graph built with {} nodes.", graph.len());

    println!("Calculating degree centrality...");
    let degree_centrality = calculate_degree_centrality(&graph);
    println!("Degree Centrality calculated for {} nodes.", degree_centrality.len());

    // Display top ten nodes based on degree centrality
    let mut centrality_vec: Vec<_> = degree_centrality.iter().collect();
    centrality_vec.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap()); // Sort by centrality value in descending order
    println!("Top 10 nodes by degree centrality:");
    for (node, centrality) in centrality_vec.iter().take(10) {
        println!("Node {}: {}", node, centrality);
    }

    println!("Calculating betweenness centrality...");
    let betweenness_centrality = calculate_betweenness_centrality(&graph);
    println!("Betweenness Centrality calculated.");

    println!("Calculating average path length...");
    let average_path_length = calculate_average_path_length(&graph);
    println!("Average Path Length: {}", average_path_length);

    println!("Validating Six Degrees of Separation theory...");
    let is_six_degrees = validate_six_degrees(average_path_length);
    println!("Validation of Six Degrees of Separation: {}", is_six_degrees);
}

fn load_edges_from_file(file_path: &str) -> Vec<(usize, usize)> {
    use std::fs::File;
    use std::io::{BufReader, BufRead};

    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut edges = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let s_node: usize = parts[0].parse().expect("Error parsing source node");
            let r_node: usize = parts[1].parse().expect("Error parsing recipient node");
            edges.push((s_node, r_node));
        }
    }

    edges
}
