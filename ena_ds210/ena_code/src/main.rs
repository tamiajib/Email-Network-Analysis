mod graph;
mod analysis;

use graph::{read_file, adjacency_list};
use analysis::{bfs_all_nodes, avg_degrees_of_separation};

fn main() {
    let filename = "email-net.txt";
    match read_file(filename) {
        Ok(edges) => {
            let adj_list = adjacency_list(&edges);
            println!("Graph built with {} nodes.", adj_list.len());
            
            let avg_dists = bfs_all_nodes(&adj_list);
            for (node, avg_dist) in avg_dists {
                println!("Node {}: Average Distance {:.2}", node, avg_dist);
            }

            let avg_degrees = avg_degrees_of_separation(&adj_list);
            println!("Average degrees of separation: {:.2}", avg_degrees);
        },
        Err(e) => println!("Failed to read or build graph: {}", e),
    }
}
