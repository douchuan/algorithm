use crate::graph::IGraph;

/// find vertices connected to a source vertex s
///
/// We use the term 'source' (起点) to distinguish the
/// vertex provided as argument to the constructor
/// from the other vertices in the graph.
pub trait Search {
    /// is v connected to s?
    fn marked(&self, v: usize) -> bool;

    /// how many vertices are connected to s?
    ///
    /// if (search.count() != G.V())
    ///   Not connected
    fn count(&self) -> usize;
}

/// depth-first search (DFS)
///
/// To search a graph, invoke a recursive method that visits vertices.
/// To visit a vertex:
///  - Mark it as having been visited.
///  - Visit (recursively) all the vertices that are adjacent to it and
///    that have not yet been marked.
pub struct DepthFirstSearch {
    marked: Vec<bool>,
    count: usize,
}

// impl trait

impl Search for DepthFirstSearch {
    fn marked(&self, v: usize) -> bool {
        self.marked[v]
    }

    fn count(&self) -> usize {
        self.count
    }
}

// impl utils

impl DepthFirstSearch {
    pub fn new(g: &Box<dyn IGraph>, s: usize) -> Self {
        let marked = vec![false; g.V()];
        let mut h = Self { marked, count: 0 };
        h.dfs(g, s);
        h
    }

    fn dfs(&mut self, g: &Box<dyn IGraph>, v: usize) {
        self.marked[v] = true;
        self.count += 1;
        for &w in g.adj(v) {
            if !self.marked[w] {
                self.dfs(g, w);
            }
        }
    }
}
