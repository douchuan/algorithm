//! The CC represents a data type for
//! determining the connected components in an undirected graph.
//! The *id* operation determines in which connected component
//! a given vertex lies; the *connected* operation
//! determines whether two vertices are in the same connected component;
//! the *count* operation determines the number of connected
//! components; and the *size* operation determines the number
//! of vertices in the connect component containing a given vertex.
//!
//! The component identifier of a connected component is one of the
//! vertices in the connected component: two vertices have the same component
//! identifier if and only if they are in the same connected component.
//!
//! This implementation uses a recursive DFS. To avoid needing
//! a potentially very large stack size, replace with a non recursive
//! DFS ala NonRecursiveDFS

use crate::graph::IGraph;

/// Connected components
pub struct CC {
    count: usize,      // number of connected components
    marked: Vec<bool>, // marked[v] = has vertex v been marked?
    id: Vec<usize>,    // id[v] = id of connected component containing v
    size: Vec<usize>,  // size[id] = number of vertices in given component
}

impl CC {
    pub fn new(g: &dyn IGraph) -> Self {
        let mut cc = Self {
            count: 0,
            marked: vec![false; g.V()],
            id: vec![0; g.V()],
            size: vec![0; g.V()],
        };

        for v in 0..g.V() {
            if !cc.marked[v] {
                cc.dfs(g, v);
                cc.count += 1;
            }
        }

        cc
    }

    /// are v and w connected?
    pub fn connected(&self, v: usize, w: usize) -> bool {
        self.id[v] == self.id[w]
    }

    /// number of connected components
    pub fn count(&self) -> usize {
        self.count
    }

    /// component identifier for v
    /// ( between 0 and count()-1 )
    pub fn id(&self, v: usize) -> usize {
        self.id[v]
    }

    /// Returns the number of vertices in the connected component containing vertex *v*.
    pub fn size(&self, v: usize) -> usize {
        self.size[self.id[v]]
    }
}

impl CC {
    fn dfs(&mut self, g: &dyn IGraph, v: usize) {
        self.marked[v] = true;
        self.id[v] = self.count;
        self.size[self.count] += 1;
        for &w in g.adj(v) {
            if !self.marked[w] {
                self.dfs(g, w);
            }
        }
    }
}
