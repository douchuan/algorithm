use algo::tree::binary::rb2::RedBlackTreeV2;
use algo::tree::binary::traverse::{InOrderVisitor, PreOrderVisitor};
use algo::tree::binary::Tree;

#[test]
fn height() {
    let mut tree = Tree::default();
    for v in 0..100 {
        tree.insert(v);
    }
    // 即使输入升序数列，rb tree仍然是平衡的
    // 证明：含有n个节点的红黑树，其高度不会超过 2 * lg(n + 1)
    assert!(tree.height() as f32 <= 2.0 * 100.0f32.log2())
}

/// Algorithms 4th Edition by Robert Sedgewick, Kevin Wayne
/// P440
#[test]
fn insert1() {
    let mut tree = Tree::default();
    for v in "SEARCHXMPL".chars() {
        tree.insert(v);
    }

    let r = unsafe { PreOrderVisitor::iterate(&tree) };
    assert_eq!(r.iter().collect::<String>(), "MECALHRPXS");
    let r = unsafe { InOrderVisitor::iterate(&tree) };
    assert_eq!(r.iter().collect::<String>(), "ACEHLMPRSX");
}

#[test]
fn insert2() {
    let mut tree = Tree::default();
    for v in "ACEHLMPRSX".chars() {
        tree.insert(v);
    }

    let r = unsafe { PreOrderVisitor::iterate(&tree) };
    assert_eq!(r.iter().collect::<String>(), "HCAERMLPXS");
    let r = unsafe { InOrderVisitor::iterate(&tree) };
    assert_eq!(r.iter().collect::<String>(), "ACEHLMPRSX");
}

#[test]
fn repeat_insert() {
    use algo::tree::binary::rb2::RedBlackTreeV2;

    let mut tree = Tree::default();
    for v in "ACEHLMPRSX".chars() {
        tree.insert(v);
    }

    //重复insert 'A'
    tree.insert('A');

    let r = unsafe { PreOrderVisitor::iterate(&tree) };
    assert_eq!(r.iter().collect::<String>(), "HCAERMLPXS");
}

#[test]
fn delete_min() {
    let mut tree = Tree::default();
    for v in 0..10 {
        tree.insert(v);
    }
    for v in 0..10 {
        assert!(tree.contains(&v));
        tree.delete_min();
        assert!(!tree.contains(&v));
    }
}

#[test]
fn delete_max() {
    let mut tree = Tree::default();
    for v in 0..10 {
        tree.insert(v);
    }
    for v in (0..10).rev() {
        assert!(tree.contains(&v));
        tree.delete_max();
        assert!(!tree.contains(&v));
    }
}

#[test]
fn delete() {
    let mut tree = Tree::default();
    for v in 0..10 {
        tree.insert(v);
    }
    for v in (0..10).rev() {
        assert!(tree.contains(&v));
        tree.delete(v);
        assert!(!tree.contains(&v));
    }
}
