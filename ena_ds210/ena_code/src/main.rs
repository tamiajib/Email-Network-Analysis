mod graph;
use crate::graph::build_graph;

mod analysis;
use crate::analysis::{calculate_degree_centrality, calculate_betweenness_centrality, calculate_average_path_length, validate_six_degrees};

fn main() {
    let edges = load_edges_from_file("email-Enron.txt"); // Load edges dynamically

    let graph = build_graph(&edges);
    let degree_centrality = calculate_degree_centrality(&graph);
    let betweenness_centrality = calculate_betweenness_centrality(&graph);
    let average_path_length = calculate_average_path_length(&graph);
    let is_six_degrees = validate_six_degrees(average_path_length);

    println!("Degree Centrality: {:?}", degree_centrality);
    println!("Betweenness Centrality: {:?}", betweenness_centrality);
    println!("Average Path Length: {}", average_path_length);
    println!("Validation of Six Degrees Theory: {}", is_six_degrees);
}

// Function to dynamically load edges from a file
fn load_edges_from_file(file_path: &str) -> Vec<(usize, usize)> {
    use std::fs::File;
    use std::io::{BufReader, BufRead};

    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut edges = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let parts: Vec<&str> = line.split_whitespace().collect(); // Split by whitespace
        if parts.len() == 2 {
            let s_node: usize = parts[0].parse().unwrap();
            let r_node: usize = parts[1].parse().unwrap();
            edges.push((s_node, r_node));
        }
    }

    edges
}
