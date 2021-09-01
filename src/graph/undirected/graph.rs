//!
//! Algorithhms 4th Edition by Robert Sedgewick, Kevin Wayne
//! CHAPTER 4
//! Representation alternatives.
//!
//! The next decision that we face in graph processing is which
//! graph representation (data structure) to use to implement
//! this API. We have two basic requirements:
//!   - We must have the space to accommodate the types of graphs
//!     that we are likely to encounter in applications.
//!   - We want to develop time-efficient implementations of Graph
//!     instance methods the basic methods that we need to develop
//!     graph processing clients.
//!
//! These requirements are a bit vague, but they are still helpful
//! in choosing among the three data structures that immediately
//! suggest themselves for representing graphs:
//!
//! - An adjacency matrix, where we main- tain a V-by-V boolean array,
//!   with the entry in row v and column w defined to be true if there is
//!   an edge adjacent to both vertex v and vertex w in the graph, and to
//!   be false otherwise. This representation fails on the first count graphs
//!   with millions of vertices are common and the space cost for the V 2 boolean
//!   values needed is prohibitive.
//!   Beyond these performance objectives, a detailed examination reveals other
//!   considerations that can be important in some applications. For example,
//!   allowing parallel edges precludes the use of an adjacency matrix, since
//!   the adjacency matrix has no way to represent them.
//!
//! - An array of edges, using an Edge class with two instance variables of type
//!   int. This direct representation is simple, but it fails on the second count
//!   implementing adj() would involve examining all the edges in the graph.
//!
//! - An array of adjacency lists, where we maintain a vertex-indexed array of
//!   lists of the vertices adjacent to each vertex. This data structure satisfies
//!   both requirements for typical applications and is the one that we will use
//!   throughout this chapter.

use crate::graph::util::parser::GraphDataParser;
use crate::graph::IGraph;
use crate::ll::{linked_list::Iter, LinkedList};

pub struct Graph {
    nv: usize, // num of vertices
    ne: usize, // num of edges
    adj: Vec<LinkedList<usize>>,
}

impl IGraph for Graph {
    fn V(&self) -> usize {
        self.nv
    }

    fn E(&self) -> usize {
        self.ne
    }

    fn add_edge(&mut self, v: usize, w: usize) {
        // Algorithhms 4th Edition by Robert Sedgewick, Kevin Wayne
        // p538, Adjacency-lists data structure
        // first adjacent vertex in input is last on list
        self.adj[v].push_front(w);
        self.adj[w].push_front(v);
        self.ne += 1;
    }

    fn adj(&self, v: usize) -> Iter<'_, usize> {
        self.adj[v].iter()
    }
}

impl Graph {
    /// compute the degree of v
    pub fn degree(&self, v: usize) -> usize {
        self.adj(v).fold(0, |acc, _| acc + 1)
    }

    /// compute maximum degree
    pub fn max_degree(&self) -> usize {
        let mut max = 0;
        for v in 0..self.V() {
            max = std::cmp::max(max, self.degree(v));
        }
        max
    }

    /// compute average degree
    pub fn avg_degree(&self) -> f32 {
        2.0 * self.E() as f32 / self.V() as f32
    }

    /// count self-loops
    pub fn number_of_self_loops(&self) -> usize {
        let mut count = 0;
        for v in 0..self.V() {
            for &w in self.adj(v) {
                if v == w {
                    count += 1;
                }
            }
        }

        // each edge counted twice
        count / 2
    }
}

/// create a V-vertex graph with no edges
impl From<usize> for Graph {
    fn from(nv: usize) -> Self {
        let mut adj = Vec::with_capacity(nv);
        for _ in 0..nv {
            adj.push(LinkedList::default());
        }

        Self { nv, ne: 0, adj }
    }
}

impl From<&str> for Graph {
    fn from(s: &str) -> Self {
        let parser = GraphDataParser::parse(s, false).unwrap();
        let mut g = Self::from(parser.get_v());
        for (v, w) in parser.get_edges() {
            g.add_edge(*v, *w);
        }
        debug_assert!(g.E() == parser.get_e());
        g
    }
}
