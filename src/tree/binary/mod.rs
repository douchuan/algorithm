use std::cmp::max;

pub mod construct;
pub mod traverse;

pub type TreeIndex = usize;

pub struct TreeNode<K> {
    pub key: K,
    pub left: Option<TreeIndex>,
    pub right: Option<TreeIndex>,
    pub parent: Option<TreeIndex>,
}

impl<K> TreeNode<K> {
    pub fn new(
        key: K,
        left: Option<TreeIndex>,
        right: Option<TreeIndex>,
        parent: Option<TreeIndex>,
    ) -> Self {
        TreeNode {
            key,
            left,
            right,
            parent,
        }
    }

    pub fn from_key(key: K) -> Self {
        Self {
            key,
            left: None,
            right: None,
            parent: None,
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
}

/// tree impl based Arena Allocators
/// https://sachanganesh.com/programming/graph-tree-traversals-in-rust/
pub struct Tree<K> {
    pub arena: Vec<Option<TreeNode<K>>>,
    pub root: Option<TreeIndex>,
}

impl<K> Tree<K> {
    pub fn new() -> Self {
        Tree {
            arena: Vec::new(),
            root: None,
        }
    }

    pub fn add_node(&mut self, node: TreeNode<K>) -> TreeIndex {
        let index = self.arena.len();
        self.arena.push(Some(node));
        index
    }

    pub fn remove(&mut self, index: TreeIndex) {
        self.arena.remove(index);
    }

    pub fn node_at(&self, i: TreeIndex) -> Option<&TreeNode<K>> {
        self.arena.get(i).and_then(|v| v.as_ref())
    }

    pub fn node_at_mut(&mut self, i: TreeIndex) -> Option<&mut TreeNode<K>> {
        self.arena.get_mut(i).and_then(|v| v.as_mut())
    }

    pub fn height(&self) -> usize {
        fn calc<T>(tree: &Tree<T>, node: Option<TreeIndex>) -> usize {
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
    pub fn node_key(&self, i: Option<TreeIndex>) -> Option<K> {
        i.and_then(|i| self.node_at(i).and_then(|node| Some(node.key)))
    }
}
