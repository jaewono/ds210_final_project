use rand::seq::IteratorRandom;
use rand::thread_rng;
use std::collections::{HashSet, VecDeque};

use crate::graph::Graph;

/// Simulates gossip spread in the graph starting from a given seed node.
/// # Arguments
/// * `graph` - The graph to simulate on
/// * `start_node` - The initial seed node where gossip starts
/// * `max_steps` - Number of time steps to simulate (depth of BFS)
/// # Returns
/// A vector where each element at index `i` represents the number of people who heard the gossip
/// up to step `i+1`.
pub fn simulate_spread(graph: &Graph, start_node: usize, max_steps: usize) -> Vec<usize> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut spread_log = Vec::new();

    visited.insert(start_node);
    queue.push_back(start_node);
    spread_log.push(1); // Step 1: only seed node heard the gossip

    for _ in 0..max_steps {
        let mut next_queue = VecDeque::new();

        for &node in &queue {
            for &neighbor in graph.neighbors(node) {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    next_queue.push_back(neighbor);
                }
            }
        }

        if next_queue.is_empty() {
            break;
        }

        queue = next_queue;
        spread_log.push(visited.len());
    }

    spread_log
}

/// Selects and returns a random node from the graph.
pub fn get_random_node(graph: &Graph) -> usize {
    let mut rng = thread_rng();
    *graph.adj.keys().choose(&mut rng).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Graph;

    #[test]
    fn test_simple_spread() {
        let mut graph = Graph { adj: Default::default() };
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let spread = simulate_spread(&graph, 1, 4);
        assert_eq!(spread, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_spread_stops_when_disconnected() {
        let mut graph = Graph { adj: Default::default() };
        graph.add_edge(1, 2);
        graph.add_edge(3, 4);

        let spread = simulate_spread(&graph, 1, 3);
        assert_eq!(spread, vec![1, 2]); // only 1 and 2 are connected
    }
}