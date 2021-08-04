use crate::graph::{IEWDigraph, IGraph};
use std::iter::Rev;
use std::slice::Iter;

/// The DepthFirstOrder represents a data type for
/// determining depth-first search ordering of the vertices in a digraph
/// or edge-weighted digraph, including preorder, postorder, and reverse
/// postorder.
/// This implementation uses depth-first search.
pub struct DepthFirstOrders {
    pre_order: Vec<usize>,  // vertices in preorder
    pre: Vec<usize>,        // pre[v] = preorder number of v
    pre_counter: usize,     // counter or preorder numbering
    post_order: Vec<usize>, // vertices in postorder
    post: Vec<usize>,       // post[v] = postorder number of v
    post_counter: usize,    // counter for postorder numbering
    marked: Vec<bool>,      // marked[v] = has v been marked in dfs?
}

impl DepthFirstOrders {
    /// Returns the vertices in preorder.
    pub fn pre(&self) -> Iter<'_, usize> {
        self.pre_order.iter()
    }

    /// Returns the vertices in postorder
    pub fn post(&self) -> Iter<'_, usize> {
        self.post_order.iter()
    }

    /// Returns the vertices in reverse postorder.
    pub fn rev_post(&self) -> Rev<Iter<'_, usize>> {
        self.post_order.iter().rev()
    }
}

impl DepthFirstOrders {
    fn new(nv: usize) -> Self {
        Self {
            pre_order: Vec::with_capacity(nv),
            pre: vec![0; nv],
            pre_counter: 0,
            post_order: Vec::with_capacity(nv),
            post: vec![0; nv],
            post_counter: 0,
            marked: vec![false; nv],
        }
    }

    /// run DFS in digraph G from vertex v and compute preorder/postorder
    fn dfs(&mut self, graph: &dyn IGraph, v: usize) {
        self.marked[v] = true;
        self.pre[v] = self.pre_counter;
        self.pre_counter += 1;
        self.pre_order.push(v);
        for &w in graph.adj(v) {
            if !self.marked[w] {
                self.dfs(graph, w);
            }
        }
        self.post_order.push(v);
        self.post[v] = self.post_counter;
        self.post_counter += 1;
    }

    /// run DFS in edge-weighted digraph G from vertex v and compute preorder/postorder
    fn dfs_ewd(&mut self, graph: &dyn IEWDigraph, v: usize) {
        self.marked[v] = true;
        self.pre[v] = self.pre_counter;
        self.pre_counter += 1;
        self.pre_order.push(v);
        for e in graph.adj(v) {
            let w = e.to();
            if !self.marked[w] {
                self.dfs_ewd(graph, w);
            }
        }
        self.post_order.push(v);
        self.post[v] = self.post_counter;
        self.post_counter += 1;
    }

    /// check that pre() and post() are consistent with pre[v] and post[v]
    pub fn check(&self) -> Result<(), &'static str> {
        let mut r = 0;
        for &v in self.post() {
            if self.post[v] != r {
                return Err("post[v] and post() inconsistent");
            }
            r += 1;
        }

        r = 0;
        for &v in self.pre() {
            if self.pre[v] != r {
                return Err("pre[v] and pre() inconsistent");
            }
            r += 1;
        }

        Ok(())
    }
}

macro_rules! impl_from {
    ($G: ty, $dfs: ident) => {
        impl From<$G> for DepthFirstOrders {
            fn from(g: $G) -> Self {
                let nv = g.V();
                let mut order = Self::new(nv);

                for v in 0..nv {
                    if !order.marked[v] {
                        order.$dfs(g, v);
                    }
                }

                debug_assert!(order.check().is_ok());
                order
            }
        }
    };
}

impl_from!(&dyn IGraph, dfs);
impl_from!(&dyn IEWDigraph, dfs_ewd);
