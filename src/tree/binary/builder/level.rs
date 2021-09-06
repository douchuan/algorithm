//! 按层建立tree
//! For example: Given binary tree {1,#,2,3},
//!     1
//!     \
//!      2
//!     /
//!    3

use crate::tree::binary::builder::TreeBuilder;
use crate::tree::binary::node::Node;
use crate::tree::binary::tree::Tree;
use std::ptr::NonNull;

pub trait BuildTreeInLevel<K, V> {
    fn build_in_level(vec: &[&str]) -> Tree<K, V>;
}

impl<K: std::str::FromStr, V> BuildTreeInLevel<K, V> for TreeBuilder {
    fn build_in_level(vec: &[&str]) -> Tree<K, V> {
        build(vec)
    }
}

// ?? why K: std::str::FromStr
fn build<K: std::str::FromStr, V>(vec: &[&str]) -> Tree<K, V> {
    let tokens = expand_sharp(vec);
    let mut tree = Tree::default();
    let mut tree_size = 0;
    let mut aux: Vec<Option<NonNull<Node<K, V>>>> = Vec::new();
    for (i, &token) in tokens.iter().enumerate() {
        let node = token.parse().map(Node::new_key).ok();
        aux.push(node);

        if node.is_some() {
            tree_size += 1;
        }

        if i == 0 {
            tree.root = node;
            continue;
        }

        match node {
            Some(mut node) => unsafe {
                let parent_i = ((i + 1) >> 1) - 1;
                let left_i = (parent_i << 1) + 1;
                let mut parent_node = aux[parent_i].unwrap();
                node.as_mut().parent = Some(parent_node);
                if left_i == i {
                    parent_node.as_mut().left = Some(node);
                } else {
                    parent_node.as_mut().right = Some(node);
                }
            },
            None => {}
        }
    }

    tree.set_size(tree_size);

    tree
}

// "#" symbol as empty child
fn expand_sharp<'a>(vec: &[&'a str]) -> Vec<&'a str> {
    let mut results = Vec::new();

    for &v in vec.iter() {
        // root
        if results.is_empty() {
            results.push(v);
            continue;
        }

        // child
        if v == "#" {
            results.push(v);
        } else {
            // child
            loop {
                // new child idx
                let idx = results.len();
                let parent_idx = ((idx + 1) >> 1) - 1;
                let parent = results[parent_idx];
                if results[parent_idx] == "#" {
                    // if parent is "#", push "#"
                    results.push(parent);
                } else {
                    results.push(v);
                    break;
                }
            }
        }
    }

    results
}

#[test]
fn t_expand_sharp() {
    let vec = vec!["1", "#", "2", "3"];
    let results = vec!["1", "#", "2", "#", "#", "3"];
    assert_eq!(results, expand_sharp(vec.as_slice()));

    let vec = vec!["1", "2", "#", "3", "4", "#", "#", "5"];
    let results = vec!["1", "2", "#", "3", "4", "#", "#", "5"];
    assert_eq!(results, expand_sharp(vec.as_slice()));
}
