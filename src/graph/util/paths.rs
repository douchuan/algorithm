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
    /// true when v < g.V()
    fn valid_vertex(&self, v: usize) -> bool;
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
        self.valid_vertex(v) && self.marked[v]
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

    fn valid_vertex(&self, v: usize) -> bool {
        #[allow(non_snake_case)]
        let V = self.marked.len();
        v < V
    }
}

impl Paths for BreadthFirstPaths {
    fn has_path(&self, v: usize) -> bool {
        self.valid_vertex(v) && self.marked[v]
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

    fn valid_vertex(&self, v: usize) -> bool {
        #[allow(non_snake_case)]
        let V = self.marked.len();
        v < V
    }
}

impl DepthFirstPaths {
    pub fn new(g: &Box<dyn IGraph>, s: usize) -> Self {
        let marked = vec![false; g.V()];
        let edge_to = vec![0; g.V()];
        let mut h = Self { marked, s, edge_to };
        h.dfs(g, s);
        h
    }

    fn dfs(&mut self, g: &Box<dyn IGraph>, v: usize) {
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
    pub fn new(g: &Box<dyn IGraph>, s: usize) -> Self {
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
        if !self.valid_vertex(v) {
            usize::MAX
        } else {
            self.dist_to[v]
        }
    }

    fn bfs(&mut self, g: &Box<dyn IGraph>, s: usize) {
        let mut queue = LinkedList::default();
        self.marked[s] = true;
        self.dist_to[s] = 0;
        queue.push_back(s);
        while let Some(v) = queue.pop_front() {
            for &w in g.adj(v) {
                if !self.marked[w] {
                    self.edge_to[w] = v;
                    self.dist_to[w] = self.dist_to[v] + 1;
                    self.marked[w] = true;
                    queue.push_back(w);
                }
            }
        }
    }
}
