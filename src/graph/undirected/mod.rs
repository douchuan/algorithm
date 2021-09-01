//! Since we consider a large number of graph processing algorithms,
//! our initial design goal is to decouple our implementations from
//! the graph representation. To do so, we develop, for each given
//! task, a task-specific class so that clients can create objects
//! to perform the task.
//!
//! |  Problem                         |    Solution
//! |----------------------------------|------------------------------------------
//! | single source connectivity       |    DepthFirstSearch (dfs)         |
//! | single source paths              |    DepthFirstPaths (util/paths.rs)      |
//! | single source shortest paths     |    BreadthFirstPaths (util/paths.rs)    |
//! | connectivity                     |    CC (cc.rs)                           |
//! | cycle detection                  |    Cycle (bipartite)                 |
//! | two-color (bipartite)            |    TwoColor (bipartite)              |
//!
pub use bipartite::Bipartite;
pub use cc::CC;
pub use cycle::Cycle;
pub use dfs::DepthFirstSearch;
pub use dfs2::NonRecursiveDFS;
pub use graph::Graph;

mod bipartite;
mod cc;
mod cycle;
mod dfs;
mod dfs2;
mod graph;

use crate::ll::linked_list::Iter;
pub trait IGraph {
    /// number of vertices
    #[allow(non_snake_case)]
    fn V(&self) -> usize;

    /// number of edges
    #[allow(non_snake_case)]
    fn E(&self) -> usize;

    /// add edge v-w to this graph
    fn add_edge(&mut self, v: usize, w: usize);

    /// vertices adjacent to v
    fn adj(&self, v: usize) -> Iter<'_, usize>;

    /// directed graph op
    fn reverse(&self) -> Box<dyn IGraph> {
        panic!("No Support");
    }
}
