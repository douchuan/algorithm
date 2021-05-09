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

    // println!("tokens = {:?}", tokens);
    while let Some(value) = tokens.pop_front() {
        let cur = tree.add_value(value);
        // println!("parent = {:?}, cur = {}", parent, value);

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
                } else if let Some(cur) = cur {
                    //无父接收，退货
                    tokens.push_front(value);
                    records.pop_back();
                    tree.remove(cur);
                }
            }
            NodeType::RightChild => {
                if let Some(parent) = parent {
                    let parent_node = tree.node_at_mut(parent).expect("invalid parent node");
                    parent_node.right = cur;
                } else if let Some(cur) = cur {
                    //无父接收，退货
                    tokens.push_front(value);
                    records.pop_back();
                    tree.remove(cur);
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

#[test]
fn t_empty_tree() {
    let tree = new_tree(&[]);
    assert!(tree.arena.is_empty());
    assert!(tree.root.is_none());

    let tree = new_tree(&["#"]);
    assert!(tree.arena.is_empty());
    assert!(tree.root.is_none());
}

#[test]
fn t_tree() {
    let tokens = vec!["1", "2", "#", "3", "4", "#", "#", "5"];
    let tree = new_tree(&tokens);
    let p0 = tree.root.expect("invalid p0");
    let p0 = tree.node_at(p0).expect("invalid p0 node");
    assert_eq!(p0.value, 1);
    assert!(p0.right.is_none());
    let p1 = p0.left.expect("invalid p0 left");
    let p1 = tree.node_at(p1).expect("invalid p1 node");
    assert_eq!(p1.value, 2);
    //p2: '#'
    let p3 = p1.left.expect("invalid p3");
    let p3 = tree.node_at(p3).expect("invalid p3 node");
    assert_eq!(p3.value, 3);
    let p4 = p1.right.expect("invalid p4");
    let p4 = tree.node_at(p4).expect("invalid p3 node");
    assert_eq!(p4.value, 4);
    //p5: '#'
    //p6: '#'
    let p7 = p3.left.expect("invalid p7");
    let p7 = tree.node_at(p7).expect("invalid p7 node");
    assert_eq!(p7.value, 5);
    assert_eq!(tree.arena.len(), 5);
}
