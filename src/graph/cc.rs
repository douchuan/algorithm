use crate::graph::Graph;

/// Connected components
pub struct CC {
    count: usize,
    marked: Vec<bool>,
    id: Vec<usize>,
}

impl CC {
    pub fn new(g: &Graph) -> Self {
        let mut cc = Self {
            count: 0,
            marked: vec![false; g.V()],
            id: vec![0; g.V()],
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
}

impl CC {
    fn dfs(&mut self, g: &Graph, v: usize) {
        self.marked[v] = true;
        self.id[v] = self.count;
        for &w in g.adj(v) {
            if !self.marked[w] {
                self.dfs(g, w);
            }
        }
    }
}
