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
use std::collections::LinkedList;

pub trait BuildTreeInLevel<K> {
    fn build_in_level(vec: &[&str]) -> Tree<K>;
}

impl<K> BuildTreeInLevel<K> for TreeBuilder
where
    K: std::str::FromStr,
{
    fn build_in_level(vec: &[&str]) -> Tree<K> {
        build(vec)
    }
}

fn build<K>(vec: &[&str]) -> Tree<K>
where
    K: std::str::FromStr,
{
    let mut tokens = LinkedList::new();
    tokens.extend(vec.iter());

    let mut tree = Tree::default();
    let mut records = LinkedList::new();
    let mut nt = NodeType::Root;
    let mut parent = None;

    // println!("tokens = {:?}", tokens);
    while let Some(value) = tokens.pop_front() {
        let cur = Node::from_str(value);
        // println!("parent = {:?}, cur = {}", parent, value);

        match (parent, cur) {
            (None, Some(cur)) if nt != NodeType::Root => {
                //无父接收，退货
                tokens.push_front(value);
                Node::release(cur);
            }
            //待处理node入队
            _ => records.push_back(cur),
        }

        match nt {
            NodeType::Root => {
                tree.root = cur;
                parent = records.pop_front().unwrap();
            }
            NodeType::LeftChild => {
                if let Some(mut parent) = parent {
                    unsafe {
                        parent.as_mut().left = cur;
                    }
                }
            }
            NodeType::RightChild => {
                if let Some(mut parent) = parent {
                    unsafe {
                        parent.as_mut().right = cur;
                    }
                }

                //parent的left&right child node构建完毕，取下一个
                if let Some(next) = records.pop_front() {
                    parent = next;
                }
            }
        }

        nt = nt.next();
    }

    tree
}

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
