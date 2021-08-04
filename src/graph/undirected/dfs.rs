//! find vertices connected to a source vertex s
//!
//! We use the term 'source' (起点) to distinguish the
//! vertex provided as argument to the constructor
//! from the other vertices in the graph.
//!
//! depth-first search (DFS)
//!
//! To search a graph, invoke a *recursive* method that visits vertices.
//! To visit a vertex:
//!  - Mark it as having been visited.
//!  - Visit (recursively) all the vertices that are adjacent to it and
//!    that have not yet been marked.

use crate::graph::IGraph;

pub struct DepthFirstSearch {
    marked: Vec<bool>,
    count: usize,
}

impl DepthFirstSearch {
    pub fn new(g: &dyn IGraph, s: usize) -> Self {
        let mut dfs = Self {
            marked: vec![false; g.V()],
            count: 0,
        };
        dfs.dfs(g, s);
        dfs
    }

    /// is v connected to s?
    pub fn marked(&self, v: usize) -> bool {
        self.marked[v]
    }

    /// how many vertices are connected to s?
    ///
    /// if (search.count() != G.V())
    ///   Not connected
    pub fn count(&self) -> usize {
        self.count
    }
}

impl DepthFirstSearch {
    fn dfs(&mut self, g: &dyn IGraph, v: usize) {
        self.marked[v] = true;
        self.count += 1;
        for &w in g.adj(v) {
            if !self.marked[w] {
                self.dfs(g, w);
            }
        }
    }
}
