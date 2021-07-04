use algo::tree::binary::Tree;

#[test]
fn rb_tree_height() {
    use algo::tree::binary::rb::RedBlackTree;
    let mut tree = Tree::default();
    for v in 0..100 {
        tree.insert(v, v);
    }
    // 即使输入升序数列，rb tree仍然是平衡的
    // 证明：含有n个节点的红黑树，其高度不会超过 2 * lg(n + 1)
    assert!(tree.height() as f32 <= 2.0 * 100.0f32.log2())
}
