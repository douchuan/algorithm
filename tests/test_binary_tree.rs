use algo::tree::binary::construct::new_tree;
use algo::tree::binary::traverse::{
    InOrderVisitor, LevelOrderVisitor, LevelOrderVisitor2, PostOrderVisitor, PreOrderVisitor,
    ZigzagOrderVisitor,
};

#[test]
fn tree_height() {
    let test_data = vec![
        (vec!["1", "#", "2", "3"], 3),
        (vec!["1", "2", "#", "3", "#", "#", "#", "4"], 4),
        (vec!["1", "2", "#", "3", "#", "#", "#", "#", "4"], 4),
        (vec!["1", "2", "#", "3", "4", "#", "#", "5"], 4),
    ];
    for (t, expect) in test_data {
        let tree = new_tree(&t);
        let r = tree.height();
        assert_eq!(
            expect, r,
            "tree = {:?}, expect = {:?}, r = {:?}",
            t, expect, r
        );
    }
}

#[test]
fn t_empty_tree() {
    let tree = new_tree(&[]);
    assert!(tree.arena.is_empty());
    assert!(tree.root.is_none());

    let tree = new_tree(&["#"]);
    assert!(tree.arena.is_empty());
    assert!(tree.root.is_none());
}

#[test]
fn t_tree_topology1() {
    let tokens = vec!["1", "#", "2", "3"];
    let tree = new_tree(&tokens);
    let p0 = tree.root.expect("invalid p0");
    let p0 = tree.node_at(p0).expect("invalid p0 node");
    assert_eq!(p0.value, 1);
    assert!(p0.left.is_none());
    //p1: '#'
    let p2 = p0.right.expect("invalid p0 right");
    let p2 = tree.node_at(p2).expect("invalid p2 node");
    assert_eq!(p2.value, 2);
    assert!(p2.right.is_none());
    let p3 = p2.left.expect("invalid p2 left");
    let p3 = tree.node_at(p3).expect("invalid p3 node");
    assert_eq!(p3.value, 3);
}

#[test]
fn t_tree_topology2() {
    let tokens = vec!["1", "2", "#", "3", "4", "#", "#", "5"];
    let tree = new_tree(&tokens);
    let p0 = tree.root.expect("invalid p0");
    let p0 = tree.node_at(p0).expect("invalid p0 node");
    assert_eq!(p0.value, 1);
    assert!(p0.right.is_none());
    let p1 = p0.left.expect("invalid p0 left");
    let p1 = tree.node_at(p1).expect("invalid p1 node");
    assert_eq!(p1.value, 2);
    //p2: '#'
    let p3 = p1.left.expect("invalid p3");
    let p3 = tree.node_at(p3).expect("invalid p3 node");
    assert_eq!(p3.value, 3);
    let p4 = p1.right.expect("invalid p4");
    let p4 = tree.node_at(p4).expect("invalid p3 node");
    assert_eq!(p4.value, 4);
    //p5: '#'
    //p6: '#'
    let p7 = p3.left.expect("invalid p7");
    let p7 = tree.node_at(p7).expect("invalid p7 node");
    assert_eq!(p7.value, 5);
    assert_eq!(tree.arena.len(), 5);
}

#[test]
fn t_preorder_iter() {
    for (t, expect) in preorder_test_data() {
        let tree = new_tree(&t);
        let r = PreOrderVisitor::iterate(&tree);
        assert_eq!(
            expect, r,
            "tree = {:?}, expect = {:?}, r = {:?}",
            t, expect, r
        );
    }
}

#[test]
fn t_preorder_morris() {
    for (t, expect) in preorder_test_data() {
        let mut tree = new_tree(&t);
        let r = PreOrderVisitor::morris(&mut tree);
        assert_eq!(
            expect, r,
            "tree = {:?}, expect = {:?}, r = {:?}",
            t, expect, r
        );
    }
}

#[test]
fn t_preorder_recursive() {
    for (t, expect) in preorder_test_data() {
        let mut tree = new_tree(&t);
        let r = PreOrderVisitor::recursive(&mut tree);
        assert_eq!(
            expect, r,
            "tree = {:?}, expect = {:?}, r = {:?}",
            t, expect, r
        );
    }
}

#[test]
fn t_inorder_iter() {
    for (t, expect) in inorder_test_data() {
        let tree = new_tree(&t);
        let r = InOrderVisitor::iterate(&tree);
        assert_eq!(
            expect, r,
            "tree = {:?}, expect = {:?}, r = {:?}",
            t, expect, r
        );
    }
}

#[test]
fn t_inorder_recursive() {
    for (t, expect) in inorder_test_data() {
        let tree = new_tree(&t);
        let r = InOrderVisitor::recursive(&tree);
        assert_eq!(
            expect, r,
            "tree = {:?}, expect = {:?}, r = {:?}",
            t, expect, r
        );
    }
}

#[test]
fn t_postorder_recursive() {
    let nodes = vec!["1", "#", "2", "3"];
    let tree = new_tree(&nodes);
    let r = PostOrderVisitor::recursive(&tree);
    assert_eq!(vec![3, 2, 1], r);
}

#[test]
fn t_postorder_iter() {
    let nodes = vec!["1", "#", "2", "3"];
    let tree = new_tree(&nodes);
    let r = PostOrderVisitor::iterate(&tree);
    assert_eq!(vec![3, 2, 1], r);
}

#[test]
fn t_levelorder_iter() {
    let nodes = vec!["3", "9", "20", "#", "#", "15", "7"];
    let tree = new_tree(&nodes);
    let r = LevelOrderVisitor::iterate(&tree);
    assert_eq!(vec![vec![3], vec![9, 20], vec![15, 7]], r);
}

#[test]
fn t_levelorder_traverse() {
    let nodes = vec!["3", "9", "20", "#", "#", "15", "7"];
    let tree = new_tree(&nodes);
    let r = LevelOrderVisitor::recursive(&tree);
    assert_eq!(vec![vec![3], vec![9, 20], vec![15, 7]], r);
}

#[test]
fn t_levelorder2_iter() {
    let nodes = vec!["3", "9", "20", "#", "#", "15", "7"];
    let tree = new_tree(&nodes);
    let r = LevelOrderVisitor2::iterate(&tree);
    assert_eq!(vec![vec![15, 7], vec![9, 20], vec![3]], r);
}

#[test]
fn t_levelorder2_traverse() {
    let nodes = vec!["3", "9", "20", "#", "#", "15", "7"];
    let tree = new_tree(&nodes);
    let r = LevelOrderVisitor2::recursive(&tree);
    assert_eq!(vec![vec![15, 7], vec![9, 20], vec![3]], r);
}

#[test]
fn t_levelorder_zigzag_iter() {
    let nodes = vec!["3", "9", "20", "#", "#", "15", "7"];
    let tree = new_tree(&nodes);
    let r = ZigzagOrderVisitor::iterate(&tree);
    assert_eq!(vec![vec![3], vec![20, 9], vec![15, 7]], r);
}

#[test]
fn t_levelorder_zigzag_traverse() {
    let nodes = vec!["3", "9", "20", "#", "#", "15", "7"];
    let tree = new_tree(&nodes);
    let r = ZigzagOrderVisitor::recursive(&tree);
    assert_eq!(vec![vec![3], vec![20, 9], vec![15, 7]], r);
}

fn preorder_test_data() -> Vec<(Vec<&'static str>, Vec<usize>)> {
    vec![
        (vec!["1", "#", "2", "3"], vec![1, 2, 3]),
        (
            vec!["1", "2", "#", "3", "#", "#", "#", "4"],
            vec![1, 2, 3, 4],
        ),
        (
            vec!["1", "2", "#", "3", "#", "#", "#", "#", "4"],
            vec![1, 2, 3, 4],
        ),
        (
            vec!["1", "2", "#", "3", "4", "#", "#", "5"],
            vec![1, 2, 3, 5, 4],
        ),
    ]
}

fn inorder_test_data() -> Vec<(Vec<&'static str>, Vec<usize>)> {
    vec![
        (vec!["1", "#", "2", "3"], vec![1, 3, 2]),
        (
            vec!["1", "2", "#", "3", "#", "#", "#", "4"],
            vec![4, 3, 2, 1],
        ),
        (
            vec!["1", "2", "#", "3", "#", "#", "#", "#", "4"],
            vec![3, 4, 2, 1],
        ),
        (
            vec!["1", "2", "#", "3", "4", "#", "#", "5"],
            vec![5, 3, 2, 4, 1],
        ),
    ]
}
