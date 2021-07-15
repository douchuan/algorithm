use crate::graph::IGraph;
use crate::ll::linked_list::Iter;
use crate::ll::LinkedList;

pub struct Digraph {
    nv: usize, // num of vertices
    ne: usize, // num of edges
    adj: Vec<LinkedList<usize>>,
}

impl IGraph for Digraph {
    fn V(&self) -> usize {
        self.nv
    }

    fn E(&self) -> usize {
        self.ne
    }

    fn add_edge(&mut self, v: usize, w: usize) {
        // 因为是建立 "v -> w" 有方向的边，
        // 此处只调用一次push_front, 不同于Graph
        self.adj[v].push_front(w);
        self.ne += 1;
    }

    fn adj(&self, v: usize) -> Iter<'_, usize> {
        self.adj[v].iter()
    }
}

impl Digraph {
    /// create a V-vertex graph with no edges
    pub fn new(nv: usize) -> Self {
        let mut adj = Vec::with_capacity(nv);
        for _ in 0..nv {
            adj.push(LinkedList::default());
        }

        Self { nv, ne: 0, adj }
    }

    /// reverse of this digraph
    pub fn reverse(&self) -> Self {
        let mut r = Digraph::new(self.nv);
        for v in 0..self.nv {
            for &w in self.adj(v) {
                r.add_edge(w, v);
            }
        }

        r
    }
}

graph_util!(Digraph);
