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

use crate::common::Stack;
use crate::graph::shortest::DirectedEdge;
use crate::graph::{IEWDigraph, IGraph};
use crate::ll::linked_list::Iter;

/// The DirectedCycle represents a data type for
/// determining whether a digraph has a directed cycle.
/// The has_cycle operation determines whether the digraph has
/// a simple directed cycle and, if so, the cycle operation
/// returns one.
/// This implementation uses depth-first search.
pub struct DirectedCycle {
    cycle: Option<Stack<usize>>,
    edge_to: Vec<usize>,
    marked: Vec<bool>,
    on_stack: Vec<bool>,
}

/// The EdgeWeightedDirectedCycle represents a data type for
/// determining whether an edge-weighted digraph has a directed cycle.
/// The has_cycle operation determines whether the edge-weighted
/// digraph has a directed cycle and, if so, the cycle operation
/// returns one.
/// This implementation uses depth-first search.
pub struct EdgeWeightedDirectedCycle {
    cycle: Option<Stack<DirectedEdge>>,
    edge_to: Vec<Option<DirectedEdge>>,
    marked: Vec<bool>,
    on_stack: Vec<bool>,
}

impl DirectedCycle {
    /// does G have a directed cycle?
    pub fn has_cycle(&self) -> bool {
        self.cycle.is_some()
    }

    /// vertices on a cycle (if one exists)
    pub fn cycle(&self) -> Option<Iter<'_, usize>> {
        self.cycle.as_ref().and_then(|v| Some(v.iter()))
    }
}

impl EdgeWeightedDirectedCycle {
    /// does G have a directed cycle?
    pub fn has_cycle(&self) -> bool {
        self.cycle.is_some()
    }

    /// vertices on a cycle (if one exists)
    pub fn cycle(&self) -> Option<Iter<'_, DirectedEdge>> {
        self.cycle.as_ref().and_then(|v| Some(v.iter()))
    }
}

impl DirectedCycle {
    fn new(nv: usize) -> Self {
        Self {
            cycle: None,
            edge_to: vec![0; nv],
            marked: vec![false; nv],
            on_stack: vec![false; nv],
        }
    }

    fn dfs(&mut self, graph: &dyn IGraph, v: usize) {
        self.on_stack[v] = true;
        self.marked[v] = true;
        for &w in graph.adj(v) {
            // short circuit if directed cycle found
            if self.cycle.is_some() {
                return;
            }

            // found new vertex, so recur
            if !self.marked[w] {
                self.edge_to[w] = v;
                self.dfs(graph, w);
            }
            // trace back directed cycle
            else if self.on_stack[w] {
                let mut cycle = Stack::new();
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
        self.on_stack[v] = false;
    }

    pub fn check(&self) -> Result<(), String> {
        match self.cycle() {
            Some(cycle) => {
                let mut first = -1;
                let mut last = -1;
                for &v in cycle {
                    if first == -1 {
                        first = v as i32;
                    }
                    last = v as i32;
                }
                if first != last {
                    return Err(format!(
                        "cycle begins with {} and ends with {}",
                        first, last
                    ));
                }
            }
            None => (),
        }

        Ok(())
    }
}

impl EdgeWeightedDirectedCycle {
    fn new(nv: usize) -> Self {
        Self {
            cycle: None,
            edge_to: vec![None; nv],
            marked: vec![false; nv],
            on_stack: vec![false; nv],
        }
    }

    fn dfs(&mut self, graph: &dyn IEWDigraph, v: usize) {
        self.on_stack[v] = true;
        self.marked[v] = true;
        for e in graph.adj(v) {
            // short circuit if directed cycle found
            if self.cycle.is_some() {
                return;
            }

            let w = e.to();
            // found new vertex, so recur
            if !self.marked[w] {
                self.edge_to[w] = Some(*e);
                self.dfs(graph, w);
            }
            // trace back directed cycle
            else if self.on_stack[w] {
                let mut cycle = Stack::new();
                let mut f = *e;
                while f.from() != w {
                    cycle.push(f);
                    f = self.edge_to[f.from()].unwrap();
                }
                cycle.push(f);
                self.cycle = Some(cycle);
            }
        }
        self.on_stack[v] = false;
    }

    pub fn check(&self) -> Result<(), String> {
        match self.cycle() {
            Some(cycle) => {
                let mut first: Option<&DirectedEdge> = None;
                let mut last: Option<&DirectedEdge> = None;
                for e in cycle {
                    if first.is_none() {
                        first = Some(e);
                    }
                    if last.is_some() {
                        if last.unwrap().to() != e.from() {
                            return Err(format!(
                                "cycle edges {} and {} not incident",
                                last.unwrap().to_string(),
                                e.to_string()
                            ));
                        }
                    }
                    last = Some(e);
                }
                if last.unwrap().to() != first.unwrap().from() {
                    return Err(format!(
                        "cycle edges {} and {} not incident",
                        last.unwrap().to_string(),
                        first.unwrap().to_string()
                    ));
                }
            }
            _ => (),
        }
        Ok(())
    }
}

macro_rules! impl_from {
    ($From: ty, $To: ty) => {
        impl From<$From> for $To {
            fn from(g: $From) -> Self {
                let nv = g.V();
                let mut cycle = Self::new(nv);

                for v in 0..nv {
                    if !cycle.marked[v] && cycle.cycle.is_none() {
                        cycle.dfs(g, v);
                    }
                }

                debug_assert!(cycle.check().is_ok());
                cycle
            }
        }
    };
}

impl_from!(&dyn IGraph, DirectedCycle);
impl_from!(&dyn IEWDigraph, EdgeWeightedDirectedCycle);
