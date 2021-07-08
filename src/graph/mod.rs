use crate::graph::parser::{parse_edges, parse_link, parse_vertices};
use crate::ll::{linked_list::Iter, LinkedList};
use std::str::FromStr;

pub mod parser;

pub struct Graph {
    nv: usize, // num of vertices
    ne: usize, // num of edges
    adj: Vec<LinkedList<u32>>,
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
    pub fn add_edge(&mut self, v: u32, w: u32) {
        self.adj[v as usize].push_back(w);
        self.adj[w as usize].push_back(v);
        self.ne += 1;
    }

    /// vertices adjacent to v
    pub fn adj(&self, v: u32) -> Iter<'_, u32> {
        self.adj[v as usize].iter()
    }

    /// compute the degree of v
    pub fn degree(&self, v: u32) -> usize {
        let mut degree = 0;
        for _ in self.adj(v) {
            degree += 1;
        }
        degree
    }

    /// compute maximum degree
    pub fn max_degree(&self) -> usize {
        let mut max = 0;
        for v in 0..self.V() {
            max = std::cmp::max(max, self.degree(v as u32));
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
            for &w in self.adj(v as u32) {
                if v as u32 == w {
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
                .adj(v as u32)
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
