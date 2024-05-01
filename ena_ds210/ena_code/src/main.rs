mod graph;
use crate::graph::build_graph;

mod analysis;
use crate::analysis::{calculate_degree_centrality, calculate_betweenness_centrality, calculate_average_path_length, validate_six_degrees};

fn load_edges_from_file(file_path: &str) -> Vec<(usize, usize)> {
    use std::fs::File;
    use std::io::{BufReader, BufRead};

    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut edges = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if line.starts_with('#') {
            continue; // Skip comment lines
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let s_node: usize = parts[0].parse().unwrap();
            let r_node: usize = parts[1].parse().unwrap();
            edges.push((s_node, r_node));
        }
    }
    edges
}

fn print_top_centrality<K, V: PartialOrd + Copy>(centrality: &HashMap<K, V>, top_n: usize) {
    let mut centrality_vec: Vec<_> = centrality.iter().collect();
    // Sort by value in descending order
    centrality_vec.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    println!("Top {} nodes:", top_n);
    for (idx, (&node, &centrality)) in centrality_vec.iter().enumerate().take(top_n) {
        println!("Node {}: Centrality = {}", node, centrality);
    }
}

fn main() {
    println!("Loading edges...");
    let edges = load_edges_from_file("email-net.txt");
    println!("Edges loaded: {}", edges.len());

    println!("Building graph...");
    let graph = build_graph(&edges);
    println!("Graph built with {} nodes.", graph.len());

    println!("Calculating degree centrality...");
    let degree_centrality = calculate_degree_centrality(&graph);
    print_top_centrality(&degree_centrality, 10);

    println!("Calculating betweenness centrality...");
    let betweenness_centrality = calculate_betweenness_centrality(&graph);
    print_top_centrality(&betweenness_centrality, 10);

    println!("Calculating average path length...");
    let average_path_length = calculate_average_path_length(&graph);
    println!("Average Path Length: {}", average_path_length);

    println!("Validating Six Degrees of Separation theory...");
    let is_six_degrees = validate_six_degrees(average_path_length);
    println!("Validation of Six Degrees of Separation: {}", is_six_degrees);
}
