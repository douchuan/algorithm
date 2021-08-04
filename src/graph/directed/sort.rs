//! A digraph has a topological order if and only if it is a DAG.

use crate::graph::directed::{DepthFirstOrders, DirectedCycle};
use crate::graph::IGraph;
use std::slice::Iter;

pub struct Topological {
    order: Option<Vec<usize>>,
}

impl Topological {
    pub fn new(graph: &dyn IGraph) -> Self {
        let cycle = DirectedCycle::from(graph);
        let order = if cycle.has_cycle() {
            None
        } else {
            let dfs = DepthFirstOrders::from(graph);
            Some(dfs.rev_post().cloned().collect())
        };

        Self { order }
    }

    pub fn is_dag(&self) -> bool {
        self.order.is_some()
    }

    pub fn order(&self) -> Option<Iter<'_, usize>> {
        self.order.as_ref().and_then(|v| Some(v.iter()))
    }
}
