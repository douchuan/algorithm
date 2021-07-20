use crate::graph::IGraph;
use std::collections::{linked_list, LinkedList};
use std::slice::Iter;

pub struct DepthFirstOrders {
    marked: Vec<bool>,

    pre: Vec<usize>,
    post: Vec<usize>,
    rev_post: LinkedList<usize>,
}

impl DepthFirstOrders {
    pub fn new(graph: &Box<dyn IGraph>) -> Self {
        let mut order = Self {
            marked: vec![false; graph.V()],
            pre: Vec::with_capacity(graph.V()),
            post: Vec::with_capacity(graph.V()),
            rev_post: LinkedList::new(),
        };

        for s in 0..graph.V() {
            if !order.marked[s] {
                order.dfs(graph, s);
            }
        }

        order
    }

    pub fn pre(&self) -> Iter<'_, usize> {
        self.pre.iter()
    }

    pub fn post(&self) -> Iter<'_, usize> {
        self.post.iter()
    }

    pub fn rev_post(&self) -> linked_list::Iter<'_, usize> {
        self.rev_post.iter()
    }
}

impl DepthFirstOrders {
    fn dfs(&mut self, graph: &Box<dyn IGraph>, v: usize) {
        self.pre.push(v);

        self.marked[v] = true;
        for &w in graph.adj(v) {
            if !self.marked[w] {
                self.dfs(graph, w);
            }
        }

        self.post.push(v);
        self.rev_post.push_front(v);
    }
}
