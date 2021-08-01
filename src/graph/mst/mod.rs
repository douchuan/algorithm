//!
//! MST Definition
//! Recall that a spanning tree of a graph is a connected subgraph
//! with no cycles that includes all the vertices.
//! A minimum spanning tree (MST) of an edge-weighted graph is a
//! spanning tree whose weight (the sum of the weights of its edges)
//! is no larger than the weight of any other spanning tree.
//!
//! Assumptions
//!   The graph is connected
//!   The edge weights are not necessarily distances
//!   The edge weights may be zero or negative
//!   The edge weights are all different.
//!
//! Definition
//! A cut of a graph is a partition of its vertices into two nonempty
//! disjoint sets. A crossing edge of a cut is an edge that connects
//! a vertex in one set with a vertex in the other.
//!
//! Proposition J.
//! (Cut property) Given any cut in an edge weighted graph, the crossing
//! edge of minimum weight is in the MST of the graph.
//!
//! Proposition K.
//! (Greedy MST algorithm) The following method colors black all edges in
//! the the MST of any connected edge weighted graph with V vertices:
//! starting with all edges colored gray, find a cut with no black edges,
//! color its minimum-weight edge black, and continue until V - 1 edges
//! have been colored black.

mod edge;
mod ewgraph;
mod kruskal_mst;
mod lazy_prim_mst;
mod prim_mst;

use crate::ll::linked_list::Iter;
pub use edge::Edge;
pub use ewgraph::EWGraph;
pub use kruskal_mst::KruskalMST;
pub use lazy_prim_mst::LazyPrimMST;
pub use prim_mst::PrimMST;

pub trait MST {
    /// Returns the edges in a minimum spanning tree (or forest)
    fn edges(&self) -> Iter<'_, Edge>;
    /// Returns the sum of the edge weights in a minimum spanning tree (or forest)
    fn weight(&self) -> f32;
}
