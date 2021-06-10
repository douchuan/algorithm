//! 一棵二叉搜索树是一棵满足下面条件的二叉树
//!   1. 所有左侧分支的值都小于本节点的值
//!   2. 本节点的值小于所有右侧分支的值

use crate::tree::binary::{Tree, TreeIndex, TreeNode};
use std::cmp::Ordering;

pub trait BSTree<K>
where
    K: std::cmp::PartialOrd,
{
    /// return true: insert success, false: not insert, exist k
    fn insert(&mut self, k: K) -> bool;
    fn delete(&mut self, k: K) -> bool;
    /// return node index
    fn find(&self, x: K) -> Option<TreeIndex>;
    fn min(&self) -> Option<TreeIndex>;
    fn max(&self) -> Option<TreeIndex>;
    /// 查找后继元素
    fn succ(&self, x: K) -> Option<TreeIndex>;
    /// 寻找前驱元素
    fn pred(&self, x: K) -> Option<TreeIndex>;
}

impl<K> BSTree<K> for Tree<K>
where
    K: std::cmp::PartialOrd + Copy,
{
    fn insert(&mut self, k: K) -> bool {
        insert(self, k, None, self.root)
    }

    fn delete(&mut self, k: K) -> bool {
        delete(self, k, self.root)
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
                    let child = TreeNode::new_leaf(k, Some(parent), None);
                    let child = tree.add_node(child);
                    tree.node_at_mut(parent).unwrap().right = Some(child);
                    true
                }
                Some(Ordering::Greater) => {
                    let child = TreeNode::new_leaf(k, Some(parent), None);
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
                        _ => return p,
                    }
                }
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
                        Some(l) if l.key == k => {
                            let p_node = tree.node_at(p.unwrap()).unwrap();
                            k = p_node.key;
                            p = p_node.parent;
                        }
                        _ => return p,
                    }
                }
            }
        }
    })
}

/// 从二叉搜索树中删除节点 x 的方法如下:
///   如果 x 没有子节点，或者只有一个孩子，直接将 x“切下”;
///   否则，x 有两个孩子，我们用其右子树中的最小值替换掉 x，然后将右子树中的这一最小值“切掉”。
///
/// idx, 起始node
fn delete<K>(tree: &mut Tree<K>, k: K, idx: Option<TreeIndex>) -> bool
where
    K: Copy + std::cmp::PartialOrd,
{
    find(tree, k, idx).map_or(false, |idx| {
        let node = tree.node_at(idx).unwrap();
        match node.children_count() {
            0 => {
                let parent = node.parent.unwrap();
                let parent_node = tree.node_at_mut(parent).unwrap();
                if parent_node.left == Some(idx) {
                    parent_node.left = None;
                } else if parent_node.right == Some(idx) {
                    parent_node.right = None;
                }
                tree.remove(idx);
            }
            1 => {
                //backup node child
                let node_child = if node.left.is_some() {
                    node.left
                } else {
                    node.right
                };

                // rm child, setup child node
                let parent = node.parent.unwrap();
                let parent_node = tree.node_at_mut(parent).unwrap();
                if parent_node.left == Some(idx) {
                    parent_node.left = node_child;
                } else if parent_node.right == Some(idx) {
                    parent_node.right = node_child;
                }
                tree.remove(idx);
            }
            _ => {
                //我们用其右子树中的最小值替换掉 x
                let right_min_idx = find_min(tree, node.right).unwrap();
                let right_min = tree.node_key(Some(right_min_idx)).unwrap();
                let node = tree.node_at_mut(idx).unwrap();
                node.key = right_min;

                //右子树中的这一最小值“切掉”
                let node_right = node.right;
                return delete(tree, right_min, node_right);
            }
        }

        true
    })
}
