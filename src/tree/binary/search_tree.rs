use crate::tree::binary::{Tree, TreeIndex, TreeNode};
use std::cmp::Ordering;

pub trait SearchTree<K>
where
    K: std::cmp::PartialOrd,
{
    /// return true: insert, false: not insert, exist k
    fn insert(&mut self, k: K) -> bool;
    /// return node index
    fn find(&self, x: K) -> Option<TreeIndex>;
    fn min(&self) -> Option<TreeIndex>;
    fn max(&self) -> Option<TreeIndex>;
    /// 查找后继元素
    fn succ(&self, x: K) -> Option<TreeIndex>;
    /// 寻找前驱元素
    fn pred(&self, x: K) -> Option<TreeIndex>;
}

impl<K> SearchTree<K> for Tree<K>
where
    K: std::cmp::PartialOrd + Copy,
{
    fn insert(&mut self, k: K) -> bool {
        insert(self, k, None, self.root)
    }

    fn find(&self, x: K) -> Option<TreeIndex> {
        find(self, x, self.root)
    }

    fn min(&self) -> Option<TreeIndex> {
        find_min(self, self.root)
    }

    fn max(&self) -> Option<TreeIndex> {
        find_max(self, self.root)
    }

    fn succ(&self, x: K) -> Option<TreeIndex> {
        succ(self, x)
    }

    fn pred(&self, x: K) -> Option<TreeIndex> {
        pred(self, x)
    }
}

fn insert<K>(tree: &mut Tree<K>, k: K, parent: Option<TreeIndex>, node: Option<TreeIndex>) -> bool
where
    K: std::cmp::PartialOrd,
{
    match (parent, node) {
        //empty tree
        (None, None) => {
            let node = TreeNode::from_key(k);
            let idx = tree.add_node(node);
            tree.root = Some(idx);
            true
        }
        (_, Some(node_idx)) => {
            let node = tree.node_at(node_idx).unwrap();
            match node.key.partial_cmp(&k) {
                Some(Ordering::Less) => {
                    let r = node.right;
                    insert(tree, k, Some(node_idx), r)
                }
                Some(Ordering::Greater) => {
                    let l = node.left;
                    insert(tree, k, Some(node_idx), l)
                }
                _ => false,
            }
        }
        (Some(parent), None) => {
            let node = tree.node_at(parent).unwrap();
            match node.key.partial_cmp(&k) {
                Some(Ordering::Less) => {
                    let child = TreeNode::new(k, None, None, Some(parent));
                    let child = tree.add_node(child);
                    tree.node_at_mut(parent).unwrap().right = Some(child);
                    true
                }
                Some(Ordering::Greater) => {
                    let child = TreeNode::new(k, None, None, Some(parent));
                    let child = tree.add_node(child);
                    tree.node_at_mut(parent).unwrap().left = Some(child);
                    true
                }
                _ => false,
            }
        }
    }
}

fn find<K>(tree: &Tree<K>, k: K, idx: Option<TreeIndex>) -> Option<TreeIndex>
where
    K: std::cmp::PartialOrd,
{
    idx.and_then(|idx| {
        let node = tree.node_at(idx).unwrap();
        match node.key.partial_cmp(&k) {
            Some(Ordering::Less) => find(tree, k, node.right),
            Some(Ordering::Greater) => find(tree, k, node.left),
            Some(Ordering::Equal) => Some(idx),
            None => None,
        }
    })
}

fn find_min<K>(tree: &Tree<K>, node_idx: Option<TreeIndex>) -> Option<TreeIndex>
where
    K: std::cmp::PartialOrd,
{
    node_idx.and_then(|idx| {
        let node = tree.node_at(idx).unwrap();
        node.left.map_or(Some(idx), |l| find_min(tree, Some(l)))
    })
}

fn find_max<K>(tree: &Tree<K>, node_idx: Option<TreeIndex>) -> Option<TreeIndex>
where
    K: std::cmp::PartialOrd,
{
    node_idx.and_then(|idx| {
        let node = tree.node_at(idx).unwrap();
        node.right.map_or(Some(idx), |r| find_max(tree, Some(r)))
    })
}

fn succ<K>(tree: &Tree<K>, mut k: K) -> Option<TreeIndex>
where
    K: std::cmp::PartialOrd + Copy,
{
    find(tree, k, tree.root).and_then(|idx| {
        let node = tree.node_at(idx).unwrap();
        match node.right {
            //右分支的最小值
            Some(r) => find_min(tree, Some(r)),
            None => {
                //右分支为空，向上找
                let mut p = node.parent;
                loop {
                    match tree.right_node_at(p) {
                        Some(r) if r.key == k => {
                            let p_node = tree.node_at(p.unwrap()).unwrap();
                            k = p_node.key;
                            p = p_node.parent;
                        }
                        _ => break,
                    }
                }
                p
            }
        }
    })
}

fn pred<K>(tree: &Tree<K>, mut k: K) -> Option<TreeIndex>
where
    K: std::cmp::PartialOrd + Copy,
{
    find(tree, k, tree.root).and_then(|idx| {
        let node = tree.node_at(idx).unwrap();
        match node.left {
            //左分支的最大值
            Some(l) => find_max(tree, Some(l)),
            None => {
                //左分支为空，向上找
                let mut p = node.parent;
                loop {
                    match tree.left_node_at(p) {
                        Some(r) if r.key == k => {
                            let p_node = tree.node_at(p.unwrap()).unwrap();
                            k = p_node.key;
                            p = p_node.parent;
                        }
                        _ => break,
                    }
                }
                p
            }
        }
    })
}
