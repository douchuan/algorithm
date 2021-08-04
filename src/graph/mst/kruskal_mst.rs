use crate::common::{Queue, UF};
use crate::graph::mst::{Edge, MST};
use crate::graph::IEWGraph;
use crate::ll::linked_list::Iter;

pub struct KruskalMST {
    weight: f32,      // weight of MST
    mst: Queue<Edge>, // edges in MST
}

impl KruskalMST {
    pub fn new(g: &dyn IEWGraph) -> Self {
        let mut edges = g.edges();
        edges.sort(); // sorted by weight

        let mut mst = Queue::new();
        let mut weight = 0.0;
        let mut uf = UF::new(g.V());
        let mut i = 0;
        while i < g.E() && mst.len() < g.V() - 1 {
            let e = edges[i];
            let v = e.either();
            let w = e.other(v);

            // v-w does not create a cycle
            if uf.find(v) != uf.find(w) {
                uf.union(v, w);
                mst.enqueue(e);
                weight += e.weight();
            }

            i += 1;
        }

        Self { mst, weight }
    }

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
            for f in self.edges() {
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

impl MST for KruskalMST {
    fn edges(&self) -> Iter<'_, Edge> {
        self.mst.iter()
    }

    fn weight(&self) -> f32 {
        self.weight
    }
}
