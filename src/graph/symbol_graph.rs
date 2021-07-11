use crate::graph::Graph;

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
pub struct SymbolGraph {}

impl SymbolGraph {
    /// build graph specified in i using delim to separate vertex names
    pub fn new(i: &str, delim: &str) -> Self {
        todo!()
    }

    /// is key a vertex?
    pub fn contains(&self, key: &str) -> bool {
        todo!()
    }

    /// index associated with key
    pub fn index(&self, key: &str) -> usize {
        todo!()
    }

    /// key associated with index v
    pub fn name(&self, v: usize) -> &str {
        todo!()
    }

    /// underlying Graph
    #[allow(non_snake_case)]
    pub fn G(&self) -> &Graph {
        todo!()
    }
}
