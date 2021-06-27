use algo::tree::binary::traverse::PreOrderVisitor;
use algo::tree::binary::Tree;

#[test]
fn rb_tree_height() {
    use algo::tree::binary::rb2::RedBlackTreeV2;
    let mut tree = Tree::default();
    for v in 0..100 {
        tree.insert(v);
    }
    // 即使输入升序数列，rb tree仍然使平衡的
    // 证明：含有n个节点的红黑树，其高度不会超过 2 * lg(n + 1)
    assert!(tree.height() as f32 <= 2.0 * 100.0f32.log2())
}

/// Algorithms 4th Edition by Robert Sedgewick, Kevin Wayne
/// P440
#[test]
fn rb_insert1() {
    use algo::tree::binary::rb2::RedBlackTreeV2;

    let mut tree = Tree::default();
    for v in vec!["S", "E", "A", "R", "C", "H", "X", "M", "P", "L"] {
        tree.insert(v);
    }

    let r = unsafe { PreOrderVisitor::iterate(&tree) };
    assert_eq!(r, vec!["M", "E", "C", "A", "L", "H", "R", "P", "X", "S"]);
}

#[test]
fn rb_insert2() {
    use algo::tree::binary::rb2::RedBlackTreeV2;

    let mut tree = Tree::default();
    for v in vec!["A", "C", "E", "H", "L", "M", "P", "R", "S", "X"] {
        tree.insert(v);
    }

    let r = unsafe { PreOrderVisitor::iterate(&tree) };
    assert_eq!(r, vec!["H", "C", "A", "E", "R", "M", "L", "P", "X", "S"]);
}
