#![allow(unused)]
use crate::tree::binary::construct;
use crate::tree::binary::{Tree, TreeIndex, TreeNode};
use std::collections::{HashSet, LinkedList};

/// Binary Tree Preorder Traversal
/// Given a binary tree, return the preorder traversal of its nodes’ values.
/// For example: Given binary tree {1, #, 2, 3},
///     1
///     \
///      2
///     /
///    3
/// return [1, 2, 3].
/// Note: Recursive solution is trivial, could you do it iteratively?
pub struct PreOrderVisitor;

/// Binary Tree Inorder Traversal
/// Given a binary tree, return the inorder traversal of its nodes’ values.
/// For example: Given binary tree {1, #, 2, 3},
///     1
///     \
///      2
///     /
///    3
/// return [1, 3, 2].
/// Note: Recursive solution is trivial, could you do it iteratively?
pub struct InOrderVisitor;

/// Binary Tree Postorder Traversal
/// Given a binary tree, return the postorder traversal of its nodes’ values.
/// For example: Given binary tree {1, #, 2, 3},
///     1
///     \
///      2
///     /
///    3
/// return [3, 2, 1].
/// Note: Recursive solution is trivial, could you do it iteratively?
pub struct PostOrderVisitor;

/// Binary Tree Level Order Traversal
/// Given a binary tree, return the level order traversal of its nodes’
/// values. (ie, from left to right, level by level).
/// For example: Given binary tree {3, 9, 20, #, #, 15, 7},
///     3
///    / \
///   9  20
///     / \
///    15  7
/// return its level order traversal as:
/// [
///     [3],
///     [9, 20],
///     [15, 7]
/// ]
pub struct LevelOrderVisitor;

/// Binary Tree Level Order Traversal 2
/// Given a binary tree, return the level order traversal of its nodes’
/// values. (ie, from left to right, level by level).
/// For example: Given binary tree {3,9,20,#,#,15,7},
///     3
///    / \
///   9  20
///     / \
///    15  7
/// return its bottom-up level order traversal as:
/// [
///     [15, 7]
///     [9, 20],
///     [3],
/// ]
pub struct LevelOrderVisitor2;

/// Zigzag Level Order Traversal
/// Given a binary tree, return the level order traversal of its nodes’
/// values. (ie, from left to right, level by level).
/// For example: Given binary tree {3,9,20,#,#,15,7},
///     3
///    / \
///   9  20
///     / \
///    15  7
/// return its zigzag level order traversal as:
/// [
///     [3],
///     [20, 9],
///     [15, 7]
/// ]
pub struct ZigzagOrderVisitor;

impl PreOrderVisitor {
    fn iterate(tree: &Tree) -> Vec<usize> {
        let mut results = vec![];
        let mut stack = vec![];
        //point current node
        let mut p = tree.root;
        while let Some(node_idx) = p {
            let node = tree.node_at(node_idx).expect("invalid node");
            results.push(node.value); //visit result
            for pp in &[node.right, node.left] {
                if let Some(pp) = pp {
                    stack.push(*pp);
                }
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
                (Some(node_idx), _) => {
                    //switch to left child
                    stack.push(node_idx);
                    let node = tree.node_at(node_idx).expect("invalid node");
                    p = node.left;
                }
                (None, false) => {
                    //visit result & switch to right child
                    p = stack.pop();
                    let node_idx = p.unwrap();
                    let node = tree.node_at(node_idx).expect("invalid node");
                    results.push(node.value);
                    p = node.right;
                }
                (None, true) => break,
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

            //switch to left child
            match node.left {
                Some(left) if !visited.contains(&left) => {
                    stack.push(node_idx);
                    p = Some(left);
                    continue;
                }
                _ => (),
            }

            //switch to right child
            match node.right {
                Some(right) if !visited.contains(&right) => {
                    stack.push(node_idx);
                    p = Some(right);
                    continue;
                }
                _ => (),
            }

            //visit & record node
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

impl LevelOrderVisitor {
    pub fn iterate(tree: &Tree) -> Vec<Vec<usize>> {
        let mut results = vec![];

        if let Some(p) = tree.get_root() {
            let mut nodes = LinkedList::new();
            let mut next_level_nodes = vec![];

            //root node enqueue
            nodes.push_back(p);
            results.push(vec![]);

            loop {
                match nodes.pop_front() {
                    Some(p) => {
                        let node = tree.node_at(p).expect("invalid node");
                        results
                            .last_mut()
                            .expect("empty results container")
                            .push(node.value);
                        for child in &[node.left, node.right] {
                            if let Some(child) = child {
                                next_level_nodes.push(*child);
                            }
                        }
                    }
                    None => {
                        if next_level_nodes.is_empty() {
                            break;
                        } else {
                            results.push(vec![]);
                            nodes.extend(next_level_nodes.iter());
                            next_level_nodes.clear();
                        }
                    }
                }
            }
        }

        results
    }

    fn recursive(tree: &Tree) -> Vec<Vec<usize>> {
        let mut results = vec![];
        fn visitor(
            tree: &Tree,
            mut level_nodes: Vec<TreeIndex>,
            results: &mut Vec<Vec<usize>>,
            pos: usize,
        ) {
            if level_nodes.is_empty() {
                return;
            }

            results.push(vec![]);

            let mut next_level_nodes = vec![];
            for p in level_nodes {
                let node = tree.node_at(p).expect("invalid node");
                for child in &[node.left, node.right] {
                    if let Some(child) = child {
                        next_level_nodes.push(*child);
                    }
                }
                results[pos].push(node.value);
            }

            visitor(tree, next_level_nodes, results, pos + 1);
        }
        if let Some(p) = tree.get_root() {
            visitor(tree, vec![p], &mut results, 0);
        }
        results
    }
}

impl LevelOrderVisitor2 {
    pub fn iterate(tree: &Tree) -> Vec<Vec<usize>> {
        let mut r = LevelOrderVisitor::iterate(tree);
        r.reverse();
        r
    }

    fn recursive(tree: &Tree) -> Vec<Vec<usize>> {
        let mut r = LevelOrderVisitor::recursive(tree);
        r.reverse();
        r
    }
}

#[test]
fn t_preorder_iter() {
    let nodes = vec!["1", "#", "2", "3"];
    let tree = construct::new_tree(&nodes);
    let r = PreOrderVisitor::iterate(&tree);
    assert_eq!(vec![1, 2, 3], r);
}

#[test]
fn t_inorder_iter() {
    let nodes = vec!["1", "#", "2", "3"];
    let tree = construct::new_tree(&nodes);
    let r = InOrderVisitor::iterate(&tree);
    assert_eq!(vec![1, 3, 2], r);
}

#[test]
fn t_preorder_recursive() {
    let nodes = vec!["1", "#", "2", "3"];
    let tree = construct::new_tree(&nodes);
    let r = PreOrderVisitor::recursive(&tree);
    assert_eq!(vec![1, 2, 3], r);
}

#[test]
fn t_inorder_recursive() {
    let nodes = vec!["1", "#", "2", "3"];
    let tree = construct::new_tree(&nodes);
    let r = InOrderVisitor::recursive(&tree);
    assert_eq!(vec![1, 3, 2], r);
}

#[test]
fn t_postorder_recursive() {
    let nodes = vec!["1", "#", "2", "3"];
    let tree = construct::new_tree(&nodes);
    let r = PostOrderVisitor::recursive(&tree);
    assert_eq!(vec![3, 2, 1], r);
}

#[test]
fn t_postorder_iter() {
    let nodes = vec!["1", "#", "2", "3"];
    let tree = construct::new_tree(&nodes);
    let r = PostOrderVisitor::iterate(&tree);
    assert_eq!(vec![3, 2, 1], r);
}

#[test]
fn t_levelorder_iter() {
    let nodes = vec!["3", "9", "20", "#", "#", "15", "7"];
    let tree = construct::new_tree(&nodes);
    let r = LevelOrderVisitor::iterate(&tree);
    assert_eq!(vec![vec![3], vec![9, 20], vec![15, 7]], r);
}

#[test]
fn t_levelorder_traverse() {
    let nodes = vec!["3", "9", "20", "#", "#", "15", "7"];
    let tree = construct::new_tree(&nodes);
    let r = LevelOrderVisitor::recursive(&tree);
    assert_eq!(vec![vec![3], vec![9, 20], vec![15, 7]], r);
}

#[test]
fn t_levelorder2_iter() {
    let nodes = vec!["3", "9", "20", "#", "#", "15", "7"];
    let tree = construct::new_tree(&nodes);
    let r = LevelOrderVisitor2::iterate(&tree);
    assert_eq!(vec![vec![15, 7], vec![9, 20], vec![3]], r);
}

#[test]
fn t_levelorder2_traverse() {
    let nodes = vec!["3", "9", "20", "#", "#", "15", "7"];
    let tree = construct::new_tree(&nodes);
    let r = LevelOrderVisitor2::recursive(&tree);
    assert_eq!(vec![vec![15, 7], vec![9, 20], vec![3]], r);
}
