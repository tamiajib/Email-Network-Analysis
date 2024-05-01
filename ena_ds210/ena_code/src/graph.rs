use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;

pub type Graph = HashMap<u32, Vec<u32>>;

pub fn read_file(filename: &str) -> io::Result<Vec<(u32, u32)>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut edges = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let nodes: Vec<&str> = line.split_whitespace().collect();
        let from_node = nodes[0].parse::<u32>().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let to_node = nodes[1].parse::<u32>().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        edges.push((from_node, to_node));
    }

    Ok(edges)
}

pub fn adjacency_list(edges: &[(u32, u32)]) -> Graph {
    let mut graph = Graph::new();
    for &(from, to) in edges {
        graph.entry(from).or_insert_with(Vec::new).push(to);
        graph.entry(to).or_insert_with(Vec::new).push(from);  // Assuming it's an undirected graph
    }
    graph
}
