use std::collections::HashMap;

pub fn build_graph(edges: &[(usize, usize)]) -> HashMap<usize, Vec<usize>> {
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    for &(src, dest) in edges {
        graph.entry(src).or_default().push(dest);
        graph.entry(dest).or_default().push(src);  // Assuming an undirected graph
    }
    graph
}
