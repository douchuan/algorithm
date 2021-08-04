use crate::common::Queue;
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
    fn path_to(&self, v: usize) -> Option<LinkedList<usize>>;
}

/// Run depth-first search on an undirected graph.
///
/// Determine reachability in a digraph from a given vertex using
/// depth-first search.
/// Runs in O(E + V) time.
pub struct DepthFirstPaths {
    marked: Vec<bool>,   // marked[v] = is there an s-v path?
    edge_to: Vec<usize>, // edgeTo[v] = last edge on s-v path
    s: usize,            // source vertex
}

/// Run breadth first search on an undirected graph.
/// Runs in O(E + V) time.
///
/// Run breadth-first search on a digraph.
/// Runs in O(E + V) time.
pub struct BreadthFirstPaths {
    marked: Vec<bool>,   // marked[v] = is there an s-v path
    edge_to: Vec<usize>, // edgeTo[v] = previous edge on shortest s-v path
    dist_to: Vec<usize>, // distTo[v] = number of edges shortest s-v path
    s: usize,            // source vertex
}

impl Paths for DepthFirstPaths {
    fn has_path(&self, v: usize) -> bool {
        self.marked[v]
    }

    fn path_to(&self, v: usize) -> Option<LinkedList<usize>> {
        if self.has_path(v) {
            let mut paths = LinkedList::default();
            let s = self.s;
            let mut x = v;

            while x != s {
                paths.push_front(x);
                x = self.edge_to[x];
            }
            paths.push_front(s);

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

    fn path_to(&self, v: usize) -> Option<LinkedList<usize>> {
        if self.has_path(v) {
            let mut paths = LinkedList::default();
            let s = self.s;
            let mut x = v;

            while x != s {
                paths.push_front(x);
                x = self.edge_to[x];
            }
            paths.push_front(s);

            Some(paths)
        } else {
            None
        }
    }
}

impl DepthFirstPaths {
    pub fn new(g: &dyn IGraph, s: usize) -> Self {
        let marked = vec![false; g.V()];
        let edge_to = vec![0; g.V()];
        let mut h = Self { marked, s, edge_to };
        h.dfs(g, s);
        h
    }

    fn dfs(&mut self, g: &dyn IGraph, v: usize) {
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
    pub fn new(g: &dyn IGraph, s: usize) -> Self {
        let marked = vec![false; g.V()];
        let edge_to = vec![0; g.V()];
        let dist_to = vec![usize::MAX; g.V()];
        let mut h = Self {
            marked,
            s,
            edge_to,
            dist_to,
        };
        h.bfs(g, s);
        h
    }

    pub fn dist_to(&self, v: usize) -> usize {
        self.dist_to[v]
    }

    fn bfs(&mut self, g: &dyn IGraph, s: usize) {
        let mut queue = Queue::new();
        self.marked[s] = true;
        self.dist_to[s] = 0;
        queue.enqueue(s);
        while let Some(v) = queue.dequeue() {
            for &w in g.adj(v) {
                if !self.marked[w] {
                    self.edge_to[w] = v;
                    self.dist_to[w] = self.dist_to[v] + 1;
                    self.marked[w] = true;
                    queue.enqueue(w);
                }
            }
        }
    }
}
