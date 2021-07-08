use algo::graph::Graph;
use std::str::FromStr;

#[test]
fn parser() {
    //test strip
    assert_eq!("abc".strip_suffix("\n"), None);
    assert_eq!("abc\n".strip_suffix("\n"), Some("abc"));

    //test parser
    let s = include_str!("res/graph/tinyG.txt");
    let s = s.strip_suffix("\n").unwrap_or(s);
    let graph = Graph::from_str(s).unwrap();
    assert_eq!(graph.E(), 13);
    assert_eq!(graph.V(), 13);
}
