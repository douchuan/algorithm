//! Strong connectivity in digraphs
//!
//! Definition.
//! Two vertices v and w are strongly connected if they are mutually
//! reachable: that is, if there is a directed path from v to w and
//! a directed path from w to v. A digraph is strongly connected if
//! all its vertices are strongly connected to one another.
//!
use crate::graph::directed::DepthFirstOrders;
use crate::graph::IGraph;

pub struct KosarajuSCC {
    marked: Vec<bool>,
    id: Vec<usize>,
    count: usize,
}

impl KosarajuSCC {
    pub fn new(graph: &Box<dyn IGraph>) -> Self {
        let mut scc = Self {
            marked: vec![false; graph.V()],
            id: vec![0; graph.V()],
            count: 0,
        };
        let order = DepthFirstOrders::new(&graph.reverse());
        for &s in order.rev_post() {
            if !scc.marked[s] {
                scc.dfs(graph, s);
                scc.count += 1;
            }
        }

        scc
    }

    pub fn strongly_connected(&self, v: usize, w: usize) -> bool {
        self.id[v] == self.id[w]
    }

    pub fn id(&self, v: usize) -> usize {
        self.id[v]
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

impl KosarajuSCC {
    fn dfs(&mut self, graph: &Box<dyn IGraph>, v: usize) {
        self.marked[v] = true;
        self.id[v] = self.count;
        for &w in graph.adj(v) {
            if !self.marked[w] {
                self.dfs(graph, w);
            }
        }
    }
}
