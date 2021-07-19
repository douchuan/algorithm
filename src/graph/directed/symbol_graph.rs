//! The SymbolGraph represents an undirected graph, where the
//! vertex names are arbitrary strings.
//! By providing mappings between string vertex names and integers,
//! it serves as a wrapper around the
//! Graph data type, which assumes the vertex names are integers
//! between 0 and V - 1.

use crate::graph::directed::Digraph;
use crate::graph::util::parser::parse_list_str;
use crate::graph::IGraph;
use std::collections::HashMap;

pub struct SymbolGraph<'a> {
    st: HashMap<&'a str, usize>,
    keys: Vec<&'a str>,
    graph: Digraph,
}

impl<'a> SymbolGraph<'a> {
    /// is key a vertex?
    pub fn contains(&self, key: &str) -> bool {
        self.st.contains_key(key)
    }

    /// index associated with key
    pub fn index(&self, key: &str) -> Option<usize> {
        self.st.get(key).cloned()
    }

    /// key associated with index v
    pub fn name(&self, v: usize) -> Option<&str> {
        self.keys.get(v).cloned()
    }

    /// underlying Graph
    #[allow(non_snake_case)]
    pub fn G(&self) -> &Digraph {
        &self.graph
    }
}

impl<'a> SymbolGraph<'a> {
    /// build graph specified in i using delim to separate vertex names
    pub fn new(i: &'a str, sep: &str) -> Self {
        // First pass
        //   builds the index, by reading strings to associate each
        //   distinct string with an index.
        let mut st = HashMap::new();
        for l in i.lines() {
            if let Ok((_, list)) = parse_list_str(l, sep) {
                for v in list {
                    if st.get(v).is_none() {
                        st.insert(v, st.len());
                    }
                }
            }
        }

        // Inverted index
        // to get string keys is an array.
        let mut keys = vec![""; st.len()];
        for &name in st.keys() {
            keys[*st.get(name).unwrap()] = name;
        }

        // Second pass
        //   builds the graph
        let mut graph = Digraph::new(st.len());
        for l in i.lines() {
            if let Ok((_, list)) = parse_list_str(l, sep) {
                let v = *st.get(list[0]).unwrap();
                for &s in &list[1..] {
                    let w = *st.get(s).unwrap();
                    graph.add_edge(v, w);
                }
            }
        }

        Self { st, keys, graph }
    }
}
