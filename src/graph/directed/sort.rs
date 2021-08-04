//! A digraph has a topological order if and only if it is a DAG.

use crate::graph::directed::{DepthFirstOrders, DirectedCycle, EdgeWeightedDigraphCycle};
use crate::graph::{IEWDigraph, IGraph};
use std::slice::Iter;

pub struct Topological {
    order: Option<Vec<usize>>,
}

impl Topological {
    pub fn is_dag(&self) -> bool {
        self.order.is_some()
    }

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
impl_from!(&dyn IEWDigraph, EdgeWeightedDigraphCycle);
