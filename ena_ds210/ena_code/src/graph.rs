use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, BufRead};

// Function to build the graph
pub fn build_graph(file_path: &str) -> HashMap<usize, Vec<usize>> {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut edges: Vec<(usize, usize)> = vec![];

    for line in reader.lines().skip(5) {
        let line = line.expect("Failed to read line");
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() == 2 {
            let s_node: usize = parts[0].parse().unwrap();
            let r_node: usize = parts[1].parse().unwrap();
            edges.push((s_node, r_node));
        }
    }

    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();

    for (s_node, r_node) in edges {
        graph.entry(s_node).or_insert_with(Vec::new).push(r_node);
        graph.entry(r_node).or_insert_with(Vec::new).push(s_node); // Include both directions
    }

    graph
}
