use crate::graph::Graph;

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

/// Finding paths
/// Given a graph and a source vertex s, support queries
/// of the form Is there a path from s to a given target
/// vertex v? If so, find such a path.
pub trait Paths {
    /// is there a path from s to v ?
    fn has_path(&self, v: usize) -> bool;
    /// path from s to v; None if no such path
    fn path_to(&self, v: usize) -> Option<Vec<usize>>;
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

impl Search for DepthFirstSearch {
    fn marked(&self, v: usize) -> bool {
        self.marked[v]
    }

    fn count(&self) -> usize {
        self.count
    }
}

impl DepthFirstSearch {
    pub fn new(g: &Graph, s: usize) -> Self {
        let marked = vec![false; g.V()];
        let mut h = Self { marked, count: 0 };
        h.dfs(g, s);
        h
    }

    fn dfs(&mut self, g: &Graph, v: usize) {
        self.marked[v] = true;
        self.count += 1;

        for &w in g.adj(v) {
            if !self.marked[w] {
                self.dfs(g, w);
            }
        }
    }
}
