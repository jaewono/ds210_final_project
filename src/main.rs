mod graph;
mod simulate;
mod analyze;

use graph::Graph;
use simulate::{simulate_spread, get_random_node};
use analyze::find_top_spreaders;
use std::fs::File;
use std::io::Write;

/// Main driver of the gossip spread simulation.
/// Loads the graph, runs the simulation, saves results, and prints analysis.

fn main() {
    // 1. Load the graph from the edge list
    let path = "facebook_combined.txt";
    let graph = Graph::load_from_file(path);
    println!("Graph loaded with {} nodes", graph.adj.len());

    // 2. Pick a seed node and simulate gossip spreading
    let seed = get_random_node(&graph);
    println!("Starting gossip from node: {}", seed);

    let max_steps = 5;
    let spread_result = simulate_spread(&graph, seed, max_steps);

    println!("Spread result from seed node {}:", seed);
    for (step, count) in spread_result.iter().enumerate() {
        println!("  Step {} → {} people have heard the gossip", step + 1, count);
    }

    // 3. Save result as CSV
    save_spread_to_csv(&spread_result, "spread_log.csv");
    println!("Saved spread log to 'spread_log.csv'");

    // 4. Run analysis on top spreaders
    println!("Running spreader analysis on 50 random nodes...");
    let top_spreaders = find_top_spreaders(&graph, 50, max_steps);

    println!("Top 10 Gossip Spreaders (in {} steps):", max_steps);
    for (rank, (node, total)) in top_spreaders.iter().take(10).enumerate() {
        println!("  #{:<2} Node {:<5} spread to {:>3} people", rank + 1, node, total);
    }
}

// Save spread result to CSV file for external plotting
fn save_spread_to_csv(data: &[usize], filename: &str) {
    let mut file = File::create(filename).expect("Cannot create file");
    writeln!(file, "Step,Spread").unwrap();
    for (step, count) in data.iter().enumerate() {
        writeln!(file, "{},{}", step + 1, count).unwrap();
    }
}
