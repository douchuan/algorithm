use crate::graph::undirected::Graph;
use crate::graph::IGraph;
use crate::ll::linked_list::LinkedList;

/// Finding paths
/// Given a graph and a source vertex s, support queries
/// of the form: Is there a path from s to a given target
/// vertex v? If so, find such a path.
pub trait Paths {
    /// is there a path from s to v ?
    fn has_path(&self, v: usize) -> bool;
    /// path from s to v; None if no such path
    fn path_to(&self, v: usize) -> Option<Vec<usize>>;
}

pub struct DepthFirstPaths {
    marked: Vec<bool>,
    edge_to: Vec<usize>,
    s: usize,
}

pub struct BreadthFirstPaths {
    marked: Vec<bool>,
    edge_to: Vec<usize>,
    s: usize,
}

impl Paths for DepthFirstPaths {
    fn has_path(&self, v: usize) -> bool {
        self.marked[v]
    }

    fn path_to(&self, v: usize) -> Option<Vec<usize>> {
        if self.has_path(v) {
            let mut paths = Vec::new();
            let s = self.s;
            let mut x = v;

            while x != s {
                paths.push(x);
                x = self.edge_to[x];
            }
            paths.push(s);
            paths.reverse();

            Some(paths)
        } else {
            None
        }
    }
}

impl Paths for BreadthFirstPaths {
    fn has_path(&self, v: usize) -> bool {
        self.marked[v]
    }

    fn path_to(&self, v: usize) -> Option<Vec<usize>> {
        if self.has_path(v) {
            let mut paths = Vec::new();
            let s = self.s;
            let mut x = v;

            while x != s {
                paths.push(x);
                x = self.edge_to[x];
            }
            paths.push(s);
            paths.reverse();

            Some(paths)
        } else {
            None
        }
    }
}

impl DepthFirstPaths {
    pub fn new(g: &Graph, s: usize) -> Self {
        let marked = vec![false; g.V()];
        let edge_to = vec![0; g.V()];
        let mut h = Self { marked, s, edge_to };
        h.dfs(g, s);
        h
    }

    fn dfs(&mut self, g: &Graph, v: usize) {
        self.marked[v] = true;
        for &w in g.adj(v) {
            if !self.marked[w] {
                self.edge_to[w] = v;
                self.dfs(g, w);
            }
        }
    }
}

impl BreadthFirstPaths {
    pub fn new(g: &Graph, s: usize) -> Self {
        let marked = vec![false; g.V()];
        let edge_to = vec![0; g.V()];
        let mut h = Self { marked, s, edge_to };
        h.bfs(g, s);
        h
    }

    fn bfs(&mut self, g: &Graph, v: usize) {
        let mut queue = LinkedList::default();
        self.marked[v] = true;
        queue.push_back(v);
        while let Some(v) = queue.pop_front() {
            for &w in g.adj(v) {
                if !self.marked[w] {
                    self.marked[w] = true;
                    queue.push_back(w);
                    self.edge_to[w] = v;
                }
            }
        }
    }
}
