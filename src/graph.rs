use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};


/// Graph structure using adjacency lists.
/// Provides methods for loading an undirected graph from an edge list file.
pub struct Graph {
    pub adj: HashMap<usize, HashSet<usize>>, // adjacency list
}

impl Graph {
    pub fn new() -> Self {
        Graph { adj: HashMap::new() }
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.adj.entry(u).or_default().insert(v);
        self.adj.entry(v).or_default().insert(u); // undirected graph
    }

    pub fn load_from_file(path: &str) -> Self {
        let file = File::open(path).expect("Could not open file");
        let reader = BufReader::new(file);
        let mut graph = Graph::new();
        for line in reader.lines() {
            let line = line.unwrap();
            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            if parts.len() != 2 { continue; }
            let u = parts[0].parse::<usize>().unwrap();
            let v = parts[1].parse::<usize>().unwrap();
            graph.add_edge(u, v);
        }
        graph
    }

    pub fn neighbors(&self, u: usize) -> impl Iterator<Item = &usize> {
        self.adj.get(&u).into_iter().flatten()
    }

    pub fn nodes(&self) -> impl Iterator<Item = &usize> {
        self.adj.keys()
    }
}