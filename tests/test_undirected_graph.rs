use algo::graph::undirected::{Cycle, DepthFirstSearch, Graph, Search, TowColor, CC};
use algo::graph::util::{BreadthFirstPaths, DepthFirstPaths, Paths, SymbolGraph};
use algo::graph::IGraph;
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
    let graph = create_graph(TINY_G);
    let search = DepthFirstSearch::new(&graph, 0);
    assert_ne!(search.count(), graph.V());
    assert!(vec![1, 2, 3, 4, 5, 6].iter().all(|&w| search.marked(w)));
    assert!(vec![7, 8, 9, 10, 11, 12].iter().all(|&w| !search.marked(w)));
}

#[test]
fn depth_first_paths() {
    let graph = create_graph(TINY_CG);
    let paths = DepthFirstPaths::new(&graph, 0);
    for (v, expect_path) in vec![
        Some(vec![0]),          // 0
        Some(vec![0, 2, 1]),    // 1
        Some(vec![0, 2]),       // 2
        Some(vec![0, 2, 3]),    // 3
        Some(vec![0, 2, 3, 4]), // 4
        Some(vec![0, 2, 3, 5]), // 5
    ]
    .iter()
    .enumerate()
    {
        assert_eq!(expect_path, &paths.path_to(v));
    }
}

#[test]
fn breadth_first_paths() {
    let graph = create_graph(TINY_CG);
    let paths = BreadthFirstPaths::new(&graph, 0);
    for (v, expect_path) in vec![
        Some(vec![0]),       // 0
        Some(vec![0, 1]),    // 1
        Some(vec![0, 2]),    // 2
        Some(vec![0, 2, 3]), // 3
        Some(vec![0, 2, 4]), // 4
        Some(vec![0, 5]),    // 5
    ]
    .iter()
    .enumerate()
    {
        assert_eq!(expect_path, &paths.path_to(v));
        let expect_dist = expect_path.as_ref().map_or(usize::MAX, |v| v.len() - 1);
        assert_eq!(expect_dist, paths.dist_to(v));
    }
}

#[test]
fn bfs_connected_components() {
    let graph = create_graph(TINY_G);
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
    let graph = create_graph(TINY_G);
    let c = Cycle::new(&graph);
    assert!(c.has_cycle());
}

#[test]
fn two_color() {
    let graph = create_graph(TINY_G);
    let c = TowColor::new(&graph);
    assert!(!c.is_bipartite());
}

#[test]
fn symbol_graph() {
    let i = ROUTES;
    let symbol = SymbolGraph::new(i, " ", |nv| Box::new(Graph::new(nv)));
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
fn degree_of_separation() {
    let i = ROUTES;
    let symbol = SymbolGraph::new(&i, " ", |nv| Box::new(Graph::new(nv)));
    let graph = symbol.G();
    let source = "JFK";
    let sink = "LAS";
    let expect = vec!["JFK", "ORD", "PHX", "LAS"];
    let finder = BreadthFirstPaths::new(graph, symbol.index(source).unwrap());
    assert!(finder.has_path(symbol.index(sink).unwrap()));
    let paths = finder.path_to(symbol.index(sink).unwrap()).unwrap();
    let mut path_names = vec![];
    for p in paths {
        path_names.push(symbol.name(p).unwrap());
    }
    assert_eq!(expect, path_names);
}

#[test]
fn locate_file() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("tests/res/graph/tinyCG.txt");
    assert!(d.exists(), "d = {}", d.display());
}

fn create_graph(i: &str) -> Box<dyn IGraph> {
    Box::new(Graph::from_str(i).unwrap())
}
