use std::collections::HashMap;
use std::fs:File;
use std::io::BufReader;

fn main() {
    //Read File + Extract Sender/Recipient IDs
    let file = File::open("email-Enron.txt").expect("Failed to open file");
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
    //Build Undirected Graph Representation
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new(); 

    for (s_node, r_node) in edges {
        graph.entry(s_node).or_insert_with(Vec::new).push(r_node);
        graph.entry(s_node).or_insert_with(Vec::new).push(r_node);
    }
}
