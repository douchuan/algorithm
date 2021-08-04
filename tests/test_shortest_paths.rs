#[macro_use]
extern crate approx;

use algo::graph::directed::{DepthFirstOrders, EdgeWeightedDigraphCycle};
use algo::graph::shortest::{DijkstraAllPairsSP, DijkstraSP, EWDigraph};
use algo::graph::IEWDigraph;
use std::str::FromStr;

const TINY_EWD: &'static str = include_str!("res/graph/tinyEWD.txt");

#[test]
fn parse() {
    let i = TINY_EWD;
    let graph = create_graph(i);
    assert_eq!(graph.V(), 8);
    assert_eq!(graph.E(), 15);
    /*
    8 15
    0: 0->2  0.26  0->4  0.38
    1: 1->3  0.29
    2: 2->7  0.34
    3: 3->6  0.52
    4: 4->7  0.37  4->5  0.35
    5: 5->1  0.32  5->7  0.28  5->4  0.35
    6: 6->4  0.93  6->0  0.58  6->2  0.40
    7: 7->3  0.39  7->5  0.28
        */
    println!("{}", graph.to_string());
}

#[test]
fn dfs() {
    let i = TINY_EWD;
    let graph = create_graph(i);
    let order = DepthFirstOrders::from(graph.as_ref());
    assert!(order.check().is_ok());
}

#[test]
fn cycle() {
    let i = TINY_EWD;
    let graph = create_graph(i);
    let cycle = EdgeWeightedDigraphCycle::from(graph.as_ref());
    assert!(cycle.check().is_ok());
}

#[allow(non_snake_case)]
#[test]
fn Dijkstra_sp() {
    let i = TINY_EWD;
    let graph = create_graph(i);
    let sp = DijkstraSP::new(graph.as_ref(), 0);
    for (i, (weight, size)) in vec![
        (0.0, 0), // (dist_to(i), path_to(i).len())
        (1.05, 3),
        (0.26, 1),
        (0.99, 3),
        (0.38, 1),
        (0.73, 2),
        (1.51, 4),
        (0.6, 2),
    ]
    .iter()
    .enumerate()
    {
        assert_eq!(*size, sp.path_to(i).map_or(0, |path| path.len()));
        assert_relative_eq!(*weight, sp.dist_to(i));
    }

    assert!(sp.check(graph.as_ref(), 0).is_ok());
}

#[allow(non_snake_case)]
#[test]
fn Dijkstra_all_pairs_sp() {
    let i = TINY_EWD;
    let graph = create_graph(i);
    let all = DijkstraAllPairsSP::new(graph.as_ref());
    assert!(all.has_path(0, 1));
}

fn create_graph(i: &str) -> Box<dyn IEWDigraph> {
    Box::new(EWDigraph::from_str(i).unwrap())
}
