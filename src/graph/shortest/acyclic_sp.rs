use crate::graph::directed::Topological;
use crate::graph::shortest::DirectedEdge;
use crate::graph::IEWDigraph;

/// The AcyclicSP represents a data type for solving the
/// single-source shortest paths problem in edge-weighted directed acyclic
/// graphs (DAGs). The edge weights can be positive, negative, or zero.
/// This implementation uses a topological-sort based algorithm.
pub struct AcyclicSP {
    dist_to: Vec<f32>,                  // dist_to[v] = distance  of shortest s->v path
    edge_to: Vec<Option<DirectedEdge>>, // edge_to[v] = last edge on shortest s->v path
}

/// The AcyclicLP represents a data type for solving the
/// single-source shortest paths problem in edge-weighted directed acyclic
/// graphs (DAGs). The edge weights can be positive, negative, or zero.
/// This implementation uses a topological-sort based algorithm.
pub struct AcyclicLP {
    dist_to: Vec<f32>,                  // dist_to[v] = distance  of longest s->v path
    edge_to: Vec<Option<DirectedEdge>>, // edge_to[v] = last edge on longest s->v path
}

impl AcyclicSP {
    pub fn new(g: &dyn IEWDigraph, s: usize) -> Result<Self, &'static str> {
        let topological = Topological::from(g);
        if let Some(order) = topological.order() {
            let nv = g.V();
            let mut sp = Self {
                dist_to: vec![f32::MAX; nv],
                edge_to: vec![None; nv],
            };
            sp.dist_to[s] = 0.0;

            // let order: Vec<usize> = order.cloned().collect();
            // println!("order = {:?}", order);

            for &v in order {
                for e in g.adj(v) {
                    sp.relax(e);
                }
            }

            Ok(sp)
        } else {
            Err("Digraph is not acyclic.")
        }
    }

    /// Returns the length of a shortest path from the source vertex s to vertex v
    pub fn dist_to(&self, v: usize) -> f32 {
        self.dist_to[v]
    }

    /// Is there a path from the source vertex s to vertex v?
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

    fn relax(&mut self, e: &DirectedEdge) {
        let v = e.from();
        let w = e.to();
        if self.dist_to[w] > self.dist_to[v] + e.weight() {
            self.dist_to[w] = self.dist_to[v] + e.weight();
            self.edge_to[w] = Some(*e);
        }
    }
}

impl AcyclicLP {
    pub fn new(g: &dyn IEWDigraph, s: usize) -> Result<Self, &'static str> {
        let topological = Topological::from(g);
        if let Some(order) = topological.order() {
            let nv = g.V();
            let mut sp = Self {
                dist_to: vec![f32::MIN; nv],
                edge_to: vec![None; nv],
            };
            sp.dist_to[s] = 0.0;

            // let order: Vec<usize> = order.cloned().collect();
            // println!("order = {:?}", order);

            for &v in order {
                for e in g.adj(v) {
                    sp.relax(e);
                }
            }

            Ok(sp)
        } else {
            Err("Digraph is not acyclic.")
        }
    }

    /// Returns the length of a longest path from the source vertex s to vertex v
    pub fn dist_to(&self, v: usize) -> f32 {
        self.dist_to[v]
    }

    /// Is there a path from the source vertex s to vertex v?
    pub fn has_path_to(&self, v: usize) -> bool {
        self.dist_to[v] > f32::MIN
    }

    /// Returns a longest path from the source vertex s to vertex v
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

    /// relax edge e, but update if you find a *longer* path
    fn relax(&mut self, e: &DirectedEdge) {
        let v = e.from();
        let w = e.to();
        if self.dist_to[w] < self.dist_to[v] + e.weight() {
            self.dist_to[w] = self.dist_to[v] + e.weight();
            self.edge_to[w] = Some(*e);
        }
    }
}
