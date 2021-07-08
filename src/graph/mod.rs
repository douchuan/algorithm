use crate::graph::parser::{parse_edges, parse_link, parse_vertices};
use std::str::FromStr;

pub mod parser;

pub struct Graph {
    nv: usize, // num of vertices
    ne: usize, // num of edges
    links: Vec<(i32, i32)>,
}

impl Graph {
    /// create a V-vertex graph with no edges
    pub fn new(vertices: usize) -> Self {
        Self {
            nv: vertices,
            ne: 0,
            links: Vec::new(),
        }
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
    pub fn add_edge(&mut self, v: i32, w: i32) {
        unimplemented!()
    }

    /// vertices adjacent to v
    pub fn adj(&self, v: i32) -> Vec<i32> {
        unimplemented!()
    }
}

impl ToString for Graph {
    fn to_string(&self) -> String {
        let mut list = Vec::with_capacity(2 + self.links.len());
        list.push(format!("{}", self.nv));
        list.push(format!("{}", self.ne));
        for (v, w) in &self.links {
            list.push(format!("{} {}", v, w));
        }
        list.join("\n")
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
        // line2...: links
        let mut links = Vec::new();
        for s in lines {
            if !s.is_empty() {
                let (_, v) = parse_link(s).ok().ok_or(())?;
                links.push((v[0], v[1]));
            }
        }

        Ok(Self { nv, ne, links })
    }
}