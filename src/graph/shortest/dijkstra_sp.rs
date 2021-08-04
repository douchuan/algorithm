//! Dijkstra's algorithm. Computes the shortest path tree.
//! Assumes all weights are non-negative.

use crate::common::IndexPQ;
use crate::graph::shortest::DirectedEdge;
use crate::graph::IEWDigraph;

pub struct DijkstraSP {
    dist_to: Vec<f32>,                  // distTo[v] = distance  of shortest s->v path
    edge_to: Vec<Option<DirectedEdge>>, // edgeTo[v] = last edge on shortest s->v path
    pq: IndexPQ<f32>,                   // priority queue of vertices
}

/// The DijkstraAllPairsSP represents a data type for solving the
/// all-pairs shortest paths problem in edge-weighted digraphs
/// where the edge weights are non-negative.
pub struct DijkstraAllPairsSP {
    all: Vec<DijkstraSP>,
}

impl DijkstraSP {
    pub fn new(g: &dyn IEWDigraph, s: usize) -> Self {
        for e in g.edges() {
            if e.weight() < 0.0 {
                panic!("edge {} has negative weight", e.to_string());
            }
        }

        let gv = g.V();
        let mut sp = Self {
            dist_to: vec![f32::MAX; gv],
            edge_to: vec![None; gv],
            pq: IndexPQ::new_min_pq(gv),
        };

        sp.dist_to[s] = 0.0;
        let _ = sp.pq.enqueue(s, sp.dist_to[s]);
        while !sp.pq.is_empty() {
            let v = sp.pq.dequeue().unwrap();
            for e in g.adj(v) {
                sp.relax(e);
            }
        }

        sp
    }

    // relax edge e and update pq if changed
    fn relax(&mut self, e: &DirectedEdge) {
        let v = e.from();
        let w = e.to();
        if self.dist_to[w] > self.dist_to[v] + e.weight() {
            self.dist_to[w] = self.dist_to[v] + e.weight();
            self.edge_to[w] = Some(*e);
            if self.pq.contains(w) {
                let _ = self.pq.decrease_key(w, self.dist_to[w]);
            } else {
                let _ = self.pq.enqueue(w, self.dist_to[w]);
            }
        }
    }

    /// Returns the length of a shortest path from the source vertex s to vertex v
    pub fn dist_to(&self, v: usize) -> f32 {
        self.dist_to[v]
    }

    /// Returns true if there is a path from the source vertex s to vertex v
    pub fn has_path_to(&self, v: usize) -> bool {
        self.dist_to[v] < f32::MAX
    }

    /// Returns a shortest path from the source vertex s to vertex v
    pub fn path_to(&self, v: usize) -> Option<Vec<DirectedEdge>> {
        if !self.has_path_to(v) {
            None
        } else {
            let mut path = Vec::new();
            let mut edge = self.edge_to[v];
            while let Some(e) = edge {
                path.push(e);
                edge = self.edge_to[e.from()];
            }
            path.reverse();
            Some(path)
        }
    }

    pub fn check(&self, g: &dyn IEWDigraph, s: usize) -> Result<(), String> {
        for e in g.edges() {
            if e.weight() < 0.0 {
                return Err("negative edge weight detected".to_string());
            }
        }

        // check that dist_to[v] and edge_to[v] are consistent
        if self.dist_to[s] != 0.0 || self.edge_to[s].is_some() {
            return Err("dist_to[s] and edge_to[s] inconsistent".to_string());
        }
        for v in 0..g.V() {
            if v == s {
                continue;
            }
            if self.edge_to[v].is_none() && self.dist_to[v] != f32::MAX {
                return Err("dist_to[s] and edge_to[s] inconsistent".to_string());
            }
        }

        // check that all edges e = v->w satisfy dist_to[w] <= dist_to[v] + e.weight()
        for v in 0..g.V() {
            for e in g.adj(v) {
                let w = e.to();
                if self.dist_to[v] + e.weight() < self.dist_to[w] {
                    return Err(format!("edge {} no relaxed", e.to_string()));
                }
            }
        }

        // check that all edges e = v->w on SPT satisfy dist_to[w] == dist_to[v] + e.weight()
        for w in 0..g.V() {
            if self.edge_to[w].is_none() {
                continue;
            }
            let e = self.edge_to[w].unwrap();
            let v = e.from();
            if self.dist_to[v] + e.weight() != self.dist_to[w] {
                return Err(format!("edge {} on shortest path not tight", e.to_string()));
            }
        }

        Ok(())
    }
}

impl DijkstraAllPairsSP {
    /// Computes a shortest paths tree from each vertex to to every other vertex in
    /// the edge-weighted digraph
    pub fn new(g: &dyn IEWDigraph) -> Self {
        let mut all = Vec::with_capacity(g.V());
        for v in 0..g.V() {
            all.push(DijkstraSP::new(g, v));
        }
        Self { all }
    }

    /// Returns a shortest path from vertex s to vertex t
    pub fn path(&self, s: usize, t: usize) -> Option<Vec<DirectedEdge>> {
        self.all[s].path_to(t)
    }

    /// Is there a path from the vertex s to vertex t?
    pub fn has_path(&self, s: usize, t: usize) -> bool {
        self.all[s].dist_to(t) < f32::MAX
    }

    /// Returns the length of a shortest path from vertex s to vertex t
    pub fn dist(&self, s: usize, t: usize) -> f32 {
        self.all[s].dist_to(t)
    }
}
