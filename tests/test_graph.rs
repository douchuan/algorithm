use algo::graph::{
    BreadthFirstPaths, Cycle, DepthFirstPaths, DepthFirstSearch, Graph, Paths, Search, SymbolGraph,
    TowColor, CC,
};
use std::path::PathBuf;
use std::str::FromStr;

const TINY_G: &'static str = include_str!("res/graph/tinyG.txt");
const TINY_CG: &'static str = include_str!("res/graph/tinyCG.txt");
const ROUTES: &'static str = include_str!("res/graph/routes.txt");

#[test]
fn parser() {
    //test strip
    assert_eq!("abc".strip_suffix("\n"), None);
    assert_eq!("abc\n".strip_suffix("\n"), Some("abc"));

    //test parser
    let s = TINY_G;
    let s = s.strip_suffix("\n").unwrap_or(s);
    let graph = Graph::from_str(s).unwrap();
    assert_eq!(graph.E(), 13);
    assert_eq!(graph.V(), 13);

    let expect = vec![6, 2, 1, 5];
    let adj0 = graph.adj(0);
    for (i, w) in adj0.enumerate() {
        assert_eq!(*w, expect[i]);
    }

    assert_eq!(graph.degree(0), 4);
    assert_eq!(graph.degree(1), 1);

    /*
    13 vertices, 13 edges
    0: 6 2 1 5
    1: 0
    2: 0
    3: 5 4
    4: 5 6 3
    5: 3 4 0
    6: 0 4
    7: 8
    8: 7
    9: 11 10 12
    10: 9
    11: 9 12
    12: 11 9
    */
    // println!("{}", graph.to_string());
}

#[test]
fn search() {
    let i = TINY_G;
    let graph = Graph::from_str(i).unwrap();
    let search = DepthFirstSearch::new(&graph, 0);
    assert_ne!(search.count(), graph.V());
    assert!(vec![1, 2, 3, 4, 5, 6].iter().all(|&w| search.marked(w)));
    assert!(vec![7, 8, 9, 10, 11, 12].iter().all(|&w| !search.marked(w)));
}

#[test]
fn deep_first_paths() {
    let i = TINY_CG;
    let graph = Graph::from_str(i).unwrap();
    let paths = DepthFirstPaths::new(&graph, 0);
    assert_eq!(paths.path_to(5), Some(vec![0, 2, 3, 5]));
}

#[test]
fn breadth_first_paths() {
    let i = TINY_CG;
    let graph = Graph::from_str(i).unwrap();
    let paths = BreadthFirstPaths::new(&graph, 0);
    assert_eq!(paths.path_to(4), Some(vec![0, 2, 4]));
    assert_eq!(paths.path_to(5), Some(vec![0, 5]));
}

#[test]
fn bfs_connected_components() {
    let i = TINY_G;
    let graph = Graph::from_str(i).unwrap();
    let cc = CC::new(&graph);
    assert_eq!(cc.count(), 3);
    assert!(cc.connected(0, 5));
    assert!(!cc.connected(7, 9));
    assert_eq!(cc.id(0), 0);
    assert_eq!(cc.id(7), 1);
    assert_eq!(cc.id(9), 2);
}

#[test]
fn cycle() {
    let i = TINY_G;
    let graph = Graph::from_str(i).unwrap();
    let c = Cycle::new(&graph);
    assert!(c.has_cycle());
}

#[test]
fn two_color() {
    let i = TINY_G;
    let graph = Graph::from_str(i).unwrap();
    let c = TowColor::new(&graph);
    assert!(!c.is_bipartite());
}

#[test]
fn symbol_graph() {
    let i = ROUTES;
    let symbol = SymbolGraph::new(i, " ");
    let graph = symbol.G();
    let mut expect = vec!["ORD", "ATL", "MCO"];
    expect.sort();
    let mut adj = vec![];
    let u = symbol.index("JFK").unwrap();
    for &w in graph.adj(u) {
        adj.push(symbol.name(w).unwrap());
    }
    adj.sort();
    assert_eq!(expect, adj);
}

#[test]
fn locate_file() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("tests/res/graph/tinyCG.txt");
    assert!(d.exists(), "d = {}", d.display());
}
