#![allow(unused)]
use crate::tree::{Tree, TreeIndex, TreeNode};

pub struct PreorderIter {
    stack: Vec<TreeIndex>,
}

impl PreorderIter {
    pub fn new(root: Option<TreeIndex>) -> Self {
        if let Some(index) = root {
            PreorderIter { stack: vec![index] }
        } else {
            PreorderIter { stack: vec![] }
        }
    }

    pub fn next(&mut self, tree: &Tree) -> Option<TreeIndex> {
        while let Some(node_index) = self.stack.pop() {
            if let Some(node) = tree.node_at(node_index) {
                if let Some(right) = node.right {
                    self.stack.push(right)
                }

                if let Some(left) = node.left {
                    self.stack.push(left)
                }

                return Some(node_index);
            }
        }

        return None;
    }
}

#[test]
fn basic() {
    let mut tree = Tree::new();
    let n3 = TreeNode::new(3, None, None);
    let n3 = tree.add_node(n3);
    let n2 = TreeNode::new(2, Some(n3), None);
    let n2 = tree.add_node(n2);
    let n1 = TreeNode::new(1, None, Some(n2));
    let n1 = tree.add_node(n1);
    let mut iter = PreorderIter::new(Some(n1));
    let mut r = vec![];
    while let Some(i) = iter.next(&tree) {
        let node = tree.node_at(i).expect("invalid node index");
        r.push(node.value);
    }
    assert_eq!(vec![1, 2, 3], r);
}
