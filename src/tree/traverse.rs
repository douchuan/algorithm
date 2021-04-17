#![allow(unused)]
use crate::tree::{Tree, TreeIndex, TreeNode};
use std::collections::HashSet;

pub struct PreOrderIter {
    stack: Vec<TreeIndex>,
}

pub struct InOrderIter {
    stack: Vec<TreeIndex>,
    last: Option<TreeIndex>,
}

pub struct PostOrderIter {
    stack: Vec<TreeIndex>,
}

impl PreOrderIter {
    pub fn new(root: Option<TreeIndex>) -> Self {
        if let Some(index) = root {
            PreOrderIter { stack: vec![index] }
        } else {
            PreOrderIter { stack: vec![] }
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

impl InOrderIter {
    pub fn new(root: Option<TreeIndex>) -> Self {
        let last = None;
        if let Some(index) = root {
            InOrderIter {
                stack: vec![index],
                last,
            }
        } else {
            InOrderIter {
                stack: vec![],
                last,
            }
        }
    }

    pub fn next(&mut self, tree: &Tree) -> Option<TreeIndex> {
        while let Some(&node_index) = self.stack.last() {
            if let Some(node) = tree.node_at(node_index) {
                if let Some(left) = node.left {
                    if let Some(last) = self.last {
                        if last == left {
                            break;
                        }
                    }

                    self.stack.push(left);
                    continue;
                }
            }
            break;
        }

        let node = self.stack.pop();

        if let Some(node_index) = node {
            if let Some(node) = tree.node_at(node_index) {
                if let Some(right) = node.right {
                    self.stack.push(right);
                }
            }
        }

        self.last = node;

        node
    }
}

fn inorder_recursive(tree: &Tree) -> Vec<usize> {
    let mut results = vec![];
    fn visitor(tree: &Tree, p: Option<TreeIndex>, results: &mut Vec<usize>) {
        if let Some(idx) = p {
            let node = tree.node_at(idx).expect("invalid node");
            visitor(tree, node.left, results);
            results.push(node.value);
            visitor(tree, node.right, results);
        }
    }
    visitor(tree, tree.get_root(), &mut results);
    results
}

impl PostOrderIter {
    pub fn new(root: Option<TreeIndex>) -> Self {
        PostOrderIter { stack: vec![] }
    }

    pub fn next(&mut self, tree: &Tree) -> Option<TreeIndex> {
        None
    }
}

fn t_helper_build_tree() -> Tree {
    let mut tree = Tree::new();
    let n3 = TreeNode::new(3, None, None);
    let n3 = tree.add_node(n3);
    let n2 = TreeNode::new(2, Some(n3), None);
    let n2 = tree.add_node(n2);
    let n1 = TreeNode::new(1, None, Some(n2));
    let n1 = tree.add_node(n1);
    tree.set_root(Some(n1));

    tree
}

#[test]
fn t_pre_order() {
    let tree = t_helper_build_tree();
    let mut iter = PreOrderIter::new(tree.get_root());
    let mut r = vec![];
    while let Some(i) = iter.next(&tree) {
        let node = tree.node_at(i).expect("invalid node index");
        r.push(node.value);
    }
    assert_eq!(vec![1, 2, 3], r);
}

#[test]
fn t_in_order() {
    let tree = t_helper_build_tree();
    let mut iter = InOrderIter::new(tree.get_root());
    let mut r = vec![];
    while let Some(i) = iter.next(&tree) {
        let node = tree.node_at(i).expect("invalid node index");
        r.push(node.value);
    }
    assert_eq!(vec![1, 3, 2], r);
}

#[test]
fn t_inorder_recursive() {
    let tree = t_helper_build_tree();
    let r = inorder_recursive(&tree);
    assert_eq!(vec![1, 3, 2], r);
}

#[test]
fn t_post_order() {
    let tree = t_helper_build_tree();
    let mut iter = PostOrderIter::new(tree.get_root());
    let mut r = vec![];
    while let Some(i) = iter.next(&tree) {
        let node = tree.node_at(i).expect("invalid node index");
        r.push(node.value);
    }
    assert_eq!(vec![3, 2, 1], r);
}
