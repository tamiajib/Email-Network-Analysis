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
        graph.entry(*s_node).or_insert_with(Vec::new).push(*r_node);
        graph.entry(*s_node).or_insert_with(Vec::new).push(*r_node);
    }
    //Degree Centrality {Number of Neighbours/Total Number of Nodes - 1}
    //(Subtracing 1 from the total number of nodes is to normalize the dgree centrality measure)
    let mut degree_centrality: HashMap<usize, f64> = HashMap::new();
    for(node, neighbors) in &graph {
        let degree_cent = neighbors.len() as f64 / (graph.len() - 1) as f64;
        degree_centrality.insert(*node, degree_cent);
    }
    //Betweenness Centrality {Brandes' Algorithm}
    let mut betweenness_centrality: HashMap<usize, f64>= HashMap::new();
    for node in graph.keys() {
        let mut num_paths = HashMap::new();
        let mut stack = Vec::new(); 
        let mut paths = HashMap::new(); 
        let mut queue = Vec::new(); 
        let mut visited = HashSet::new();

        num_paths.insert(*node, 1);
        queue.push(*node);
        
        //Breadth-First Search (BFS)
        while !queue.is_empty() {
            let current_node = queue.remove(0); 
            visited.insert(current_node); 

            for neighbor in graph.get(current_node).unwrap() {
                if !visited.contains(neighbor) {
                    if !queue.contains(neighbor) {
                        queue.push(*neighbor);
                    }
                    if !stack.contains(&current_node) {
                        stack.push(current_node);
                    }
                    if !num_paths.contains_key(neighbor) {
                        num_paths.insert(*neighbor, 0); 
                    } 
                    num_paths.insert(*neighbor, num_paths.get(neighbor).unwrap() + num_paths.get(&current_node).unwrap());
                }
            }
            //Depenency Calculation 
        }


    }

}
