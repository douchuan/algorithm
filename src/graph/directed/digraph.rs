use crate::graph::util::parser::GraphDataParser;
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

    /// reverse of this digraph
    fn reverse(&self) -> Box<dyn IGraph> {
        let mut r = Digraph::from(self.nv);
        for v in 0..self.nv {
            for &w in self.adj(v) {
                r.add_edge(w, v);
            }
        }

        Box::new(r)
    }
}

/// create a V-vertex graph with no edges
impl From<usize> for Digraph {
    fn from(nv: usize) -> Self {
        let mut adj = Vec::with_capacity(nv);
        for _ in 0..nv {
            adj.push(LinkedList::default());
        }

        Self { nv, ne: 0, adj }
    }
}

impl From<&str> for Digraph {
    fn from(s: &str) -> Self {
        let parser = GraphDataParser::parse(s, false).unwrap();
        let mut g = Self::from(parser.get_v());
        for (v, w) in parser.get_edges() {
            g.add_edge(*v, *w);
        }
        debug_assert!(g.E() == parser.get_e());
        g
    }
}
