#![allow(unused)]
//! 二叉树
//! 是一种更为典型的树状结构。如它名字所描述的那样，二叉树是每个节点最多有两个子树的树结构，
//! 通常子树被称作“左子树”和“右子树”。
//!
//! 前序遍历
//! 前序遍历首先访问根节点，然后遍历左子树，最后遍历右子树。
//!
//! 中序遍历
//! 中序遍历是先遍历左子树，然后访问根节点，然后遍历右子树。
//!
//! 后序遍历
//! 后序遍历是先遍历左子树，然后遍历右子树，最后访问树的根节点。
//!

mod traverse;

pub struct TreeNode {
    pub value: usize,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn new(value: usize, left: Option<Box<TreeNode>>, right: Option<Box<TreeNode>>) -> Self {
        TreeNode { value, left, right }
    }
}

pub struct Tree {
    pub root: Option<TreeNode>,
}

impl Tree {
    pub fn new(root: Option<TreeNode>) -> Self {
        Tree { root }
    }
}
