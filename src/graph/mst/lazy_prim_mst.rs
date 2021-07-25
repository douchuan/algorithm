use crate::common::{MinPQ, Queue};
use crate::graph::mst::Edge;
use crate::graph::IEWGraph;
use crate::ll::linked_list::Iter;

pub struct LazyPrimMST {
    weight: f32,
    mst: Queue<Edge>,
    marked: Vec<bool>,
    pq: MinPQ<Edge>,
}

impl LazyPrimMST {
    pub fn new(g: &Box<dyn IEWGraph>) -> Self {
        let mut mst = Self {
            weight: 0.0,
            mst: Queue::new(),
            marked: vec![false; g.V()],
            pq: MinPQ::new(),
        };
        for v in 0..g.V() {
            if !mst.marked[v] {
                mst.prim(g, v);
            }
        }
        mst
    }

    pub fn edges(&self) -> Iter<'_, Edge> {
        self.mst.iter()
    }

    pub fn weight(&self) -> f32 {
        self.weight
    }
}

impl LazyPrimMST {
    fn prim(&mut self, g: &Box<dyn IEWGraph>, s: usize) {
        self.scan(g, s);
        while !self.pq.is_empty() {
            let e = self.pq.del_min().unwrap();
            let v = e.either();
            let w = e.other(v);
            debug_assert!(self.marked[v] || self.marked[w]);
            if self.marked[v] && self.marked[w] {
                continue;
            }
            self.mst.enqueue(e);
            self.weight += e.weight();
            if self.marked[v] {
                self.scan(g, v);
            }
            if self.marked[w] {
                self.scan(g, w);
            }
        }
    }

    fn scan(&mut self, g: &Box<dyn IEWGraph>, v: usize) {
        self.marked[v] = true;
        for e in g.adj(v) {
            if !self.marked[e.other(v)] {
                self.pq.insert(*e);
            }
        }
    }
}
