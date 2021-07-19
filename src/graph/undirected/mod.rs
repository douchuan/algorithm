//! Since we consider a large number of graph processing algorithms,
//! our initial design goal is to decouple our implementations from
//! the graph representation. To do so, we develop, for each given
//! task, a task-specific class so that clients can create objects
//! to perform the task.
//!
//! |  Problem                         |    Solution
//! |----------------------------------|------------------------------------------
//! | single source connectivity       |    DepthFirstSearch (search.rs)         |
//! | single source paths              |    DepthFirstPaths (util/paths.rs)      |
//! | single source shortest paths     |    BreadthFirstPaths (util/paths.rs)    |
//! | connectivity                     |    CC (cc.rs)                           |
//! | cycle detection                  |    Cycle (detection.rs)                 |
//! | two-color (bipartite)            |    TwoColor (detection.rs)              |
//!
pub use cc::CC;
pub use detection::{Cycle, TowColor};
pub use graph::Graph;
pub use search::{DepthFirstSearch, Search};
pub use symbol_graph::SymbolGraph;

mod cc;
mod detection;
mod graph;
mod search;
mod symbol_graph;
