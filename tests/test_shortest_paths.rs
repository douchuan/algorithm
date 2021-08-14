#[macro_use]
extern crate approx;

use algo::graph::directed::{DepthFirstOrders, EdgeWeightedDirectedCycle};
use algo::graph::shortest::{
    AcyclicLP, AcyclicSP, Arbitrage, BellmanFordSP, DijkstraAllPairsSP, DijkstraSP, EWDigraph, CPM,
};
use algo::graph::IEWDigraph;
use std::str::FromStr;

const TINY_EWD: &'static str = include_str!("../res/graph/tinyEWD.txt");
const TINY_EWD_NEGATIVE: &'static str = include_str!("../res/graph/tinyEWDn.txt");
const TINY_EWD_NEGATIVE_CYCLE: &'static str = include_str!("../res/graph/tinyEWDnc.txt");
const TINY_EWDAG: &'static str = include_str!("../res/graph/tinyEWDAG.txt");
const JOBS_PC: &'static str = include_str!("../res/graph/jobsPC.txt");
const RATES: &'static str = include_str!("../res/graph/rates.txt");

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
    // println!("{}", graph.to_string());
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
    let cycle = EdgeWeightedDirectedCycle::from(graph.as_ref());
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

#[test]
fn acyclic_sp() {
    let i = TINY_EWDAG;
    let graph = create_graph(i);
    let sp = AcyclicSP::new(graph.as_ref(), 5).unwrap();
    for (i, (weight, size)) in vec![
        (0.73, 2), // (dist_to(i), path_to(i).len())
        (0.32, 1),
        (0.62, 2),
        (0.61, 2),
        (0.35, 1),
        (0.0, 0),
        (1.13, 3),
        (0.28, 1),
    ]
    .iter()
    .enumerate()
    {
        assert_eq!(*size, sp.path_to(i).map_or(0, |path| path.len()));
        assert_relative_eq!(*weight, sp.dist_to(i));
    }
}

#[test]
fn acyclic_lp() {
    let i = TINY_EWDAG;
    let graph = create_graph(i);
    let sp = AcyclicLP::new(graph.as_ref(), 5).unwrap();
    for (i, (weight, size)) in vec![
        (2.44, 5), // (dist_to(i), path_to(i).len())
        (0.32, 1),
        (2.77, 6),
        (0.61, 2),
        (2.06, 4),
        (0.0, 0),
        (1.13, 3),
        (2.43, 5),
    ]
    .iter()
    .enumerate()
    {
        assert_eq!(*size, sp.path_to(i).map_or(0, |path| path.len()));
        assert_relative_eq!(*weight, sp.dist_to(i));
    }
}

#[test]
fn cpm() {
    use std::convert::TryFrom;
    let i = JOBS_PC;
    let cpm = CPM::try_from(i).unwrap();
    assert_eq!(173.0, cpm.finish_time());

    /*
     job   start  finish
    --------------------
       0     0.0    41.0
       1    41.0    92.0
       2   123.0   173.0
       3    91.0   127.0
       4    70.0   108.0
       5     0.0    45.0
       6    70.0    91.0
       7    41.0    73.0
       8    91.0   123.0
       9    41.0    70.0
    Finish time:   173.0
    */
    println!(" job   start  finish");
    println!("--------------------");
    let n = cpm.len();
    for i in 0..n {
        println!("{:>4} {:7.1} {:7.1}", i, cpm.dist_to(i), cpm.dist_to(i + n));
    }
    println!("Finish time: {:7.1}", cpm.finish_time());
}

#[allow(non_snake_case)]
#[test]
fn Bellman_Ford_sp() {
    let i = TINY_EWD_NEGATIVE;
    let g = create_graph(i);
    let sp = BellmanFordSP::new(g.as_ref(), 0);
    assert!(!sp.has_negative_cycle());
    assert!(sp.negative_cycle().is_none());
    assert!(sp.check(g.as_ref(), 0).is_ok());

    // negative cycle
    let i = TINY_EWD_NEGATIVE_CYCLE;
    let g = create_graph(i);
    let sp = BellmanFordSP::new(g.as_ref(), 0);
    assert!(sp.has_negative_cycle());
    assert!(sp.negative_cycle().is_some());
    assert!(sp.check(g.as_ref(), 0).is_ok());
}

#[test]
fn arbitrage() {
    use std::convert::TryFrom;
    let i = RATES;
    let arbitrage = Arbitrage::try_from(i).unwrap();

    /*
    1000.00000 USD =  741.00000 EUR
     741.00000 EUR = 1012.20605 CAD
    1012.20605 CAD = 1007.14502 USD
         */
    let mut stake = 1000.0;
    assert_relative_eq!(1007.14497, arbitrage.calc(stake).unwrap());
    let cycle = arbitrage.opportunity_cycle().unwrap();
    for it in cycle {
        println!("{:10.5} {} = {:10.5} {}", stake, it.0, stake * it.2, it.1);
        stake *= it.2;
    }
}

fn create_graph(i: &str) -> Box<dyn IEWDigraph> {
    Box::new(EWDigraph::from_str(i).unwrap())
}
