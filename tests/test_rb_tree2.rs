use algo::tree::binary::rb2::RedBlackTreeV2;
use algo::tree::binary::traverse::{InOrderVisitor, PreOrderVisitor};
use algo::tree::binary::Tree;

#[test]
fn height() {
    let mut tree = Tree::default();
    for v in 0..100 {
        tree.insert(v, v);
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
        tree.insert(v, v);
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
        tree.insert(v, v);
    }

    let r = unsafe { PreOrderVisitor::iterate(&tree) };
    assert_eq!(r.iter().collect::<String>(), "HCAERMLPXS");
    let r = unsafe { InOrderVisitor::iterate(&tree) };
    assert_eq!(r.iter().collect::<String>(), "ACEHLMPRSX");
}

#[test]
fn repeat_insert() {
    let mut tree = Tree::default();
    for v in "ACEHLMPRSX".chars() {
        tree.insert(v, v);
    }

    //重复insert 'A'
    tree.insert('A', 'A');

    let r = unsafe { PreOrderVisitor::iterate(&tree) };
    assert_eq!(r.iter().collect::<String>(), "HCAERMLPXS");
}

#[test]
fn delete_min() {
    let mut tree = Tree::default();
    for v in 0..10 {
        tree.insert(v, v);
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
        tree.insert(v, v);
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
        tree.insert(v, v);
    }
    for v in (0..10).rev() {
        assert!(tree.contains(&v));
        tree.delete(&v);
        assert!(!tree.contains(&v));
    }
}

#[test]
fn min_max() {
    let mut tree = Tree::default();
    for v in 0..10 {
        tree.insert(v, v);
    }
    assert_eq!(tree.min(), Some(&0));
    assert_eq!(tree.max(), Some(&9));
}

#[test]
fn keys() {
    let mut tree = Tree::default();
    for v in 0..10 {
        tree.insert(v, v);
    }
    assert_eq!(tree.keys(), vec![&0, &1, &2, &3, &4, &5, &6, &7, &8, &9]);

    // 倒序加入
    let mut tree = Tree::default();
    for v in (0..10).rev() {
        tree.insert(v, v);
    }
    assert_eq!(tree.keys(), vec![&0, &1, &2, &3, &4, &5, &6, &7, &8, &9]);
}

#[test]
fn drop_clear() {
    use algo::common::drop::{self, Elem};
    drop::with(|ctx| {
        let mut tree = Tree::default();
        for v in 0..100 {
            tree.insert(v, Elem);
        }
        drop(tree);
        assert_eq!(100, ctx.get());
    });
}

#[test]
fn drop_with_delete() {
    use algo::common::drop::{self, Elem};
    drop::with(|ctx| {
        let mut tree = Tree::default();
        for v in 0..100 {
            tree.insert(v, Elem);
        }

        for v in 0..10 {
            tree.delete(&v);
        }
        assert_eq!(10, ctx.get());

        drop(tree);
        assert_eq!(100, ctx.get());
    });
}
