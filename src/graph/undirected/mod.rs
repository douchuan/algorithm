mod cc;
mod detection;
mod graph;
mod paths;
mod search;
mod symbol_graph;

pub use graph::Graph;

// Since we consider a large number of graph processing algorithms,
// our initial design goal is to decouple our implementations from
// the graph representation. To do so, we develop, for each given
// task, a task-specific class so that clients can create objects
// to perform the task.
//
//   Problem                        |    Solution
//                                  |
// single source connectivity       |    DepthFirstSearch
// single source paths              |    DepthFirstPaths
// single source shortest paths     |    BreadthFirstPaths
// connectivity                     |    CC
// cycle detection                  |    Cycle
// two-color (bipartite)            |    TowColor
//                                  |
pub use cc::CC;
pub use detection::{Cycle, TowColor};
pub use paths::{BreadthFirstPaths, DepthFirstPaths, Paths};
pub use search::{DepthFirstSearch, Search};
pub use symbol_graph::SymbolGraph;
