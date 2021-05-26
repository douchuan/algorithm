//! 算法新解, 刘新宇
//! Version: 0.6180339887498949
//! 8.4 本质改进
//!
//! 构建锦标赛树，复用比较结果，对Selection sort的性能做改进

use crate::tree::binary::{Tree, TreeNode};
use std::cmp::max;

/// 排序结果：大 -> 小
///
/// 构建tree的时间复杂度 O(n)
/// 每次pop的时间复杂度 O(log2(n))，所以弹出n个元素的的时间复杂度为 O(n * log2(n))
pub fn sort_desc<T>(data: &[T]) -> Vec<T>
where
    T: Copy + std::cmp::Ord + Minimal,
{
    let mut tree = build_tournament_tree(data);
    let mut r = Vec::with_capacity(data.len());
    while let Some(v) = pop(&mut tree) {
        r.push(v);
    }
    r
}

pub trait Minimal {
    fn minimal() -> Self;
}

impl Minimal for i32 {
    fn minimal() -> Self {
        i32::MIN
    }
}

fn pop<T>(tree: &mut Tree<T>) -> Option<T>
where
    T: Copy + std::cmp::Ord + Minimal,
{
    match tree.node_key(tree.root) {
        Some(root_key) if root_key != T::minimal() => {
            //每次取出锦标赛树的根节点后，自顶向下将其替换为min
            let leaf = replace_max_by_min(tree, root_key);
            //由叶子节点的父指针向上回溯，设置新的最大值
            setup_new_max(tree, leaf);

            Some(root_key)
        }
        _ => None,
    }
}

// 返回叶子节点的序号
fn replace_max_by_min<T>(tree: &mut Tree<T>, root_key: T) -> usize
where
    T: Copy + std::cmp::Ord + Minimal,
{
    let mut idx = tree.root.unwrap();
    tree.node_at_mut(idx).unwrap().key = T::minimal();

    loop {
        match tree.node_at(idx) {
            Some(node) if node.is_leaf() => break,
            Some(node) => {
                if tree.node_key(node.left) == Some(root_key) {
                    let l = node.left.unwrap();
                    tree.node_at_mut(l).unwrap().key = T::minimal();
                    idx = l;
                    continue;
                }

                if tree.node_key(node.right) == Some(root_key) {
                    let r = node.right.unwrap();
                    tree.node_at_mut(r).unwrap().key = T::minimal();
                    idx = r;
                }
            }
            None => break,
        }
    }

    idx
}

fn setup_new_max<T>(tree: &mut Tree<T>, mut idx: usize)
where
    T: Copy + std::cmp::Ord,
{
    loop {
        match tree.node_at(idx) {
            Some(node) if node.parent.is_some() => {
                let parent = node.parent.unwrap();
                let parent_node = tree.node_at(parent).unwrap();
                let mut new_max = parent_node.key;
                if let Some(key) = tree.node_key(parent_node.left) {
                    new_max = new_max.max(key);
                }
                if let Some(key) = tree.node_key(parent_node.right) {
                    new_max = new_max.max(key);
                }
                tree.node_at_mut(parent).unwrap().key = new_max;

                idx = parent;
            }
            _ => break,
        }
    }
}

// build Tournament tree, from bottom to top
// a中不能包含T::minimal()这个特殊值，pop需要用到T::minimal()做临界值
fn build_tournament_tree<T>(data: &[T]) -> Tree<T>
where
    T: Copy + std::cmp::Ord,
{
    let mut tree = Tree::new();

    //build leaf
    let mut nodes: Vec<usize> = data
        .iter()
        .map(|v| tree.add_node(TreeNode::from_key(*v)))
        .collect();

    while nodes.len() > 1 {
        nodes = nodes
            .chunks(2)
            .map(|chunk| match chunk {
                &[t1, t2] => branch(&mut tree, t1, t2),
                &[t] => t,
                _ => unreachable!(),
            })
            .collect();
    }

    //tree.arena last is root
    let root = tree.arena.len().saturating_sub(1);
    tree.root = Some(root);

    tree
}

// 创建分支节点，取t1, t2较大者的value构造parent
fn branch<T>(tree: &mut Tree<T>, t1: usize, t2: usize) -> usize
where
    T: Copy + std::cmp::Ord,
{
    //create node
    let t1_node = tree.node_at(t1).unwrap();
    let t2_node = tree.node_at(t2).unwrap();
    let key = max(t1_node.key, t2_node.key);
    let node = TreeNode::new(key, Some(t1), Some(t2), None);
    let node_i = tree.add_node(node);

    //set parent
    let t1_node = tree.node_at_mut(t1).unwrap();
    t1_node.parent = Some(node_i);
    let t2_node = tree.node_at_mut(t2).unwrap();
    t2_node.parent = Some(node_i);

    node_i
}

#[test]
fn t_build_tree() {
    /*
                                            16
                        /                                       \
                     16                                           14
                 /         \                                /           \
            16                   13                    10                    14
         /     \               /     \               /    \                /     \
       7         16         8          13        10          9         12         14
     /  \       /  \       /  \       /  \      /   \      /  \       /  \       /  \
    7    6    15   16     8    4    13    3    5    10    9    1     12   2    11   14

    */
    let a = &[7, 6, 15, 16, 8, 4, 13, 3, 5, 10, 9, 1, 12, 2, 11, 14];
    let tree = build_tournament_tree(a);
    let r = crate::tree::binary::traverse::PreOrderVisitor::recursive(&tree);
    assert_eq!(
        r,
        vec![
            16, 16, 16, 7, 7, 6, 16, 15, 16, 13, 8, 8, 4, 13, 13, 3, 14, 10, 10, 5, 10, 9, 9, 1,
            14, 12, 12, 2, 14, 11, 14
        ]
    );
}

#[test]
fn t_pop() {
    let mut a = vec![7, 6, 15, 16, 8, 4, 13, 3, 5, 10, 9, 1, 12, 2, 11, 14];
    let mut tree = build_tournament_tree(&a);

    //make a desc sorted
    a.sort();
    a.reverse();

    for v in a {
        assert_eq!(pop(&mut tree), Some(v));
    }

    assert_eq!(pop(&mut tree), None);
}
