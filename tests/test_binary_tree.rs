use algo::tree::binary::bst::BSTree;
use algo::tree::binary::builder::level::BuildTreeInLevel;
use algo::tree::binary::builder::TreeBuilder;
use algo::tree::binary::traverse::{
    InOrderVisitor, LevelOrderVisitor, LevelOrderVisitor2, PostOrderVisitor, PreOrderVisitor,
    ZigzagOrderVisitor,
};
use algo::tree::binary::Tree2;

#[test]
fn tree_height() {
    let test_data = vec![
        (vec!["1", "#", "2", "3"], 3),
        (vec!["1", "2", "#", "3", "#", "#", "#", "4"], 4),
        (vec!["1", "2", "#", "3", "#", "#", "#", "#", "4"], 4),
        (vec!["1", "2", "#", "3", "4", "#", "#", "5"], 4),
    ];
    for (t, expect) in test_data {
        let tree: Tree2<usize> = TreeBuilder::build_in_level(t.as_slice());
        let r = tree.height();
        assert_eq!(
            expect, r,
            "tree = {:?}, expect = {:?}, r = {:?}",
            t, expect, r
        );
    }
}

#[test]
fn empty_tree() {
    let vec = vec![];
    let tree: Tree2<usize> = TreeBuilder::build_in_level(vec.as_slice());
    assert!(tree.root.is_none());

    let vec = vec!["#"];
    let tree: Tree2<usize> = TreeBuilder::build_in_level(vec.as_slice());
    assert!(tree.root.is_none());

    let r = unsafe { PreOrderVisitor::iterate(&tree) };
    assert!(r.is_empty());
}

#[test]
fn tree_topology1() {
    let tokens = vec!["1", "#", "2", "3"];
    let tree: Tree2<usize> = TreeBuilder::build_in_level(tokens.as_slice());
    let r = unsafe { PreOrderVisitor::iterate(&tree) };
    assert_eq!(r, vec![1, 2, 3]);
}

#[test]
fn tree_topology2() {
    let tokens = vec!["1", "2", "#", "3", "4", "#", "#", "5"];
    let tree: Tree2<usize> = TreeBuilder::build_in_level(tokens.as_slice());
    let r = unsafe { PreOrderVisitor::iterate(&tree) };
    assert_eq!(r, vec![1, 2, 3, 5, 4]);
}

#[test]
fn preorder_iter() {
    for (t, expect) in preorder_test_data() {
        let tree: Tree2<usize> = TreeBuilder::build_in_level(t.as_slice());
        let r = unsafe { PreOrderVisitor::iterate(&tree) };
        assert_eq!(
            expect, r,
            "tree = {:?}, expect = {:?}, r = {:?}",
            t, expect, r
        );
    }
}

#[test]
fn preorder_morris() {
    for (t, expect) in preorder_test_data() {
        let mut tree: Tree2<usize> = TreeBuilder::build_in_level(t.as_slice());
        let r = unsafe { PreOrderVisitor::morris(&mut tree) };
        assert_eq!(
            expect, r,
            "tree = {:?}, expect = {:?}, r = {:?}",
            t, expect, r
        );
    }
}

#[test]
fn preorder_recursive() {
    for (t, expect) in preorder_test_data() {
        let mut tree: Tree2<usize> = TreeBuilder::build_in_level(t.as_slice());
        let r = unsafe { PreOrderVisitor::recursive(&mut tree) };
        assert_eq!(
            expect, r,
            "tree = {:?}, expect = {:?}, r = {:?}",
            t, expect, r
        );
    }
}

#[test]
fn inorder_iter() {
    for (t, expect) in inorder_test_data() {
        let tree: Tree2<usize> = TreeBuilder::build_in_level(t.as_slice());
        let r = unsafe { InOrderVisitor::iterate(&tree) };
        assert_eq!(
            expect, r,
            "tree = {:?}, expect = {:?}, r = {:?}",
            t, expect, r
        );
    }
}

#[test]
fn inorder_recursive() {
    for (t, expect) in inorder_test_data() {
        let tree: Tree2<usize> = TreeBuilder::build_in_level(t.as_slice());
        let r = unsafe { InOrderVisitor::recursive(&tree) };
        assert_eq!(
            expect, r,
            "tree = {:?}, expect = {:?}, r = {:?}",
            t, expect, r
        );
    }
}

#[test]
fn postorder_recursive() {
    let nodes = vec!["1", "#", "2", "3"];
    let tree: Tree2<usize> = TreeBuilder::build_in_level(nodes.as_slice());
    let r = unsafe { PostOrderVisitor::recursive(&tree) };
    assert_eq!(vec![3, 2, 1], r);
}

#[test]
fn postorder_iter() {
    let nodes = vec!["1", "#", "2", "3"];
    let tree: Tree2<usize> = TreeBuilder::build_in_level(nodes.as_slice());
    let r = unsafe { PostOrderVisitor::iterate(&tree) };
    assert_eq!(vec![3, 2, 1], r);
}

#[test]
fn levelorder_iter() {
    let nodes = vec!["3", "9", "20", "#", "#", "15", "7"];
    let tree: Tree2<usize> = TreeBuilder::build_in_level(nodes.as_slice());
    let r = unsafe { LevelOrderVisitor::iterate(&tree) };
    assert_eq!(vec![vec![3], vec![9, 20], vec![15, 7]], r);
}

#[test]
fn levelorder_traverse() {
    let nodes = vec!["3", "9", "20", "#", "#", "15", "7"];
    let tree: Tree2<usize> = TreeBuilder::build_in_level(nodes.as_slice());
    let r = unsafe { LevelOrderVisitor::recursive(&tree) };
    assert_eq!(vec![vec![3], vec![9, 20], vec![15, 7]], r);
}

#[test]
fn levelorder2_iter() {
    let nodes = vec!["3", "9", "20", "#", "#", "15", "7"];
    let tree: Tree2<usize> = TreeBuilder::build_in_level(nodes.as_slice());
    let r = unsafe { LevelOrderVisitor2::iterate(&tree) };
    assert_eq!(vec![vec![15, 7], vec![9, 20], vec![3]], r);
}

#[test]
fn levelorder2_traverse() {
    let nodes = vec!["3", "9", "20", "#", "#", "15", "7"];
    let tree: Tree2<usize> = TreeBuilder::build_in_level(nodes.as_slice());
    let r = unsafe { LevelOrderVisitor2::recursive(&tree) };
    assert_eq!(vec![vec![15, 7], vec![9, 20], vec![3]], r);
}

#[test]
fn levelorder_zigzag_iter() {
    let nodes = vec!["3", "9", "20", "#", "#", "15", "7"];
    let tree: Tree2<usize> = TreeBuilder::build_in_level(nodes.as_slice());
    let r = unsafe { ZigzagOrderVisitor::iterate(&tree) };
    assert_eq!(vec![vec![3], vec![20, 9], vec![15, 7]], r);
}

#[test]
fn levelorder_zigzag_traverse() {
    let nodes = vec!["3", "9", "20", "#", "#", "15", "7"];
    let tree: Tree2<usize> = TreeBuilder::build_in_level(nodes.as_slice());
    let r = unsafe { ZigzagOrderVisitor::recursive(&tree) };
    assert_eq!(vec![vec![3], vec![20, 9], vec![15, 7]], r);
}

#[test]
fn build_binary_search_tree() {
    let mut tree = Tree2::default();
    let data = vec![4, 3, 8, 1, 7, 16, 2, 10, 9, 14];
    for v in &data {
        tree.insert(*v);
    }
    let r = unsafe { PreOrderVisitor::iterate(&tree) };
    assert_eq!(r, vec![4, 3, 1, 2, 8, 7, 16, 10, 9, 14]);

    for v in data {
        assert!(tree.find(v).is_some());
    }
    assert_eq!(tree.find(100), None);
}

#[test]
fn binary_search_tree_min_max() {
    let mut tree = Tree2::default();
    let data = vec![4, 3, 8, 1, 7, 16, 2, 10, 9, 14];
    for v in &data {
        tree.insert(*v);
    }

    //min
    let node = tree.min().unwrap();
    let element = unsafe { (*node.as_ptr()).element };
    assert_eq!(element, 1);

    //max
    let node = tree.max().unwrap();
    let element = unsafe { (*node.as_ptr()).element };
    assert_eq!(element, 16);
}

#[test]
fn binary_search_tree_succ_pred() {
    let mut tree = Tree2::default();
    let data = vec![4, 3, 8, 1, 7, 16, 2, 10, 9, 14];
    for v in &data {
        tree.insert(*v);
    }

    //succ
    let node = tree.succ(8).unwrap();
    let element = unsafe { (*node.as_ptr()).element };
    assert_eq!(element, 9);
    let node = tree.succ(2).unwrap();
    let element = unsafe { (*node.as_ptr()).element };
    assert_eq!(element, 3);

    //pred
    let node = tree.pred(9).unwrap();
    let element = unsafe { (*node.as_ptr()).element };
    assert_eq!(element, 8);
    let node = tree.pred(3).unwrap();
    let element = unsafe { (*node.as_ptr()).element };
    assert_eq!(element, 2);
}

#[test]
fn delete_binary_search_tree() {
    let mut tree = Tree2::default();
    for v in vec![4, 3, 8, 1, 7, 16, 2, 10, 9, 14] {
        tree.insert(v);
    }

    for (v, expect) in vec![
        (1, vec![4, 3, 2, 8, 7, 16, 10, 9, 14]),
        (8, vec![4, 3, 2, 9, 7, 16, 10, 14]),
        (4, vec![7, 3, 2, 9, 16, 10, 14]),
    ] {
        tree.delete(v);
        let r = unsafe { PreOrderVisitor::iterate(&tree) };
        assert_eq!(r, expect);
    }
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
