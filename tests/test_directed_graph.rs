use algo::graph::directed::{
    Digraph, DirectedCycle, DirectedDFS, KosarajuSCC, Topological, TransitiveClosure,
};
use algo::graph::util::{BreadthFirstPaths, DepthFirstPaths, Paths, SymbolGraph};
use algo::graph::IGraph;
use std::str::FromStr;

const TINY_DG: &'static str = include_str!("res/graph/tinyDG.txt");
const JOBS: &'static str = include_str!("res/graph/jobs.txt");

#[test]
fn depth_first_paths() {
    let graph = create_digraph();
    let paths = DepthFirstPaths::new(&graph, 3);
    for (v, expect_path) in vec![
        Some(vec![3, 5, 4, 2, 0]),    // 0
        Some(vec![3, 5, 4, 2, 0, 1]), // 1
        Some(vec![3, 5, 4, 2]),       // 2
        Some(vec![3]),                // 3
        Some(vec![3, 5, 4]),          // 4
        Some(vec![3, 5]),             // 5
        None,                         // 6
        None,                         // 7
        None,                         // 8
        None,                         // 9
        None,                         // 10
        None,                         // 11
        None,                         // 12
    ]
    .iter()
    .enumerate()
    {
        assert_eq!(expect_path, &paths.path_to(v));
    }
}

#[test]
fn breadth_first_paths() {
    let graph = create_digraph();
    let paths = BreadthFirstPaths::new(&graph, 3);
    for (v, expect_path) in vec![
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
        assert_eq!(expect_path, &paths.path_to(v));
        let expect_dist = expect_path.as_ref().map_or(usize::MAX, |v| v.len() - 1);
        assert_eq!(expect_dist, paths.dist_to(v));
    }
}

#[test]
fn search() {
    let graph = create_digraph();
    let reach = DirectedDFS::new_single(&graph, 1);
    assert!(!reach.marked(0));
    assert!(reach.marked(1));
    let reach = DirectedDFS::new_multi(&graph, &vec![0, 1]);
    assert!(reach.marked(2));
    assert!(reach.marked(3));
}

#[test]
fn cycle() {
    let graph = create_digraph();
    let cycle = DirectedCycle::new(&graph);
    assert!(cycle.has_cycle());
}

#[test]
fn topological() {
    let graph = create_digraph();
    let cycle = Topological::new(&graph);
    assert!(!cycle.is_dag());

    let i = JOBS;
    let symbol_graph = SymbolGraph::new(i, "/", |nv| Box::new(Digraph::new(nv)));
    let graph = symbol_graph.G();
    let cycle = Topological::new(graph);
    assert!(cycle.is_dag());
}

#[test]
fn scc() {
    let graph = create_digraph();
    let scc = KosarajuSCC::new(&graph);
    assert_eq!(5, scc.count());
}

#[test]
fn transitive_closure() {
    let graph = create_digraph();
    let tc = TransitiveClosure::new(&graph);
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

fn create_digraph() -> Box<dyn IGraph> {
    let i = TINY_DG;
    Box::new(Digraph::from_str(i).unwrap())
}
