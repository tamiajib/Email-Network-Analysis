mod analysis {
    use std::collections::{HashMap, HashSet};

    // Function to calculate degree centrality
    pub fn calculate_degree_centrality(graph: &HashMap<usize, Vec<usize>>) -> HashMap<usize, f64> {
        let mut degree_centrality: HashMap<usize, f64> = HashMap::new();
        for (node, neighbors) in graph {
            let degree_cent = neighbors.len() as f64 / (graph.len() - 1) as f64;
            degree_centrality.insert(*node, degree_cent);
        }
        degree_centrality
    }

    // Function to calculate betweenness centrality
    pub fn calculate_betweenness_centrality(graph: &HashMap<usize, Vec<usize>>) -> HashMap<usize, f64> {
        let mut betweenness_centrality: HashMap<usize, f64> = HashMap::new();
        for node in graph.keys() {
            let mut num_paths = HashMap::new();
            let mut stack = Vec::new();
            let mut paths = HashMap::new();
            let mut queue = Vec::new();
            let mut visited = HashSet::new();

            num_paths.insert(*node, 1);
            queue.push(*node);

            // Breadth-First Search (BFS)
            while !queue.is_empty() {
                let current_node = queue.remove(0);
                visited.insert(current_node);

                for neighbor in graph.get(&current_node).unwrap() {
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

                // Dependency Calculation
                if queue.is_empty() && !stack.is_empty() {
                    let node = stack.pop().unwrap();
                    if !paths.contains_key(&node) {
                        paths.insert(node, 0.0);
                    }
                    paths.insert(node, paths.get(&node).unwrap() + 1.0);

                    for neighbor in graph.get(&node).unwrap() {
                        let np = *num_paths.get(&node).unwrap() as f64;
                        let path = *paths.get(&node).unwrap() as f64;
                        let path_share = path / np;
                        if !betweenness_centrality.contains_key(neighbor) {
                            betweenness_centrality.insert(*neighbor, 0.0);
                        }
                        betweenness_centrality.insert(*neighbor, betweenness_centrality.get(neighbor).unwrap() + path_share);
                        if *neighbor != node {
                            let npp = *num_paths.get(neighbor).unwrap_or(&0) as f64;
                            if !betweenness_centrality.contains_key(&node) {
                                betweenness_centrality.insert(node, 0.0);
                            }
                            betweenness_centrality.insert(node, betweenness_centrality.get(&node).unwrap() + npp * path_share);
                        }
                    }
                }
            }
        }

        // Normalize the Betweeness Centrality Values 
        for (_, bc) in betweenness_centrality.iter_mut() {
            *bc /= 2.0;
        }

        betweenness_centrality
    }

    // Function to calculate average path length
    pub fn calculate_average_path_length(graph: &HashMap<usize, Vec<usize>>) -> f64 {
        let mut total_path_length = 0;
        let mut total_paths = 0;

        for start_node in graph.keys() {
            let mut distances = HashMap::new();
            let mut visited = HashSet::new();
            let mut queue = Vec::new();

            distances.insert(*start_node, 0);
            queue.push(*start_node);

            while let Some(current_node) = queue.pop() {
                if !visited.contains(&current_node) {
                    visited.insert(current_node);
                    for neighbor in graph.get(&current_node).unwrap() {
                        let distance = *distances.get(&current_node).unwrap() + 1;
                        if !distances.contains_key(neighbor) {
                            distances.insert(*neighbor, distance);
                            queue.push(*neighbor);
                        }
                    }
                }
            }

            for (_, distance) in distances.iter() {
                total_path_length += distance;
                total_paths += 1;
            }
        }

        let average_path_length = total_path_length as f64 / total_paths as f64;
        average_path_length
    }

    // Function to validate the "six degrees of separation" theory
    pub fn validate_six_degrees(average_path_length: f64) -> bool {
        let six_degrees = 6.0;
        average_path_length <= six_degrees
    }
}
