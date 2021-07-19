//! Cycles in digraphs.
//! If job x must be completed before job y, job y before job z,
//! and job z before job x, then someone has made a mistake,
//! because those three constraints cannot all be satisfied.
//! In general, if a precedence-constrained scheduling problem
//! has a directed cycle, then there is no feasible solution.
//! To check for such errors, we need to be able to solve the
//! following problem:
//!
//! Directed cycle detection.
//! Does a given digraph have a directed cycle?
//! If so, find the vertices on some such cycle, in order from
//! some vertex back to itself.

use crate::graph::IGraph;
use std::slice::Iter;

/// cycle-finding
pub struct DirectedCycle {
    marked: Vec<bool>,
    edge_to: Vec<usize>,
    cycle: Option<Vec<usize>>,
    on_stack: Vec<bool>,
}

impl DirectedCycle {
    pub fn new(graph: &Box<dyn IGraph>) -> Self {
        let marked = vec![false; graph.V()];
        let edge_to = vec![0; graph.V()];
        let cycle = None;
        let on_stack = vec![false; graph.V()];
        let mut c = Self {
            marked,
            edge_to,
            cycle,
            on_stack,
        };
        for s in 0..graph.V() {
            if !c.marked[s] {
                c.dfs(graph, s);
            }
        }
        c
    }

    /// does G have a directed cycle?
    pub fn has_cycle(&self) -> bool {
        self.cycle.is_some()
    }

    /// vertices on a cycle (if one exists)
    pub fn cycle(&self) -> Option<Iter<'_, usize>> {
        self.cycle.as_ref().and_then(|v| Some(v.iter()))
    }
}

impl DirectedCycle {
    fn dfs(&mut self, graph: &Box<dyn IGraph>, v: usize) {
        self.on_stack[v] = true;
        self.marked[v] = true;
        for &w in graph.adj(v) {
            if self.has_cycle() {
                return;
            } else if !self.marked[w] {
                self.edge_to[w] = v;
                self.dfs(graph, w);
            } else if self.on_stack[w] {
                let mut cycle = Vec::new();
                let mut x = v;
                while x != w {
                    cycle.push(x);
                    x = self.edge_to[x];
                }
                cycle.push(w);
                cycle.push(v);
                cycle.reverse();
                self.cycle = Some(cycle);
            }
        }
        self.on_stack[v] = false;
    }
}
