use algo::graph::directed::{
    DepthFirstOrders, Digraph, DirectedCycle, DirectedDFS, KosarajuSCC, Topological,
    TransitiveClosure,
};
use algo::graph::util::{BreadthFirstPaths, DepthFirstPaths, Paths, SymbolGraph};
use algo::graph::IGraph;
use std::str::FromStr;

const TINY_DG: &'static str = include_str!("res/graph/tinyDG.txt");
const TINY_DAG: &'static str = include_str!("res/graph/tinyDAG.txt");
const JOBS: &'static str = include_str!("res/graph/jobs.txt");

#[test]
fn parser() {
    //test parser
    let i = TINY_DG;
    let graph = Digraph::from_str(i).unwrap();
    assert_eq!(graph.V(), 13);
    assert_eq!(graph.E(), 22);
}

#[test]
fn dfs() {
    let graph = create_digraph(TINY_DG);
    let order = DepthFirstOrders::from(graph.as_ref());
    assert!(order.check().is_ok());
}

#[test]
fn dfs_paths() {
    let graph = create_digraph(TINY_DG);
    let paths = DepthFirstPaths::new(graph.as_ref(), 3);
    for (v, expect) in vec![
        Some(vec![3usize, 5, 4, 2, 0]), // 0
        Some(vec![3, 5, 4, 2, 0, 1]),   // 1
        Some(vec![3, 5, 4, 2]),         // 2
        Some(vec![3]),                  // 3
        Some(vec![3, 5, 4]),            // 4
        Some(vec![3, 5]),               // 5
        None,                           // 6
        None,                           // 7
        None,                           // 8
        None,                           // 9
        None,                           // 10
        None,                           // 11
        None,                           // 12
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
    let graph = create_digraph(TINY_DG);
    let paths = BreadthFirstPaths::new(graph.as_ref(), 3);
    for (v, expect) in vec![
        Some(vec![3, 2, 0]),    // 0
        Some(vec![3, 2, 0, 1]), // 1
        Some(vec![3, 2]),       // 2
        Some(vec![3]),          // 3
        Some(vec![3, 5, 4]),    // 4
        Some(vec![3, 5]),       // 5
        None,                   // 6
        None,                   // 7
        None,                   // 8
        None,                   // 9
        None,                   // 10
        None,                   // 11
        None,                   // 12
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
fn search() {
    let graph = create_digraph(TINY_DG);
    let reach = DirectedDFS::new_single(graph.as_ref(), 1);
    assert!(!reach.marked(0));
    assert!(reach.marked(1));
    let reach = DirectedDFS::new_multi(graph.as_ref(), &vec![0, 1]);
    assert!(reach.marked(2));
    assert!(reach.marked(3));
}

#[test]
fn cycle() {
    let graph = create_digraph(TINY_DG);
    let cycle = DirectedCycle::from(graph.as_ref());
    assert!(cycle.has_cycle());
    assert!(cycle.cycle().unwrap().eq(vec![3, 5, 4, 3].iter()));
    assert!(cycle.check().is_ok());

    //DAG
    let graph = create_digraph(TINY_DAG);
    let cycle = DirectedCycle::from(graph.as_ref());
    assert!(!cycle.has_cycle());
    assert!(cycle.cycle().is_none());
    assert!(cycle.check().is_ok());
}

#[test]
fn topological() {
    let graph = create_digraph(TINY_DG);
    let cycle = Topological::from(graph.as_ref());
    assert!(!cycle.has_order());

    let i = JOBS;
    let symbol_graph = SymbolGraph::new(i, "/", |nv| Box::new(Digraph::new(nv)));
    let graph = symbol_graph.G();
    let cycle = Topological::from(graph);
    assert!(cycle.has_order());
}

#[test]
fn scc() {
    let graph = create_digraph(TINY_DG);
    let scc = KosarajuSCC::new(graph.as_ref());
    assert_eq!(5, scc.count());
}

#[test]
fn transitive_closure() {
    let graph = create_digraph(TINY_DG);
    let tc = TransitiveClosure::new(graph.as_ref());
    let expect = vec![
        vec![1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0], // 0
        vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 1
        vec![1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0], // 2
        vec![1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0], // 3
        vec![1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0], // 4
        vec![1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0], // 5
        vec![1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1], // 6
        vec![1; 13],                                 // 7
        vec![1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1], // 8
        vec![1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1], // 9
        vec![1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1], // 10
        vec![1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1], // 11
        vec![1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1], // 12
    ];
    for v in 0..graph.V() {
        for w in 0..graph.V() {
            assert_eq!(
                expect[v][w],
                if tc.reachable(v, w) { 1 } else { 0 },
                "v = {}, w = {}, ",
                v,
                w
            );
        }
    }
}

fn create_digraph(i: &str) -> Box<dyn IGraph> {
    Box::new(Digraph::from_str(i).unwrap())
}
