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
//!     instance meth- ods—the basic methods that we need to develop
//!     graph-processing clients.
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

use crate::graph::parser::{parse_edges, parse_link, parse_vertices};
use crate::ll::{linked_list::Iter, LinkedList};
use std::str::FromStr;

pub struct Graph {
    nv: usize, // num of vertices
    ne: usize, // num of edges
    adj: Vec<LinkedList<usize>>,
}

impl Graph {
    /// create a V-vertex graph with no edges
    pub fn new(nv: usize) -> Self {
        let mut adj = Vec::with_capacity(nv);
        for _ in 0..nv {
            adj.push(LinkedList::default());
        }

        Self { nv, ne: 0, adj }
    }

    /// number of vertices
    #[allow(non_snake_case)]
    pub fn V(&self) -> usize {
        self.nv
    }

    /// number of edges
    #[allow(non_snake_case)]
    pub fn E(&self) -> usize {
        self.ne
    }

    /// add edge v-w to this graph
    pub fn add_edge(&mut self, v: usize, w: usize) {
        self.adj[v as usize].push_back(w);
        self.adj[w as usize].push_back(v);
        self.ne += 1;
    }

    /// vertices adjacent to v
    pub fn adj(&self, v: usize) -> Iter<'_, usize> {
        self.adj[v].iter()
    }

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

impl ToString for Graph {
    fn to_string(&self) -> String {
        let mut buf = Vec::new();
        buf.push(format!("{} vertices, {} edges", self.nv, self.ne));
        for v in 0..self.V() {
            let adj = self
                .adj(v)
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(" ");
            buf.push(format!("{}: {}", v, adj));
        }
        buf.join("\n")
    }
}

impl FromStr for Graph {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        // line0: V
        let s = lines.next().ok_or(())?;
        let (_, nv) = parse_vertices(s).ok().ok_or(())?;
        // line1: E
        let s = lines.next().ok_or(())?;
        let (_, ne) = parse_edges(s).ok().ok_or(())?;

        let mut graph = Self::new(nv);
        // line2...: links
        for s in lines {
            if !s.is_empty() {
                let (_, v) = parse_link(s).ok().ok_or(())?;
                graph.add_edge(v[0], v[1]);
            }
        }

        debug_assert_eq!(ne, graph.ne);

        Ok(graph)
    }
}
