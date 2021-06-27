use algo::tree::binary::traverse::PreOrderVisitor;
use algo::tree::binary::Tree;

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
fn bst_tree_height() {
    // 升序数列，使BST退化成一个linked list
    use algo::tree::binary::bst::BSTree;
    let mut tree = Tree::default();
    for v in 0..100 {
        tree.insert(v);
    }
    assert_eq!(tree.height(), 100);
}
