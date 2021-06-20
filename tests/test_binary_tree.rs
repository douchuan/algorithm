use algo::tree::binary::builder::level::BuildTreeInLevel;
use algo::tree::binary::builder::TreeBuilder;
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
fn empty_tree() {
    let vec = vec![];
    let tree: Tree<usize> = TreeBuilder::build_in_level(vec.as_slice());
    assert!(tree.root.is_none());

    let vec = vec!["#"];
    let tree: Tree<usize> = TreeBuilder::build_in_level(vec.as_slice());
    assert!(tree.root.is_none());

    let r = unsafe { PreOrderVisitor::iterate(&tree) };
    assert!(r.is_empty());
}

#[test]
fn tree_topology1() {
    let tokens = vec!["1", "#", "2", "3"];
    let tree: Tree<usize> = TreeBuilder::build_in_level(tokens.as_slice());
    let r = unsafe { PreOrderVisitor::iterate(&tree) };
    assert_eq!(r, vec![1, 2, 3]);
}

#[test]
fn tree_topology2() {
    let tokens = vec!["1", "2", "#", "3", "4", "#", "#", "5"];
    let tree: Tree<usize> = TreeBuilder::build_in_level(tokens.as_slice());
    let r = unsafe { PreOrderVisitor::iterate(&tree) };
    assert_eq!(r, vec![1, 2, 3, 5, 4]);
}

#[test]
fn preorder_iter() {
    for (t, expect) in preorder_test_data() {
        let tree: Tree<usize> = TreeBuilder::build_in_level(t.as_slice());
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
        let mut tree: Tree<usize> = TreeBuilder::build_in_level(t.as_slice());
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
        let mut tree: Tree<usize> = TreeBuilder::build_in_level(t.as_slice());
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
        let tree: Tree<usize> = TreeBuilder::build_in_level(t.as_slice());
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
        let tree: Tree<usize> = TreeBuilder::build_in_level(t.as_slice());
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
    let tree: Tree<usize> = TreeBuilder::build_in_level(nodes.as_slice());
    let r = unsafe { PostOrderVisitor::recursive(&tree) };
    assert_eq!(vec![3, 2, 1], r);
}

#[test]
fn postorder_iter() {
    let nodes = vec!["1", "#", "2", "3"];
    let tree: Tree<usize> = TreeBuilder::build_in_level(nodes.as_slice());
    let r = unsafe { PostOrderVisitor::iterate(&tree) };
    assert_eq!(vec![3, 2, 1], r);
}

#[test]
fn levelorder_iter() {
    let nodes = vec!["3", "9", "20", "#", "#", "15", "7"];
    let tree: Tree<usize> = TreeBuilder::build_in_level(nodes.as_slice());
    let r = unsafe { LevelOrderVisitor::iterate(&tree) };
    assert_eq!(vec![vec![3], vec![9, 20], vec![15, 7]], r);
}

#[test]
fn levelorder_traverse() {
    let nodes = vec!["3", "9", "20", "#", "#", "15", "7"];
    let tree: Tree<usize> = TreeBuilder::build_in_level(nodes.as_slice());
    let r = unsafe { LevelOrderVisitor::recursive(&tree) };
    assert_eq!(vec![vec![3], vec![9, 20], vec![15, 7]], r);
}

#[test]
fn levelorder2_iter() {
    let nodes = vec!["3", "9", "20", "#", "#", "15", "7"];
    let tree: Tree<usize> = TreeBuilder::build_in_level(nodes.as_slice());
    let r = unsafe { LevelOrderVisitor2::iterate(&tree) };
    assert_eq!(vec![vec![15, 7], vec![9, 20], vec![3]], r);
}

#[test]
fn levelorder2_traverse() {
    let nodes = vec!["3", "9", "20", "#", "#", "15", "7"];
    let tree: Tree<usize> = TreeBuilder::build_in_level(nodes.as_slice());
    let r = unsafe { LevelOrderVisitor2::recursive(&tree) };
    assert_eq!(vec![vec![15, 7], vec![9, 20], vec![3]], r);
}

#[test]
fn levelorder_zigzag_iter() {
    let nodes = vec!["3", "9", "20", "#", "#", "15", "7"];
    let tree: Tree<usize> = TreeBuilder::build_in_level(nodes.as_slice());
    let r = unsafe { ZigzagOrderVisitor::iterate(&tree) };
    assert_eq!(vec![vec![3], vec![20, 9], vec![15, 7]], r);
}

#[test]
fn levelorder_zigzag_traverse() {
    let nodes = vec!["3", "9", "20", "#", "#", "15", "7"];
    let tree: Tree<usize> = TreeBuilder::build_in_level(nodes.as_slice());
    let r = unsafe { ZigzagOrderVisitor::recursive(&tree) };
    assert_eq!(vec![vec![3], vec![20, 9], vec![15, 7]], r);
}

#[test]
fn build_binary_search_tree() {
    use algo::tree::binary::bst::BSTree;
    let mut tree = Tree::default();
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
    use algo::tree::binary::bst::BSTree;
    let mut tree = Tree::default();
    let data = vec![4, 3, 8, 1, 7, 16, 2, 10, 9, 14];
    for v in &data {
        tree.insert(*v);
    }

    //min
    let v = tree.min();
    assert_eq!(v, Some(1));

    //max
    let v = tree.max();
    assert_eq!(v, Some(16));
}

#[test]
fn binary_search_tree_succ_pred() {
    use algo::tree::binary::bst::BSTree;
    let mut tree = Tree::default();
    let data = vec![4, 3, 8, 1, 7, 16, 2, 10, 9, 14];
    for v in &data {
        tree.insert(*v);
    }

    //succ
    let v = tree.succ(8);
    assert_eq!(v, Some(9));
    let v = tree.succ(2);
    assert_eq!(v, Some(3));

    //pred
    let v = tree.pred(9);
    assert_eq!(v, Some(8));
    let v = tree.pred(3);
    assert_eq!(v, Some(2));
}

#[test]
fn delete_binary_search_tree() {
    use algo::tree::binary::bst::BSTree;
    let mut tree = Tree::default();
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

#[test]
fn rb_tree_height() {
    use algo::tree::binary::rb::RedBlackTree;
    let mut tree = Tree::default();
    for v in 0..100 {
        tree.insert(v);
    }
    assert!(tree.height() as f32 <= 2.0 * 100.0f32.log2())
}

#[test]
fn rb_rotate_left() {
    use algo::tree::binary::bst::BSTree;
    use algo::tree::binary::rb::rotate_left;
    use algo::tree::binary::traverse::{InOrderVisitor, PreOrderVisitor};
    use algo::tree::binary::Tree;

    /*
         10
        /  \
       5   15
         /    \
        14    16
    */
    let mut tree = Tree::default();
    for v in vec![10, 5, 15, 14, 16] {
        tree.insert(v);
    }

    /*
        15
       /  \
      10  16
     /  \
    5   14
     */
    unsafe {
        tree.root = rotate_left(tree.root, tree.root.unwrap());
        assert_eq!(PreOrderVisitor::recursive(&tree), vec![15, 10, 5, 14, 16]);
        assert_eq!(InOrderVisitor::recursive(&tree), vec![5, 10, 14, 15, 16]);
    }
}

#[test]
fn rb_rotate_right() {
    use algo::tree::binary::bst::BSTree;
    use algo::tree::binary::rb::{rotate_left, rotate_right};
    use algo::tree::binary::traverse::{InOrderVisitor, PreOrderVisitor};
    use algo::tree::binary::Tree;

    let mut tree = Tree::default();
    for v in vec![10, 5, 15, 14, 16] {
        tree.insert(v);
    }

    unsafe {
        tree.root = rotate_left(tree.root, tree.root.unwrap());
        tree.root = rotate_right(tree.root, tree.root.unwrap());
        assert_eq!(PreOrderVisitor::recursive(&tree), vec![10, 5, 15, 14, 16]);
        assert_eq!(InOrderVisitor::recursive(&tree), vec![5, 10, 14, 15, 16]);
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
