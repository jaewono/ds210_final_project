use crate::graph::Graph;
use crate::simulate::simulate_spread;
use rand::seq::SliceRandom;
use rand::thread_rng;

/// Finds the top seed nodes that spread gossip to the most people.

/// # Arguments
/// * `graph` - Reference to the graph structure
/// * `num_trials` - Number of seed nodes to randomly sample
/// * `max_steps` - Maximum number of spread steps per simulation

/// A vector of (node ID, total spread count) sorted in descending order of spread.

pub fn find_top_spreaders(graph: &Graph, num_trials: usize, max_steps: usize) -> Vec<(usize, usize)> {
    let mut rng = thread_rng();
    let all_nodes: Vec<_> = graph.nodes().cloned().collect();
    let sampled_nodes = all_nodes.choose_multiple(&mut rng, num_trials);

    let mut results = Vec::new();

    for &seed in sampled_nodes {
        let spread = simulate_spread(graph, seed, max_steps);
        let total_spread = *spread.last().unwrap_or(&1); 
        results.push((seed, total_spread));
    }

    
    results.sort_by(|a, b| b.1.cmp(&a.1));
    results
}