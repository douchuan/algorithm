use crate::graph::shortest::DirectedEdge;
use crate::graph::util::parser::GraphDataParser;
use crate::graph::IEWDigraph;
use crate::ll::linked_list::Iter;
use crate::ll::LinkedList;

pub struct EWDigraph {
    nv: usize,                          // number of vertices in this digraph
    ne: usize,                          // number of edges in this digraph
    adj: Vec<LinkedList<DirectedEdge>>, // adj[v] = adjacency list for vertex v
    in_degree: Vec<usize>,              // in_degree[v] = in degree of vertex v
}

impl IEWDigraph for EWDigraph {
    fn V(&self) -> usize {
        self.nv
    }

    fn E(&self) -> usize {
        self.ne
    }

    fn add_edge(&mut self, v: usize, w: usize, weight: f32) {
        let e = DirectedEdge::new(v, w, weight);
        self.adj[v].push_front(e);
        self.in_degree[w] += 1;
        self.ne += 1;
    }

    fn adj(&self, v: usize) -> Iter<'_, DirectedEdge> {
        self.adj[v].iter()
    }

    fn edges(&self) -> Vec<DirectedEdge> {
        let mut list = Vec::new();
        for v in 0..self.V() {
            for e in self.adj(v) {
                list.push(*e)
            }
        }
        list
    }

    fn out_degree(&self, v: usize) -> usize {
        self.adj[v].len()
    }

    fn in_degree(&self, v: usize) -> usize {
        self.in_degree[v]
    }
}

impl From<usize> for EWDigraph {
    fn from(nv: usize) -> Self {
        let mut adj = Vec::with_capacity(nv);
        for _ in 0..nv {
            adj.push(LinkedList::default());
        }

        Self {
            nv,
            ne: 0,
            adj,
            in_degree: vec![0; nv],
        }
    }
}

impl From<&str> for EWDigraph {
    fn from(s: &str) -> Self {
        let parser = GraphDataParser::parse(s, true).unwrap();
        let mut g = Self::from(parser.get_v());
        for (v, w, weight) in parser.get_weighted_edges() {
            g.add_edge(*v, *w, *weight);
        }
        debug_assert!(g.E() == parser.get_e());
        g
    }
}
