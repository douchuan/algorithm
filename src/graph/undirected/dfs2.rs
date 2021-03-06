//! The *NonRecursiveDFS* represents a data type for finding
//! the vertices connected to a source vertex *s* in the undirected
//! graph.
//!
//! This implementation uses a non recursive version of depth-first search
//! with an explicit stack.

use crate::common::Stack;
use crate::graph::IGraph;

pub struct NonRecursiveDFS {
    marked: Vec<bool>,
}

impl NonRecursiveDFS {
    pub fn new(graph: &dyn IGraph, s: usize) -> Self {
        let mut dfs = Self {
            marked: vec![false; graph.V()],
        };
        dfs.mark(graph, s);
        dfs
    }

    pub fn marked(&self, v: usize) -> bool {
        self.marked[v]
    }
}

impl NonRecursiveDFS {
    fn mark(&mut self, graph: &dyn IGraph, s: usize) {
        let mut adj = Vec::with_capacity(graph.V());
        for v in 0..graph.V() {
            adj.push(graph.adj(v));
        }

        let mut stack = Stack::default();
        stack.push(s);
        self.marked[s] = true;

        while let Some(&v) = stack.peek() {
            if let Some(&w) = adj[v].next() {
                if !self.marked[w] {
                    self.marked[w] = true;
                    stack.push(w);
                }
            } else {
                let _ = stack.pop();
            }
        }
    }
}
