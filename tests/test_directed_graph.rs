use algo::graph::directed::{Digraph, DirectedDFS};
use std::str::FromStr;

const TINY_DG: &'static str = include_str!("res/graph/tinyDG.txt");

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
