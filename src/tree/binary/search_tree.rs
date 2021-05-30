use crate::tree::binary::{Tree, TreeIndex, TreeNode};
use std::cmp::Ordering;

pub trait SearchTree<K>
where
    K: std::cmp::PartialOrd,
{
    /// return true: insert, false: not insert, exist k
    fn insert(&mut self, k: K) -> bool;
    /// return node index
    fn lookup(&self, x: K) -> Option<TreeIndex>;
    fn min(&self) -> Option<TreeIndex>;
    fn max(&self) -> Option<TreeIndex>;
    /// 查找后继元素
    fn succ(&self, x: K) -> Option<TreeIndex>;
    /// 寻找前驱元素
    fn pred(&self, x: K) -> Option<TreeIndex>;
}

impl<K> SearchTree<K> for Tree<K>
where
    K: std::cmp::PartialOrd,
{
    fn insert(&mut self, k: K) -> bool {
        do_insert(self, k, None, self.root)
    }

    fn lookup(&self, x: K) -> Option<TreeIndex> {
        do_lookup(self, x, self.root)
    }

    fn min(&self) -> Option<usize> {
        do_min(self, self.root)
    }

    fn max(&self) -> Option<usize> {
        do_max(self, self.root)
    }

    fn succ(&self, x: K) -> Option<usize> {
        todo!()
    }

    fn pred(&self, x: K) -> Option<usize> {
        todo!()
    }
}

fn do_insert<K>(
    tree: &mut Tree<K>,
    k: K,
    parent: Option<TreeIndex>,
    node: Option<TreeIndex>,
) -> bool
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
                None => false,
                Some(Ordering::Less) => {
                    let r = node.right;
                    do_insert(tree, k, Some(node_idx), r)
                }
                Some(Ordering::Equal) => false,
                Some(Ordering::Greater) => {
                    let l = node.left;
                    do_insert(tree, k, Some(node_idx), l)
                }
            }
        }
        (Some(parent), None) => {
            let node = tree.node_at(parent).unwrap();
            match node.key.partial_cmp(&k) {
                None => false,
                Some(Ordering::Less) => {
                    let child = TreeNode::from_key(k);
                    let child_idx = tree.add_node(child);
                    tree.node_at_mut(parent).unwrap().right = Some(child_idx);
                    true
                }
                Some(Ordering::Equal) => false,
                Some(Ordering::Greater) => {
                    let child = TreeNode::from_key(k);
                    let child_idx = tree.add_node(child);
                    tree.node_at_mut(parent).unwrap().left = Some(child_idx);
                    true
                }
            }
        }
    }
}

fn do_lookup<K>(tree: &Tree<K>, k: K, node: Option<TreeIndex>) -> Option<TreeIndex>
where
    K: std::cmp::PartialOrd,
{
    node.and_then(|idx| {
        let node = tree.node_at(idx).unwrap();
        match node.key.partial_cmp(&k) {
            None => None,
            Some(Ordering::Less) => do_lookup(tree, k, node.right),
            Some(Ordering::Equal) => Some(idx),
            Some(Ordering::Greater) => do_lookup(tree, k, node.left),
        }
    })
}

fn do_min<K>(tree: &Tree<K>, node_idx: Option<TreeIndex>) -> Option<TreeIndex>
where
    K: std::cmp::PartialOrd,
{
    node_idx.and_then(|idx| {
        let node = tree.node_at(idx).unwrap();
        node.left.map_or(Some(idx), |l| do_min(tree, Some(l)))
    })
}

fn do_max<K>(tree: &Tree<K>, node_idx: Option<TreeIndex>) -> Option<TreeIndex>
where
    K: std::cmp::PartialOrd,
{
    node_idx.and_then(|idx| {
        let node = tree.node_at(idx).unwrap();
        node.right.map_or(Some(idx), |r| do_max(tree, Some(r)))
    })
}
