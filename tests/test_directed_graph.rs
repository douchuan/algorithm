use algo::graph::directed::{
    Digraph, DirectedCycle, DirectedDFS, KosarajuSCC, SymbolGraph, Topological,
};
use std::str::FromStr;

const TINY_DG: &'static str = include_str!("res/graph/tinyDG.txt");
const JOBS: &'static str = include_str!("res/graph/jobs.txt");

#[test]
fn search() {
    let i = TINY_DG;
    let graph = Digraph::from_str(i).unwrap();
    let reach = DirectedDFS::new_single(&graph, 1);
    assert!(!reach.marked(0));
    assert!(reach.marked(1));
    let reach = DirectedDFS::new_multi(&graph, &vec![0, 1]);
    assert!(reach.marked(2));
    assert!(reach.marked(3));
}

#[test]
fn cycle() {
    let i = TINY_DG;
    let graph = Digraph::from_str(i).unwrap();
    let cycle = DirectedCycle::new(&graph);
    assert!(cycle.has_cycle());
}

#[test]
fn topological() {
    let i = TINY_DG;
    let graph = Digraph::from_str(i).unwrap();
    let cycle = Topological::new(&graph);
    assert!(!cycle.is_dag());

    let i = JOBS;
    let symbol_graph = SymbolGraph::new(i, "/");
    let graph = symbol_graph.G();
    let cycle = Topological::new(graph);
    assert!(cycle.is_dag());
}

#[test]
fn scc() {
    let i = TINY_DG;
    let graph = Digraph::from_str(i).unwrap();
    let scc = KosarajuSCC::new(&graph);
    assert_eq!(5, scc.count());
}
