#![allow(unused)]
pub mod construct;
pub mod traverse;
pub mod visualization;

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
    pub arena: Vec<Option<TreeNode>>,
    pub root: Option<TreeIndex>,
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
        index
    }

    pub fn remove(&mut self, index: TreeIndex) {
        self.arena.remove(index);
    }

    pub fn node_at(&self, index: TreeIndex) -> Option<&TreeNode> {
        if let Some(node) = self.arena.get(index) {
            node.as_ref()
        } else {
            None
        }
    }

    pub fn node_at_mut(&mut self, index: TreeIndex) -> Option<&mut TreeNode> {
        if let Some(node) = self.arena.get_mut(index) {
            node.as_mut()
        } else {
            None
        }
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

    pub fn height(&self) -> usize {
        fn calc(tree: &Tree, parent: Option<usize>) -> usize {
            match parent {
                Some(parent) => {
                    let node = tree.node_at(parent).expect("invalid index");
                    let lh = calc(tree, node.left);
                    let rh = calc(tree, node.right);
                    1 + lh.max(rh)
                }
                None => 0,
            }
        }

        calc(self, self.root)
    }
}
