//! The Cycle represents a data type for
//! determining whether an undirected graph has a simple cycle.
//! The *has_cycle* operation determines whether the graph has
//! a cycle and, if so, the *cycle* operation returns one.
//!
//! This implementation uses depth-first search.
//! The constructor takes O(V + E) time in the
//! worst case, where *V* is the number of vertices and
//! *E* is the number of edges.
//! (The depth-first search part takes only *O(V)* time;
//! however, checking for self-loops and parallel edges takes
//! O(V + E) time in the worst case.)
//! Each instance method takes O(1) time.
//! It uses O(V) extra space (not including the graph).

use crate::common::Stack;
use crate::graph::IGraph;
use crate::ll::linked_list::Iter;

pub struct Cycle {
    marked: Vec<bool>,
    cycle: Option<Stack<usize>>,
    edge_to: Vec<usize>,
}

impl Cycle {
    pub fn new(g: &dyn IGraph) -> Self {
        let mut cycle = Self {
            marked: vec![false; g.V()],
            cycle: None,
            edge_to: vec![0; g.V()],
        };

        if !cycle.has_parallel_edges(g) {
            for s in 0..g.V() {
                if !cycle.marked[s] {
                    cycle.dfs(g, s, s);
                }
            }
        }

        cycle
    }

    /// Returns true if the graph G has a cycle.
    pub fn has_cycle(&self) -> bool {
        self.cycle.is_some()
    }

    /// Returns a cycle in the graph G.
    pub fn cycle(&self) -> Option<Iter<'_, usize>> {
        self.cycle.as_ref().map(|v| v.iter())
    }
}

impl Cycle {
    fn dfs(&mut self, g: &dyn IGraph, v: usize, u: usize) {
        self.marked[v] = true;
        for &w in g.adj(v) {
            // short circuit if cycle already found
            if self.cycle.is_some() {
                return;
            }

            if !self.marked[w] {
                self.edge_to[w] = v;
                self.dfs(g, w, v);
            } else if w != u {
                let mut cycle = Stack::default();
                let mut x = v;
                while x != w {
                    cycle.push(x);
                    x = self.edge_to[x];
                }
                cycle.push(w);
                cycle.push(v);
                self.cycle = Some(cycle);
            }
        }
    }

    // does this graph have two parallel edges?
    fn has_parallel_edges(&mut self, graph: &dyn IGraph) -> bool {
        self.marked.fill(false);

        for v in 0..graph.V() {
            for &w in graph.adj(v) {
                if self.marked[w] {
                    let mut cycle = Stack::default();
                    cycle.push(v);
                    cycle.push(w);
                    cycle.push(v);
                    self.cycle = Some(cycle);
                    return true;
                }
                self.marked[w] = true;
            }

            // reset so marked[v] = false for all v
            self.marked.fill(false);
        }

        false
    }
}
