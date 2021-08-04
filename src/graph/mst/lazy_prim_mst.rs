use crate::common::{Queue, PQ, UF};
use crate::graph::mst::{Edge, MST};
use crate::graph::IEWGraph;
use crate::ll::linked_list::Iter;

pub struct LazyPrimMST {
    weight: f32,
    mst: Queue<Edge>,
    marked: Vec<bool>,
    pq: PQ<Edge>,
}

impl LazyPrimMST {
    pub fn new(g: &dyn IEWGraph) -> Self {
        let mut mst = Self {
            weight: 0.0,
            mst: Queue::new(),
            marked: vec![false; g.V()],
            pq: PQ::new_min_pq(1),
        };

        for v in 0..g.V() {
            if !mst.marked[v] {
                mst.prim(g, v);
            }
        }

        mst
    }
}

impl MST for LazyPrimMST {
    fn edges(&self) -> Iter<'_, Edge> {
        self.mst.iter()
    }

    fn weight(&self) -> f32 {
        self.weight
    }
}

impl LazyPrimMST {
    fn prim(&mut self, g: &dyn IEWGraph, s: usize) {
        self.scan(g, s);
        while !self.pq.is_empty() {
            let e = self.pq.dequeue().unwrap();
            let v = e.either();
            let w = e.other(v);
            debug_assert!(self.marked[v] || self.marked[w]);
            if self.marked[v] && self.marked[w] {
                continue;
            }

            self.mst.enqueue(e);
            self.weight += e.weight();

            if !self.marked[v] {
                self.scan(g, v);
            }
            if !self.marked[w] {
                self.scan(g, w);
            }
        }
    }

    fn scan(&mut self, g: &dyn IEWGraph, v: usize) {
        self.marked[v] = true;
        for e in g.adj(v) {
            if !self.marked[e.other(v)] {
                self.pq.enqueue(*e);
            }
        }
    }
}

impl LazyPrimMST {
    pub fn check(&self, g: &dyn IEWGraph) -> Result<(), String> {
        // check that it is acyclic
        let mut uf = UF::new(g.V());
        for e in self.edges() {
            let v = e.either();
            let w = e.other(v);
            if uf.find(v) == uf.find(w) {
                return Err("Not a forest".to_string());
            }
            uf.union(v, w);
        }

        // check that it is a spanning forest
        for e in g.edges() {
            let v = e.either();
            let w = e.other(v);
            if uf.find(v) != uf.find(w) {
                return Err("Not a spanning forest".to_string());
            }
        }

        // check that it is a minimal spanning forest (cut optimality conditions)
        for e in self.edges() {
            // all edges in MST except e
            uf = UF::new(g.V());
            for f in self.mst.iter() {
                if f != e {
                    let x = f.either();
                    let y = f.other(x);
                    uf.union(x, y);
                }
            }

            // check that e is min weight edge in crossing cut
            for f in g.edges() {
                let x = f.either();
                let y = f.other(x);
                if uf.find(x) != uf.find(y) {
                    if f.weight() < e.weight() {
                        return Err(format!(
                            "Edge {} violates cut optimality conditions",
                            f.to_string()
                        ));
                    }
                }
            }
        }

        Ok(())
    }
}
