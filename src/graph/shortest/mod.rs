mod acyclic_sp;
mod arbitrage;
mod bellman_ford_sp;
mod cpm;
mod dijkstra_sp;
mod directed_edge;
mod ew_digraph;

pub use acyclic_sp::{AcyclicLP, AcyclicSP};
pub use arbitrage::Arbitrage;
pub use bellman_ford_sp::BellmanFordSP;
pub use cpm::CPM;
pub use dijkstra_sp::{DijkstraAllPairsSP, DijkstraSP};
pub use directed_edge::DirectedEdge;
pub use ew_digraph::EWDigraph;

use crate::ll::linked_list::Iter;
/// Edge weighted graph
pub trait IEWDigraph {
    /// number of vertices
    #[allow(non_snake_case)]
    fn V(&self) -> usize;

    /// number of edges
    #[allow(non_snake_case)]
    fn E(&self) -> usize;

    /// Adds the directed edge e to this edge-weighted graph
    fn add_edge(&mut self, v: usize, w: usize, weight: f32);

    /// Returns the edges incident on vertex v
    fn adj(&self, v: usize) -> Iter<'_, DirectedEdge>;

    /// Returns all edges in this edge-weighted graph
    fn edges(&self) -> Vec<DirectedEdge>;

    /// Returns the degree of vertex v
    fn out_degree(&self, v: usize) -> usize;

    /// Returns the number of directed edges incident to vertex
    fn in_degree(&self, v: usize) -> usize;
}
