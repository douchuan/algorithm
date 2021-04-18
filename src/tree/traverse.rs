#![allow(unused)]
use crate::tree::{Tree, TreeIndex, TreeNode};
use std::collections::HashSet;

pub struct PreOrderVisitor {
    stack: Vec<TreeIndex>,
}

pub struct InOrderVisitor {
    stack: Vec<TreeIndex>,
    last: Option<TreeIndex>,
}

pub struct PostOrderVisitor {
    stack: Vec<TreeIndex>,
}

impl PreOrderVisitor {
    pub fn new(root: Option<TreeIndex>) -> Self {
        if let Some(index) = root {
            PreOrderVisitor { stack: vec![index] }
        } else {
            PreOrderVisitor { stack: vec![] }
        }
    }

    pub fn iter_next(&mut self, tree: &Tree) -> Option<TreeIndex> {
        let node_index = self.stack.pop();
        if let Some(node_index) = node_index {
            let node = tree.node_at(node_index).expect("invalid node");
            if let Some(right) = node.right {
                self.stack.push(right)
            }

            if let Some(left) = node.left {
                self.stack.push(left)
            }
        }
        node_index
    }

    fn recursive(tree: &Tree) -> Vec<usize> {
        let mut results = vec![];
        fn visitor(tree: &Tree, p: Option<TreeIndex>, results: &mut Vec<usize>) {
            if let Some(idx) = p {
                let node = tree.node_at(idx).expect("invalid node");
                results.push(node.value);
                visitor(tree, node.left, results);
                visitor(tree, node.right, results);
            }
        }
        visitor(tree, tree.get_root(), &mut results);
        results
    }
}

impl InOrderVisitor {
    pub fn new(root: Option<TreeIndex>) -> Self {
        let last = None;
        if let Some(index) = root {
            InOrderVisitor {
                stack: vec![index],
                last,
            }
        } else {
            InOrderVisitor {
                stack: vec![],
                last,
            }
        }
    }

    pub fn iter_next(&mut self, tree: &Tree) -> Option<TreeIndex> {
        while let Some(&node_index) = self.stack.last() {
            let node = tree.node_at(node_index).expect("invalid node");
            if let Some(left) = node.left {
                if let Some(last) = self.last {
                    if last == left {
                        break;
                    }
                }
                self.stack.push(left);
            } else {
                break;
            }
        }

        let node = self.stack.pop();

        if let Some(node_index) = node {
            let node = tree.node_at(node_index).expect("invalid node");
            if let Some(right) = node.right {
                self.stack.push(right);
            }
        }

        self.last = node;

        node
    }

    fn recursive(tree: &Tree) -> Vec<usize> {
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
}

impl PostOrderVisitor {
    pub fn new(root: Option<TreeIndex>) -> Self {
        PostOrderVisitor { stack: vec![] }
    }

    pub fn next(&mut self, tree: &Tree) -> Option<TreeIndex> {
        None
    }

    fn recursive(tree: &Tree) -> Vec<usize> {
        let mut results = vec![];
        fn visitor(tree: &Tree, p: Option<TreeIndex>, results: &mut Vec<usize>) {
            if let Some(idx) = p {
                let node = tree.node_at(idx).expect("invalid node");
                visitor(tree, node.left, results);
                visitor(tree, node.right, results);
                results.push(node.value);
            }
        }
        visitor(tree, tree.get_root(), &mut results);
        results
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
    let mut iter = PreOrderVisitor::new(tree.get_root());
    let mut r = vec![];
    while let Some(i) = iter.iter_next(&tree) {
        let node = tree.node_at(i).expect("invalid node index");
        r.push(node.value);
    }
    assert_eq!(vec![1, 2, 3], r);
}

#[test]
fn t_in_order() {
    let tree = t_helper_build_tree();
    let mut iter = InOrderVisitor::new(tree.get_root());
    let mut r = vec![];
    while let Some(i) = iter.iter_next(&tree) {
        let node = tree.node_at(i).expect("invalid node index");
        r.push(node.value);
    }
    assert_eq!(vec![1, 3, 2], r);
}

#[test]
fn t_preorder_recursive() {
    let tree = t_helper_build_tree();
    let r = PreOrderVisitor::recursive(&tree);
    assert_eq!(vec![1, 2, 3], r);
}

#[test]
fn t_inorder_recursive() {
    let tree = t_helper_build_tree();
    let r = InOrderVisitor::recursive(&tree);
    assert_eq!(vec![1, 3, 2], r);
}

#[test]
fn t_postorder_recursive() {
    let tree = t_helper_build_tree();
    let r = PostOrderVisitor::recursive(&tree);
    assert_eq!(vec![3, 2, 1], r);
}

#[test]
fn t_post_order() {
    let tree = t_helper_build_tree();
    let mut iter = PostOrderVisitor::new(tree.get_root());
    let mut r = vec![];
    while let Some(i) = iter.next(&tree) {
        let node = tree.node_at(i).expect("invalid node index");
        r.push(node.value);
    }
    assert_eq!(vec![3, 2, 1], r);
}
