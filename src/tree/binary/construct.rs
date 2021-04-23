use crate::tree::binary::{Tree, TreeIndex, TreeNode};
use std::collections::LinkedList;

enum NodeType {
    Root,
    LeftChild,
    RightChild,
}

impl NodeType {
    fn next(self) -> Self {
        match self {
            NodeType::Root => Self::LeftChild,
            NodeType::LeftChild => Self::RightChild,
            NodeType::RightChild => Self::LeftChild,
        }
    }
}

/// 按层建立tree
/// For example: Given binary tree {1,#,2,3},
///     1
///     \
///      2
///     /
///    3
pub fn new_tree(nodes: &[&str]) -> Tree {
    let mut tree = Tree::new();
    if nodes.is_empty() || nodes[0] == "#" {
        return tree;
    }

    let mut nodes = nodes.to_vec();
    nodes.reverse();

    let mut records = LinkedList::new();
    let mut nt = NodeType::Root;
    let mut parent_idx = None;
    while let Some(value) = nodes.pop() {
        let node_index = tree.add_value(value);

        //待处理node入队
        if let Some(v) = node_index {
            records.push_back(v);
        }

        match nt {
            NodeType::Root => {
                assert!(node_index.is_some(), "root node None");
                tree.set_root(node_index);
                parent_idx = records.pop_front();
            }
            NodeType::LeftChild => {
                let parent = parent_idx.expect("invalid parent index");
                let parent_node = tree.node_at_mut(parent).expect("invalid parent node");
                parent_node.left = node_index;
            }
            NodeType::RightChild => {
                let parent = parent_idx.expect("invalid parent index");
                let parent_node = tree.node_at_mut(parent).expect("invalid parent node");
                parent_node.right = node_index;

                //parent的left&right child node构建完毕，取下一个
                parent_idx = records.pop_front();
            }
        }

        nt = nt.next();
    }

    tree
}
