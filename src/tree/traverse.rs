#![allow(unused)]
use crate::tree::{Tree, TreeIndex, TreeNode};
use std::collections::HashSet;
use std::ops::Deref;

pub struct PreOrderVisitor;
pub struct InOrderVisitor;
pub struct PostOrderVisitor;

impl PreOrderVisitor {
    fn iterate(tree: &Tree) -> Vec<usize> {
        let mut results = vec![];
        let mut stack = vec![];
        //point current node
        let mut p = tree.root;
        while let Some(node_idx) = p {
            let node = tree.node_at(node_idx).expect("invalid node");

            results.push(node.value); //visit result

            if let Some(right) = node.right {
                stack.push(right)
            }
            if let Some(left) = node.left {
                stack.push(left)
            }

            p = stack.pop();
        }

        results
    }

    fn recursive(tree: &Tree) -> Vec<usize> {
        let mut results = vec![];
        fn visitor(tree: &Tree, p: Option<TreeIndex>, results: &mut Vec<usize>) {
            if let Some(node_idx) = p {
                let node = tree.node_at(node_idx).expect("invalid node");
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
    fn iterate(tree: &Tree) -> Vec<usize> {
        let mut results = vec![];
        let mut stack = vec![];
        //point current node
        let mut p = tree.root;
        loop {
            match (p, stack.is_empty()) {
                (None, true) => break,
                (Some(node_idx), _) => {
                    //traverse left nodes
                    stack.push(node_idx);
                    let node = tree.node_at(node_idx).expect("invalid node");
                    p = node.left;
                }
                (None, false) => {
                    //visit result & switch to right node
                    p = stack.pop();
                    let node_idx = p.unwrap();
                    let node = tree.node_at(node_idx).expect("invalid node");
                    results.push(node.value);
                    p = node.right;
                }
            }
        }

        results
    }

    fn recursive(tree: &Tree) -> Vec<usize> {
        let mut results = vec![];
        fn visitor(tree: &Tree, p: Option<TreeIndex>, results: &mut Vec<usize>) {
            if let Some(node_idx) = p {
                let node = tree.node_at(node_idx).expect("invalid node");
                visitor(tree, node.left, results);
                results.push(node.value); //visit result
                visitor(tree, node.right, results);
            }
        }
        visitor(tree, tree.get_root(), &mut results);
        results
    }
}

impl PostOrderVisitor {
    pub fn iterate(tree: &Tree) -> Vec<usize> {
        let mut results = vec![];
        let mut stack = vec![];
        let mut visited = HashSet::new();
        //point current node
        let mut p = tree.root;

        while let Some(node_idx) = p {
            let node = tree.node_at(node_idx).expect("invalid node");
            if let Some(left) = node.left {
                if !visited.contains(&left) {
                    stack.push(node_idx);
                    p = Some(left);
                    continue;
                }
            }

            if let Some(right) = node.right {
                if !visited.contains(&right) {
                    stack.push(node_idx);
                    p = Some(right);
                    continue;
                }
            }

            results.push(node.value);
            visited.insert(node_idx);
            p = stack.pop();
        }

        results
    }

    fn recursive(tree: &Tree) -> Vec<usize> {
        let mut results = vec![];
        fn visitor(tree: &Tree, p: Option<TreeIndex>, results: &mut Vec<usize>) {
            if let Some(node_idx) = p {
                let node = tree.node_at(node_idx).expect("invalid node");
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
fn t_preorder_iter() {
    let tree = t_helper_build_tree();
    let r = PreOrderVisitor::iterate(&tree);
    assert_eq!(vec![1, 2, 3], r);
}

#[test]
fn t_inorder_iter() {
    let tree = t_helper_build_tree();
    let r = InOrderVisitor::iterate(&tree);
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
fn t_postorder_iter() {
    let tree = t_helper_build_tree();
    let r = PostOrderVisitor::iterate(&tree);
    assert_eq!(vec![3, 2, 1], r);
}
