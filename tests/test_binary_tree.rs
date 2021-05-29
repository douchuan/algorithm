use algo::tree::binary::builder::level::BuildTreeInLevel;
use algo::tree::binary::builder::TreeBuilder;
use algo::tree::binary::search_tree::SearchTree;
use algo::tree::binary::traverse::{
    InOrderVisitor, LevelOrderVisitor, LevelOrderVisitor2, PostOrderVisitor, PreOrderVisitor,
    ZigzagOrderVisitor,
};
use algo::tree::binary::Tree;

#[test]
fn tree_height() {
    let test_data = vec![
        (vec!["1", "#", "2", "3"], 3),
        (vec!["1", "2", "#", "3", "#", "#", "#", "4"], 4),
        (vec!["1", "2", "#", "3", "#", "#", "#", "#", "4"], 4),
        (vec!["1", "2", "#", "3", "4", "#", "#", "5"], 4),
    ];
    for (t, expect) in test_data {
        let tree: Tree<usize> = TreeBuilder::build_in_level(t.as_slice());
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
    let vec = vec![];
    let tree: Tree<usize> = TreeBuilder::build_in_level(vec.as_slice());
    assert!(tree.arena.is_empty());
    assert!(tree.root.is_none());

    let vec = vec!["#"];
    let tree: Tree<usize> = TreeBuilder::build_in_level(vec.as_slice());
    assert!(tree.arena.is_empty());
    assert!(tree.root.is_none());

    let r = PreOrderVisitor::iterate(&tree);
    assert!(r.is_empty());
}

#[test]
fn t_tree_topology1() {
    let tokens = vec!["1", "#", "2", "3"];
    let tree: Tree<usize> = TreeBuilder::build_in_level(tokens.as_slice());
    let r = PreOrderVisitor::iterate(&tree);
    assert_eq!(r, vec![1, 2, 3]);
}

#[test]
fn t_tree_topology2() {
    let tokens = vec!["1", "2", "#", "3", "4", "#", "#", "5"];
    let tree: Tree<usize> = TreeBuilder::build_in_level(tokens.as_slice());
    let r = PreOrderVisitor::iterate(&tree);
    assert_eq!(r, vec![1, 2, 3, 5, 4]);
}

#[test]
fn t_preorder_iter() {
    for (t, expect) in preorder_test_data() {
        let tree: Tree<usize> = TreeBuilder::build_in_level(t.as_slice());
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
        let mut tree: Tree<usize> = TreeBuilder::build_in_level(t.as_slice());
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
        let mut tree: Tree<usize> = TreeBuilder::build_in_level(t.as_slice());
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
        let tree: Tree<usize> = TreeBuilder::build_in_level(t.as_slice());
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
        let tree: Tree<usize> = TreeBuilder::build_in_level(t.as_slice());
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
    let tree: Tree<usize> = TreeBuilder::build_in_level(nodes.as_slice());
    let r = PostOrderVisitor::recursive(&tree);
    assert_eq!(vec![3, 2, 1], r);
}

#[test]
fn t_postorder_iter() {
    let nodes = vec!["1", "#", "2", "3"];
    let tree: Tree<usize> = TreeBuilder::build_in_level(nodes.as_slice());
    let r = PostOrderVisitor::iterate(&tree);
    assert_eq!(vec![3, 2, 1], r);
}

#[test]
fn t_levelorder_iter() {
    let nodes = vec!["3", "9", "20", "#", "#", "15", "7"];
    let tree: Tree<usize> = TreeBuilder::build_in_level(nodes.as_slice());
    let r = LevelOrderVisitor::iterate(&tree);
    assert_eq!(vec![vec![3], vec![9, 20], vec![15, 7]], r);
}

#[test]
fn t_levelorder_traverse() {
    let nodes = vec!["3", "9", "20", "#", "#", "15", "7"];
    let tree: Tree<usize> = TreeBuilder::build_in_level(nodes.as_slice());
    let r = LevelOrderVisitor::recursive(&tree);
    assert_eq!(vec![vec![3], vec![9, 20], vec![15, 7]], r);
}

#[test]
fn t_levelorder2_iter() {
    let nodes = vec!["3", "9", "20", "#", "#", "15", "7"];
    let tree: Tree<usize> = TreeBuilder::build_in_level(nodes.as_slice());
    let r = LevelOrderVisitor2::iterate(&tree);
    assert_eq!(vec![vec![15, 7], vec![9, 20], vec![3]], r);
}

#[test]
fn t_levelorder2_traverse() {
    let nodes = vec!["3", "9", "20", "#", "#", "15", "7"];
    let tree: Tree<usize> = TreeBuilder::build_in_level(nodes.as_slice());
    let r = LevelOrderVisitor2::recursive(&tree);
    assert_eq!(vec![vec![15, 7], vec![9, 20], vec![3]], r);
}

#[test]
fn t_levelorder_zigzag_iter() {
    let nodes = vec!["3", "9", "20", "#", "#", "15", "7"];
    let tree: Tree<usize> = TreeBuilder::build_in_level(nodes.as_slice());
    let r = ZigzagOrderVisitor::iterate(&tree);
    assert_eq!(vec![vec![3], vec![20, 9], vec![15, 7]], r);
}

#[test]
fn t_levelorder_zigzag_traverse() {
    let nodes = vec!["3", "9", "20", "#", "#", "15", "7"];
    let tree: Tree<usize> = TreeBuilder::build_in_level(nodes.as_slice());
    let r = ZigzagOrderVisitor::recursive(&tree);
    assert_eq!(vec![vec![3], vec![20, 9], vec![15, 7]], r);
}

#[test]
fn build_binary_search_tree() {
    let mut tree = Tree::new();
    let data = vec![4, 3, 8, 1, 7, 16, 2, 10, 9, 14];
    for v in &data {
        tree.insert(*v);
    }
    let r = PreOrderVisitor::iterate(&tree);
    assert_eq!(r, vec![4, 3, 1, 2, 8, 7, 16, 10, 9, 14]);

    for (idx, v) in data.iter().enumerate() {
        assert_eq!(tree.lookup(*v), Some(idx));
    }
    assert_eq!(tree.lookup(100), None);
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
