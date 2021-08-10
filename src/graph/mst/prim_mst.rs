use crate::common::{IndexPQ, UF};
use crate::graph::mst::Edge;
use crate::graph::IEWGraph;

/// The PrimMST represents a data type for computing a
/// minimum spanning tree in an edge-weighted graph.
/// The edge weights can be positive, zero, or negative and need not
/// be distinct. If the graph is not connected, it computes a minimum
/// spanning forest, which is the union of minimum spanning trees
/// in each connected component. The weight() method returns the
/// weight of a minimum spanning tree and the edges() method
/// returns its edges.
/// This implementation uses Prim's algorithm with an indexed
/// binary heap.
pub struct PrimMST {
    edge_to: Vec<Option<Edge>>, // edgeTo[v] = shortest edge from tree vertex to non-tree vertex
    dist_to: Vec<f32>,          // distTo[v] = weight of shortest such edge
    marked: Vec<bool>,          // marked[v] = true if v on tree, false otherwise
    pq: IndexPQ<f32>,           // eligible crossing edges
}

impl PrimMST {
    /// Compute a minimum spanning tree (or forest) of an edge-weighted graph
    pub fn new(g: &dyn IEWGraph) -> Self {
        let gv = g.V();
        let mut mst = Self {
            edge_to: vec![None; gv],
            dist_to: vec![f32::MAX; gv],
            marked: vec![false; gv],
            pq: IndexPQ::new_min_pq(gv),
        };

        for s in 0..gv {
            if !mst.marked[s] {
                mst.prim(g, s);
            }
        }

        mst
    }

    /// Returns the edges in a minimum spanning tree (or forest)
    pub fn edges(&self) -> Vec<Edge> {
        self.edge_to.iter().cloned().flatten().collect()
    }

    /// Returns the sum of the edge weights in a minimum spanning tree (or forest)
    pub fn weight(&self) -> f32 {
        self.edge_to
            .iter()
            .flatten()
            .fold(0.0, |acc, e| acc + e.weight())
    }

    /// run Prim's algorithm in graph G, starting from vertex s
    fn prim(&mut self, g: &dyn IEWGraph, s: usize) {
        let _ = self.pq.enqueue(s, 0.0);
        while !self.pq.is_empty() {
            let v = self.pq.dequeue().unwrap();
            self.scan(g, v);
        }
    }

    fn scan(&mut self, g: &dyn IEWGraph, v: usize) {
        self.marked[v] = true;
        for e in g.adj(v) {
            let w = e.other(v);
            if self.marked[w] {
                continue;
            }
            if e.weight() < self.dist_to[w] {
                self.dist_to[w] = e.weight();
                self.edge_to[w] = Some(*e);
                if self.pq.contains(w) {
                    let _ = self.pq.decrease_key(w, self.dist_to[w]);
                } else {
                    let _ = self.pq.enqueue(w, self.dist_to[w]);
                }
            }
        }
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
        let mst = self.edges();
        for e in mst.iter() {
            // all edges in MST except e
            uf = UF::new(g.V());
            for f in mst.iter() {
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
                if uf.find(x) != uf.find(y) && f.weight() < e.weight() {
                    return Err(format!(
                        "Edge {} violates cut optimality conditions",
                        f.to_string()
                    ));
                }
            }
        }

        Ok(())
    }
}
