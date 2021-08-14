use algo::graph::undirected::{Bipartite, Cycle, DepthFirstSearch, Graph, NonRecursiveDFS, CC};
use algo::graph::util::{BreadthFirstPaths, DepthFirstPaths, Paths, SymbolGraph};
use algo::graph::IGraph;
use std::path::PathBuf;
use std::str::FromStr;

const TINY_G: &'static str = include_str!("../res/graph/tinyG.txt");
const TINY_CG: &'static str = include_str!("../res/graph/tinyCG.txt");
const ROUTES: &'static str = include_str!("../res/graph/routes.txt");

#[test]
fn parser() {
    //test strip
    assert_eq!("abc".strip_suffix("\n"), None);
    assert_eq!("abc\n".strip_suffix("\n"), Some("abc"));

    //test parser
    let i = TINY_G;
    let i = i.strip_suffix("\n").unwrap_or(i);
    let graph = Graph::from_str(i).unwrap();
    assert_eq!(graph.V(), 13);
    assert_eq!(graph.E(), 13);

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
fn dfs() {
    let graph = create_graph(TINY_G);
    let dfs = DepthFirstSearch::new(graph.as_ref(), 0);
    assert_ne!(dfs.count(), graph.V());
    assert!(vec![1, 2, 3, 4, 5, 6].iter().all(|&w| dfs.marked(w)));
    assert!(vec![7, 8, 9, 10, 11, 12].iter().all(|&w| !dfs.marked(w)));
}

#[test]
fn non_recursive_dfs() {
    let graph = create_graph(TINY_G);
    let dfs = NonRecursiveDFS::new(graph.as_ref(), 0);
    assert!(vec![1, 2, 3, 4, 5, 6].iter().all(|&w| dfs.marked(w)));
    assert!(vec![7, 8, 9, 10, 11, 12].iter().all(|&w| !dfs.marked(w)));
}

#[test]
fn dfs_paths() {
    let graph = create_graph(TINY_CG);
    let paths = DepthFirstPaths::new(graph.as_ref(), 0);
    for (v, expect) in vec![
        Some(vec![0usize]),     // 0
        Some(vec![0, 2, 1]),    // 1
        Some(vec![0, 2]),       // 2
        Some(vec![0, 2, 3]),    // 3
        Some(vec![0, 2, 3, 4]), // 4
        Some(vec![0, 2, 3, 5]), // 5
    ]
    .iter()
    .enumerate()
    {
        let paths: Option<Vec<usize>> = paths
            .path_to(v)
            .and_then(|paths| Some(paths.iter().map(|&v| v).collect()));
        assert_eq!(expect, &paths);
    }
}

#[test]
fn bfs_paths() {
    let graph = create_graph(TINY_CG);
    let paths = BreadthFirstPaths::new(graph.as_ref(), 0);
    for (v, expect) in vec![
        Some(vec![0usize]),  // 0
        Some(vec![0, 1]),    // 1
        Some(vec![0, 2]),    // 2
        Some(vec![0, 2, 3]), // 3
        Some(vec![0, 2, 4]), // 4
        Some(vec![0, 5]),    // 5
    ]
    .iter()
    .enumerate()
    {
        {
            let paths: Option<Vec<usize>> = paths
                .path_to(v)
                .and_then(|paths| Some(paths.iter().map(|&v| v).collect()));
            assert_eq!(expect, &paths);
        }
        let expect_dist = expect.as_ref().map_or(usize::MAX, |v| v.len() - 1);
        assert_eq!(expect_dist, paths.dist_to(v));
    }
}

#[test]
fn cc() {
    let graph = create_graph(TINY_G);
    let cc = CC::new(graph.as_ref());
    assert_eq!(cc.count(), 3);
    let mut components = vec![Vec::new(); cc.count()];
    for v in 0..graph.V() {
        components[cc.id(v)].push(v);
    }
    let expect = vec![vec![0, 1, 2, 3, 4, 5, 6], vec![7, 8], vec![9, 10, 11, 12]];
    assert_eq!(expect, components);
}

#[test]
fn cycle() {
    let graph = create_graph(TINY_G);
    let c = Cycle::new(graph.as_ref());
    assert!(c.has_cycle());
    assert!(c.cycle().unwrap().eq(vec![3, 4, 5, 3].iter()));
}

#[test]
fn two_color() {
    let graph = create_graph(TINY_G);
    let c = Bipartite::new(graph.as_ref());
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
    d.push("res/graph/tinyCG.txt");
    assert!(d.exists(), "d = {}", d.display());
}

fn create_graph(i: &str) -> Box<dyn IGraph> {
    Box::new(Graph::from_str(i).unwrap())
}
