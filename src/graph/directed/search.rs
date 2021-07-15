use crate::graph::directed::Digraph;
use crate::graph::IGraph;

pub struct DirectedDFS {
    marked: Vec<bool>,
}

impl DirectedDFS {
    /// find vertices in G that are reachable from s
    pub fn new_single(graph: &Digraph, s: usize) -> Self {
        let marked = vec![false; graph.V()];
        let mut search = Self { marked };
        search.dfs(graph, s);
        search
    }

    /// find vertices in G that are reachable from sources
    pub fn new_multi(graph: &Digraph, sources: &[usize]) -> Self {
        let marked = vec![false; graph.V()];
        let mut search = Self { marked };
        for &s in sources {
            if !search.marked[s] {
                search.dfs(graph, s);
            }
        }
        search
    }

    /// is v reachable?
    pub fn marked(&self, v: usize) -> bool {
        self.marked[v]
    }
}

impl DirectedDFS {
    fn dfs(&mut self, graph: &Digraph, v: usize) {
        self.marked[v] = true;
        for &w in graph.adj(v) {
            if !self.marked[w] {
                self.dfs(graph, w);
            }
        }
    }
}
