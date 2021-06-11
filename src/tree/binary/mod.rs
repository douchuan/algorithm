use crate::tree::binary::rb::Color;
use std::cmp::max;

pub mod bst;
pub mod builder;
pub mod rb;
pub mod traverse;

pub struct TreeNode<K> {
    pub key: K,
    pub left: Option<usize>,
    pub right: Option<usize>,
    pub parent: Option<usize>,
    pub color: Option<Color>,
}

/// tree impl based Arena Allocators
/// https://sachanganesh.com/programming/graph-tree-traversals-in-rust/
pub struct Tree<K> {
    pub arena: Vec<Option<TreeNode<K>>>,
    pub root: Option<usize>,
}

impl<K> TreeNode<K> {
    pub fn new(
        key: K,
        left: Option<usize>,
        right: Option<usize>,
        parent: Option<usize>,
        color: Option<Color>,
    ) -> Self {
        TreeNode {
            key,
            left,
            right,
            parent,
            color,
        }
    }

    pub fn new_leaf(k: K, parent: Option<usize>, color: Option<Color>) -> Self {
        Self::new(k, None, None, parent, color)
    }

    pub fn from_key(key: K) -> Self {
        Self {
            key,
            left: None,
            right: None,
            parent: None,
            color: None,
        }
    }

    /// 一个节点的左右子树都为空，称之为 叶子节点
    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    /// 分支节点
    pub fn is_branch(&self) -> bool {
        !self.is_leaf()
    }

    /// 直接子节点个数，不包括孙子...
    pub fn children_count(&self) -> usize {
        match (self.left, self.right) {
            (Some(_), Some(_)) => 2,
            (Some(_), None) | (None, Some(_)) => 1,
            (None, None) => 0,
        }
    }
}

impl<K> Tree<K> {
    pub fn new() -> Self {
        Tree {
            arena: Vec::new(),
            root: None,
        }
    }

    pub fn add_node(&mut self, node: TreeNode<K>) -> usize {
        let index = self.arena.len();
        self.arena.push(Some(node));
        index
    }

    pub fn remove(&mut self, index: usize) {
        self.arena[index] = None;
    }

    pub fn node_at(&self, i: usize) -> Option<&TreeNode<K>> {
        self.arena.get(i).and_then(|v| v.as_ref())
    }

    pub fn left_node_at(&self, i: Option<usize>) -> Option<&TreeNode<K>> {
        i.and_then(|i| {
            self.arena.get(i).and_then(|v| {
                v.as_ref().and_then(|node| {
                    node.left
                        .and_then(|l| self.arena.get(l).and_then(|l| l.as_ref()))
                })
            })
        })
    }

    pub fn right_node_at(&self, i: Option<usize>) -> Option<&TreeNode<K>> {
        i.and_then(|i| {
            self.arena.get(i).and_then(|v| {
                v.as_ref().and_then(|node| {
                    node.right
                        .and_then(|r| self.arena.get(r).and_then(|r| r.as_ref()))
                })
            })
        })
    }

    pub fn node_at_mut(&mut self, i: usize) -> Option<&mut TreeNode<K>> {
        self.arena.get_mut(i).and_then(|v| v.as_mut())
    }

    pub fn height(&self) -> usize {
        fn calc<T>(tree: &Tree<T>, node: Option<usize>) -> usize {
            node.map_or(0, |node| {
                let node = tree.node_at(node).unwrap();
                let lh = calc(tree, node.left);
                let rh = calc(tree, node.right);
                1 + max(lh, rh)
            })
        }

        calc(self, self.root)
    }
}

impl<K> Tree<K>
where
    K: Copy,
{
    pub fn node_key(&self, i: Option<usize>) -> Option<K> {
        i.and_then(|i| self.node_at(i).and_then(|node| Some(node.key)))
    }
}
