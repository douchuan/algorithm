use crate::tree::binary::{Tree, TreeIndex, TreeNode};
use std::collections::LinkedList;

#[derive(Copy, Clone, PartialOrd, PartialEq)]
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
pub fn new_tree(orig: &[&str]) -> Tree {
    let mut tokens = LinkedList::new();
    tokens.extend(orig.iter());

    let mut tree = Tree::new();
    let mut records = LinkedList::new();
    let mut nt = NodeType::Root;
    let mut parent = None;

    while let Some(value) = tokens.pop_front() {
        let cur = tree.add_value(value);

        //待处理node入队
        records.push_back(cur);

        match nt {
            NodeType::Root => {
                tree.set_root(cur);
                parent = records.pop_front().unwrap();
            }
            NodeType::LeftChild => {
                if let Some(parent) = parent {
                    let parent_node = tree.node_at_mut(parent).expect("invalid parent node");
                    parent_node.left = cur;
                }
            }
            NodeType::RightChild => {
                if let Some(parent) = parent {
                    let parent_node = tree.node_at_mut(parent).expect("invalid parent node");
                    parent_node.right = cur;
                }

                //parent的left&right child node构建完毕，取下一个
                if let Some(next) = records.pop_front() {
                    parent = next;
                }
            }
        }

        match (parent, cur) {
            (None, Some(cur)) => {
                //无父接收，退货
                tokens.push_front(value);
                records.pop_back();
                tree.remove(cur);
            }
            _ => (),
        }

        nt = nt.next();
    }

    tree
}

#[test]
fn t_empty_tree() {
    let tree = new_tree(&[]);
    assert!(tree.arena.is_empty());
    assert!(tree.root.is_none());

    let tree = new_tree(&["#"]);
    assert!(tree.arena.is_empty());
    assert!(tree.root.is_none());
}
