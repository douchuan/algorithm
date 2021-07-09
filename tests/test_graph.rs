use algo::graph::{DepthFirstSearch, Graph, Search};
use std::str::FromStr;

const TINY_G: &'static str = include_str!("res/graph/tinyG.txt");

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

    let expect = vec![5, 1, 2, 6];
    let adj0 = graph.adj(0);
    for (i, w) in adj0.enumerate() {
        assert_eq!(*w, expect[i]);
    }

    assert_eq!(graph.degree(0), 4);
    assert_eq!(graph.degree(1), 1);

    // println!("{}", graph.to_string());
}

#[test]
fn dfs() {
    let s = TINY_G;
    let graph = Graph::from_str(s).unwrap();
    let search = DepthFirstSearch::new(&graph, 0);
    assert_ne!(search.count(), graph.V());
    assert!(vec![1, 2, 3, 4, 5, 6].iter().all(|&w| search.marked(w)));
    assert!(vec![7, 8, 9, 10, 11, 12].iter().all(|&w| !search.marked(w)));
}
