use crate::graph::IGraph;

pub struct DirectedDFS {
    marked: Vec<bool>,
}

impl DirectedDFS {
    /// find vertices in G that are reachable from s
    pub fn new_single(graph: &Box<dyn IGraph>, s: usize) -> Self {
        let mut dfs = Self {
            marked: vec![false; graph.V()],
        };
        dfs.dfs(graph, s);
        dfs
    }

    /// find vertices in G that are reachable from sources
    pub fn new_multi(graph: &Box<dyn IGraph>, sources: &[usize]) -> Self {
        let mut dfs = Self {
            marked: vec![false; graph.V()],
        };
        for &s in sources {
            if !dfs.marked[s] {
                dfs.dfs(graph, s);
            }
        }
        dfs
    }

    /// is v reachable?
    pub fn marked(&self, v: usize) -> bool {
        self.marked[v]
    }
}

impl DirectedDFS {
    fn dfs(&mut self, graph: &Box<dyn IGraph>, v: usize) {
        self.marked[v] = true;
        for &w in graph.adj(v) {
            if !self.marked[w] {
                self.dfs(graph, w);
            }
        }
    }
}
