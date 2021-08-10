use crate::graph::util::parser::parse_list_str;
use crate::graph::IGraph;
use std::collections::HashMap;

/// Typical applications involve processing graphs defined in files or
/// on web pages, using strings, not integer indices, to define and refer
/// to vertices. To accommodate such applications, we define an input
/// format with the following properties:
/// - Vertex names are strings.
/// - A specified delimiter separates vertex names (to allow for the possibility
///   of spaces in names).
/// - Each line represents a set of edges, connecting the first vertex name on
///   the line to each of the other vertices named on the line.
/// - The number of vertices V and the number of edges E are both implicitly
///   defined.
///
/// It builds three data structures:
/// - A symbol table st with String keys (vertex names) and int values (indices)
/// - An array keys[] that serves as an inverted index, giving the vertex name
///   associated with each integer index
/// - A Graph G built using the indices to refer to vertices
///
/// SymbolGraph uses two passes through the data to build these data structures
///
/// symbol table
/// ST<String, Integer> st
///   JFK | 0
///   MCO | 1
///   ORD | 2
///   ...
///
/// inverted index
/// String[] keys
///   0 JFK
///   1 MCO
///   2 ORD
///   ...
///
pub struct SymbolGraph<'a> {
    st: HashMap<&'a str, usize>,
    keys: Vec<&'a str>,
    graph: Box<dyn IGraph>,
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
    pub fn G(&self) -> &dyn IGraph {
        self.graph.as_ref()
    }
}

impl<'a> SymbolGraph<'a> {
    /// build graph specified in i using delim to separate vertex names
    pub fn new<F>(i: &'a str, sep: &str, graph_ctor: F) -> Self
    where
        F: FnOnce(usize) -> Box<dyn IGraph>,
    {
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
        let mut graph = graph_ctor(st.len());
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
