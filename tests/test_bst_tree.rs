use algo::tree::binary::bst::BSTree;
use algo::tree::binary::traverse::PreOrderVisitor;
use algo::tree::binary::{bst, Tree};

#[test]
fn build_binary_search_tree() {
    let mut tree = Tree::default();
    let data = vec![4, 3, 8, 1, 7, 16, 2, 10, 9, 14];
    for v in &data {
        tree.insert(*v, *v);
    }
    let r = unsafe { PreOrderVisitor::iterate(&tree) };
    assert_eq!(r, vec![4, 3, 1, 2, 8, 7, 16, 10, 9, 14]);

    for v in data {
        assert!(tree.get(&v).is_some());
    }
    assert_eq!(tree.get(&100), None);
}

#[test]
fn binary_search_tree_min_max() {
    let mut tree = Tree::default();
    let data = vec![4, 3, 8, 1, 7, 16, 2, 10, 9, 14];
    for v in &data {
        tree.insert(*v, *v);
    }

    //min
    let v = tree.min();
    assert_eq!(v, Some(&1));

    //max
    let v = tree.max();
    assert_eq!(v, Some(&16));
}

#[test]
fn binary_search_tree_succ_pred() {
    let mut tree = Tree::default();
    let data = vec![4, 3, 8, 1, 7, 16, 2, 10, 9, 14];
    for v in &data {
        tree.insert(*v, *v);
    }

    //succ
    let v = tree.succ(&8);
    assert_eq!(v, Some(&9));
    let v = tree.succ(&2);
    assert_eq!(v, Some(&3));

    //pred
    let v = tree.pred(&9);
    assert_eq!(v, Some(&8));
    let v = tree.pred(&3);
    assert_eq!(v, Some(&2));
}

#[test]
fn delete_binary_search_tree() {
    let mut tree = Tree::default();
    for v in vec![4, 3, 8, 1, 7, 16, 2, 10, 9, 14] {
        tree.insert(v, v);
    }

    for (v, expect) in vec![
        (1, vec![4, 3, 2, 8, 7, 16, 10, 9, 14]),
        (8, vec![4, 3, 2, 9, 7, 16, 10, 14]),
        (4, vec![7, 3, 2, 9, 16, 10, 14]),
    ] {
        tree.delete(&v);
        let r = unsafe { PreOrderVisitor::iterate(&tree) };
        assert_eq!(r, expect);
    }
}

#[test]
fn bst_tree_height() {
    // 升序数列，使BST退化成一个linked list
    let mut tree = Tree::default();
    for v in 0..100 {
        tree.insert(v, v);
    }
    assert_eq!(tree.height(), 100);
}

#[test]
fn is_bst() {
    let mut tree = Tree::default();
    let data = vec![4, 3, 8, 1, 7, 16, 2, 10, 9, 14];
    for v in &data {
        tree.insert(*v, *v);
    }
    assert!(bst::is_bst(tree.root, None, None));
}

#[test]
fn delete_right_left_deviation() {
    // right-hand deviation
    let mut tree = Tree::default();
    for v in 0..100 {
        tree.insert(v, v);
    }
    let mut height = 100;
    for v in 0..100 {
        tree.delete(&v);
        height -= 1;
        assert_eq!(height, tree.height());
    }

    // left-hand deviation
    let mut tree = Tree::default();
    for v in (0..100).rev() {
        tree.insert(v, v);
    }
    let mut height = 100;
    for v in (0..100).rev() {
        tree.delete(&v);
        height -= 1;
        assert_eq!(height, tree.height());
    }
}

#[test]
fn drop_clear() {
    static mut DROPS: i32 = 0;
    struct Elem;
    impl Drop for Elem {
        fn drop(&mut self) {
            unsafe {
                DROPS += 1;
            }
        }
    }

    let mut tree = Tree::default();
    for v in 0..100 {
        tree.insert(v, Elem);
    }

    drop(tree);

    assert_eq!(unsafe { DROPS }, 100);
}

#[test]
fn drop_with_delete() {
    static mut DROPS: i32 = 0;
    struct Elem;
    impl Drop for Elem {
        fn drop(&mut self) {
            unsafe {
                DROPS += 1;
            }
        }
    }

    let mut tree = Tree::default();
    for v in 0..100 {
        tree.insert(v, Elem);
    }

    for v in 0..10 {
        tree.delete(&v);
    }
    assert_eq!(10, unsafe { DROPS });

    drop(tree);
    assert_eq!(100, unsafe { DROPS });
}
