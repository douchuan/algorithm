use crate::graph::Graph;

pub struct Cycle {
    marked: Vec<bool>,
    has_cycle: bool,
}

impl Cycle {
    pub fn new(g: &Graph) -> Self {
        let mut cc = Self {
            marked: vec![false; g.V()],
            has_cycle: false,
        };

        for s in 0..g.V() {
            if !cc.marked[s] {
                cc.dfs(g, s, s);
            }
        }

        cc
    }

    pub fn has_cycle(&self) -> bool {
        self.has_cycle
    }
}

impl Cycle {
    fn dfs(&mut self, g: &Graph, v: usize, u: usize) {
        self.marked[v] = true;
        for &w in g.adj(v) {
            if !self.marked[w] {
                self.dfs(g, w, v);
            } else if w != u {
                self.has_cycle = true;
            }
        }
    }
}
