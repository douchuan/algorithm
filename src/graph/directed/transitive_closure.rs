//! Definition.
//! The transitive closure of a digraph G is another digraph with
//! the same set of vertices, but with an edge from v to w in the
//! transitive closure if and only if w is reachable from v in G.
//!
//! By convention, every vertex is reachable from itself
//!
//! Compute transitive closure of a digraph and support
//! reachability queries.
//!
//! Preprocessing time: O(V(E + V)) time.
//! Query time: O(1).
//! Space: O(V^2).

use crate::graph::directed::{Digraph, DirectedDFS};
use crate::graph::IGraph;

pub struct TransitiveClosure {
    tc: Vec<DirectedDFS>, // tc[v] = reachable from v
}

impl TransitiveClosure {
    /// Computes the transitive closure of the digraph
    pub fn new(graph: &Digraph) -> Self {
        let mut tc = Vec::with_capacity(graph.V());
        for v in 0..graph.V() {
            tc.push(DirectedDFS::new_single(graph, v));
        }
        Self { tc }
    }

    /// Is there a directed path from vertex v to vertex w in the digraph?
    pub fn reachable(&self, v: usize, w: usize) -> bool {
        #[allow(non_snake_case)]
        let V = self.tc.len();
        if v < V && w < V {
            self.tc[v].marked(w)
        } else {
            false
        }
    }
}
