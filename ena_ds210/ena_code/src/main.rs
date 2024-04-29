mod graph;
use crate::graph::build_graph;

mod analysis;
use crate::analysis::calculate_degree_centrality;
use crate::analysis::calculate_betweenness_centrality;
use crate::analysis::calculate_average_path_length;
use crate::analysis::validate_six_degrees;

fn main() {
    let graph = build_graph("email-Enron.txt");
    let degree_centrality = calculate_degree_centrality(&graph);
    let betweenness_centrality = calculate_betweenness_centrality(&graph);
    let average_path_length = calculate_average_path_length(&graph);
    let is_six_degrees = validate_six_degrees(average_path_length);

    println!("Degree Centrality: {:?}", degree_centrality);
    println!("Betweenness Centrality: {:?}", betweenness_centrality);
    println!("Average Path Length: {}", average_path_length);
    println!("Validation of Theory: {}", is_six_degrees);
}