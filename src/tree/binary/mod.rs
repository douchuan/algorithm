#![allow(unused)]
mod construct;
mod traverse;

pub type TreeIndex = usize;

pub struct TreeNode {
    pub value: usize,
    pub left: Option<TreeIndex>,
    pub right: Option<TreeIndex>,
}

impl TreeNode {
    pub fn new(value: usize, left: Option<TreeIndex>, right: Option<TreeIndex>) -> Self {
        TreeNode { value, left, right }
    }
}

/// tree impl based Arena Allocators
/// https://sachanganesh.com/programming/graph-tree-traversals-in-rust/
pub struct Tree {
    arena: Vec<Option<TreeNode>>,
    root: Option<TreeIndex>,
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            arena: Vec::new(),
            root: None,
        }
    }

    pub fn set_root(&mut self, root: Option<TreeIndex>) {
        self.root = root;
    }

    pub fn get_root(&self) -> Option<TreeIndex> {
        self.root
    }

    pub fn add_node(&mut self, node: TreeNode) -> TreeIndex {
        let index = self.arena.len();
        self.arena.push(Some(node));
        return index;
    }

    pub fn remove_node_at(&mut self, index: TreeIndex) -> Option<TreeNode> {
        if let Some(node) = self.arena.get_mut(index) {
            node.take()
        } else {
            None
        }
    }

    pub fn node_at(&self, index: TreeIndex) -> Option<&TreeNode> {
        return if let Some(node) = self.arena.get(index) {
            node.as_ref()
        } else {
            None
        };
    }

    pub fn node_at_mut(&mut self, index: TreeIndex) -> Option<&mut TreeNode> {
        return if let Some(node) = self.arena.get_mut(index) {
            node.as_mut()
        } else {
            None
        };
    }

    pub fn add_value(&mut self, value: &str) -> Option<TreeIndex> {
        match value {
            "#" => None,
            v => {
                let value = v.parse::<usize>().expect("invalid value");
                let node = TreeNode::new(value, None, None);
                Some(self.add_node(node))
            }
        }
    }
}
