use crate::common::Queue;
use crate::graph::directed::EdgeWeightedDirectedCycle;
use crate::graph::shortest::{DirectedEdge, EWDigraph};
use crate::graph::IEWDigraph;

/// The BellmanFordSP represents a data type for solving the
/// single-source shortest paths problem in edge-weighted digraphs with
/// no negative cycles.
/// The edge weights can be positive, negative, or zero.
/// This class finds either a shortest path from the source vertex s
/// to every other vertex or a negative cycle reachable from the source vertex.
/// This implementation uses a queue-based implementation of
/// the Bellman-Ford-Moore algorithm.
pub struct BellmanFordSP {
    dist_to: Vec<f32>,                  // dist_to[v] = distance of shortest s->v path
    edge_to: Vec<Option<DirectedEdge>>, // edge_to[v] = last edge on shortest s->v path
    on_queue: Vec<bool>,                // on_queue[v] = is v currently on the queue?
    queue: Queue<usize>,                // queue of vertices to relax
    cost: usize,                        // number of calls to relax()
    cycle: Option<Vec<DirectedEdge>>,   // negative cycle (or null if no such cycle)
}

impl BellmanFordSP {
    /// Computes a shortest paths tree from s to every other vertex in
    /// the edge-weighted digraph G.
    pub fn new(g: &dyn IEWDigraph, s: usize) -> Self {
        let nv = g.V();
        let mut sp = Self {
            dist_to: vec![f32::MAX; nv],
            edge_to: vec![None; nv],
            on_queue: vec![false; nv],
            queue: Queue::new(),
            cost: 0,
            cycle: None,
        };

        // Bellman-Ford algorithm
        sp.dist_to[s] = 0.0;
        sp.queue.enqueue(s);
        sp.on_queue[s] = true;
        while !sp.queue.is_empty() && !sp.has_negative_cycle() {
            let v = sp.queue.dequeue().unwrap();
            sp.on_queue[v] = false;
            sp.relax(g, v);
        }

        sp
    }

    /// relax vertex v and put other endpoints on queue if changed
    fn relax(&mut self, g: &dyn IEWDigraph, v: usize) {
        for e in g.adj(v) {
            let w = e.to();
            if self.dist_to[w] > self.dist_to[v] + e.weight() + f32::EPSILON {
                self.dist_to[w] = self.dist_to[v] + e.weight();
                self.edge_to[w] = Some(*e);
                if !self.on_queue[w] {
                    self.queue.enqueue(w);
                    self.on_queue[w] = true;
                }
            }

            self.cost += 1;
            if self.cost % g.V() == 0 {
                self.find_negative_cycle();
                if self.has_negative_cycle() {
                    return;
                }
            }
        }
    }

    /// by finding a cycle in predecessor graph
    fn find_negative_cycle(&mut self) {
        let nv = self.edge_to.len();
        // spt: shortest path tree
        let mut spt: Box<dyn IEWDigraph> = Box::new(EWDigraph::new(nv));
        for v in 0..nv {
            if let Some(e) = self.edge_to[v] {
                spt.add_edge(e.from(), e.to(), e.weight());
            }
        }

        let finder = EdgeWeightedDirectedCycle::from(spt.as_ref());
        if let Some(cycle) = finder.cycle() {
            self.cycle = Some(cycle.cloned().collect());
        }
    }

    /// Is there a negative cycle reachable from the source vertex s?
    pub fn has_negative_cycle(&self) -> bool {
        self.cycle.is_some()
    }

    /// Returns a negative cycle reachable from the source vertex s, or None
    /// if there is no such cycle.
    pub fn negative_cycle(&self) -> Option<std::slice::Iter<'_, DirectedEdge>> {
        self.cycle.as_ref().and_then(|v| Some(v.iter()))
    }

    // check optimality conditions: either
    // (i) there exists a negative cycle reachable from s
    //     or
    // (ii)  for all edges e = v->w:            distTo[w] <= distTo[v] + e.weight()
    // (ii') for all edges e = v->w on the SPT: distTo[w] == distTo[v] + e.weight()
    pub fn check(&self, g: &dyn IEWDigraph, s: usize) -> Result<(), String> {
        // has a negative cycle
        if let Some(cycle) = self.cycle.clone() {
            let mut weight = 0.0;
            for e in cycle {
                weight += e.weight();
            }
            if weight >= 0.0 {
                return Err(format!("weight of netative cycle = {}", weight));
            }
        }
        // no negative cycle reachable from source
        else {
            if self.dist_to[s] != 0.0 || self.edge_to[s].is_some() {
                return Err("dist_to[s] and edge_to[s] inconsistent".to_string());
            }
            for v in 0..g.V() {
                if v != s {
                    if self.edge_to[v].is_none() && self.dist_to[v] != f32::MAX {
                        return Err("dist_to[] and edge_to[] inconsistent".to_string());
                    }
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
                if let Some(e) = self.edge_to[w] {
                    let v = e.from();
                    if w != e.to() {
                        return Err(format!("illegal edge {}", e.to_string()));
                    }
                    if self.dist_to[v] + e.weight() != self.dist_to[w] {
                        return Err(format!("edge {} on shortest path not tight", e.to_string()));
                    }
                }
            }
        }

        Ok(())
    }
}
