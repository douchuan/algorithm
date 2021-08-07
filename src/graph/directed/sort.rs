//! A digraph has a topological order if and only if it is a DAG.

use crate::graph::directed::{DepthFirstOrders, DirectedCycle, EdgeWeightedDirectedCycle};
use crate::graph::{IEWDigraph, IGraph};
use std::slice::Iter;

/// The Topological represents a data type for
/// determining a topological order of a directed acyclic graph (DAG).
/// A digraph has a topological order if and only if it is a DAG.
/// The has_order operation determines whether the digraph has
/// a topological order, and if so, the order operation
/// returns one.
/// This implementation uses depth-first search.
pub struct Topological {
    order: Option<Vec<usize>>,
}

impl Topological {
    /// Does the digraph have a topological order?
    /// true if the digraph has a topological order (or equivalently,
    /// if the digraph is a DAG), and false otherwise
    pub fn has_order(&self) -> bool {
        self.order.is_some()
    }

    /// Returns a topological order if the digraph has a topologial order,
    /// and None otherwise.
    pub fn order(&self) -> Option<Iter<'_, usize>> {
        self.order.as_ref().and_then(|v| Some(v.iter()))
    }
}

macro_rules! impl_from {
    ($From: ty, $Cycle: ident) => {
        impl From<$From> for Topological {
            fn from(g: $From) -> Self {
                let cycle = $Cycle::from(g);
                let order = if cycle.has_cycle() {
                    None
                } else {
                    let dfs = DepthFirstOrders::from(g);
                    Some(dfs.rev_post().cloned().collect())
                };

                Self { order }
            }
        }
    };
}

impl_from!(&dyn IGraph, DirectedCycle);
impl_from!(&dyn IEWDigraph, EdgeWeightedDirectedCycle);
