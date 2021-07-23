use crate::graph::mst::Edge;
use crate::graph::IEWGraph;
use crate::ll::linked_list::Iter;
use crate::ll::LinkedList;
use std::cmp::Ordering;

/// The EWGraph class represents an edge-weighted
/// graph of vertices named 0 through V – 1, where each
/// undirected edge is of type Edge and has a real-valued weight.
/// It supports the following two primary operations: add an edge to the graph,
/// iterate over all of the edges incident to a vertex. It also provides
/// methods for returning the degree of a vertex, the number of vertices
/// V in the graph, and the number of edges E in the graph.
/// Parallel edges and self-loops are permitted.
/// By convention, a self-loop v-v appears in the
/// adjacency list of v twice and contributes two to the degree
/// of v.
pub struct EWGraph {
    nv: usize,
    ne: usize,
    adj: Vec<LinkedList<Edge>>,
}

impl IEWGraph for EWGraph {
    fn V(&self) -> usize {
        self.nv
    }

    fn E(&self) -> usize {
        self.ne
    }

    fn add_edge(&mut self, e: Edge) {
        let v = e.either();
        let w = e.other(v);
        self.adj[v].push_front(e);
        self.adj[w].push_front(e);
        self.ne += 1;
    }

    fn adj(&self, v: usize) -> Iter<'_, Edge> {
        self.adj[v].iter()
    }

    fn edges(&self) -> Vec<Edge> {
        let mut list = Vec::new();
        for v in 0..self.V() {
            let mut self_loops = 0;
            for e in self.adj(v) {
                match e.other(v).cmp(&v) {
                    Ordering::Greater => list.push(*e),
                    Ordering::Equal => {
                        // add only one copy of each self loop (self loops will be consecutive)
                        // add_edge(v, v, weighted), add_edge实现，加入两个edge
                        if self_loops % 2 == 0 {
                            list.push(*e);
                        }
                        self_loops += 1;
                    }
                    Ordering::Less => (),
                }
            }
        }
        list
    }

    fn degree(&self, v: usize) -> usize {
        self.adj[v].len()
    }
}

impl EWGraph {
    pub fn new(nv: usize) -> Self {
        let mut adj = Vec::with_capacity(nv);
        for _ in 0..nv {
            adj.push(LinkedList::default());
        }

        Self { nv, ne: 0, adj }
    }
}

weighted_graph_util!(EWGraph);
