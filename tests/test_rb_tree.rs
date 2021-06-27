use algo::tree::binary::Tree;

#[test]
fn rb_tree_height() {
    use algo::tree::binary::rb::RedBlackTree;
    let mut tree = Tree::default();
    for v in 0..100 {
        tree.insert(v);
    }
    // 即使输入升序数列，rb tree仍然使平衡的
    // 证明：含有n个节点的红黑树，其高度不会超过 2 * lg(n + 1)
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
