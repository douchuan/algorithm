//! 红黑树
//!
//! 红黑树是一种自平衡二叉搜索树，通过对节点进行着色和旋转，红黑树可以很容易地保持树的平衡。
//! 我们需要在二叉搜索树上增加一个额外的颜色信息。节点可以被涂成红色或黑色。如果一棵二叉搜
//! 索树满足下面的全部5条性质，我们称之为红黑树。
//!   1. 任一节点要么是红色，要么是黑色。
//!   2. 根节点为黑色。
//!   3. 所有的叶节点(NIL 节点)为黑色。
//!   4. 如果一个节点为红色，则它的两个子节点都是黑色。
//!   5. 对任一节点，从它出发到所有叶子节点的路径上包含相同数量的黑色节点。
//!
//! 红黑树，一种被广泛使用的自平衡二叉搜索树(self-adjusting balanced binary search tree)。
//! 另外一种自平衡树――AVL树。
//! splay树，它能够随着操作，逐渐把树变得越来越平衡。
//!
//! 保证树的平衡
//! 通过二叉树旋转，使二叉搜索树保持平衡。旋转操作可以在保持元素顺序(中序遍历结果不变)的
//! 前提下，改变树的结构，因此可以用来提高平衡性。
//!
//! 有很多集合(set)和 map 容器是使用红黑树来实现的。包括 C++ 标准 库 STL
//!
//!
//! 证明：含有 n 个节点的红黑树，其高度 h 不会超过 2 * lg(n + 1)
//!
//!
//! 令节点的颜色变量为C，取值为B或R。非空节点表达为一个四元组:
//! T = (C, l, k, r), 其中l、r是左右子树，k是值
//!
//!
//! 红黑树插入算法定义：
//! insert T k = makeBlack (ins T k)
//!
//! ins EMPTY k = (R, EMPTY, k, EMPTY)
//!
//! ins (C l, k', r) k 分2种情况
//!     1. k < k':  balance C (ins l k) k' r
//!     2. k > k':  balance C l k' (ins r k)
//!
//! balance B (R, (R, a, x, b), y, c) z d = (R, (B, a, x, b), y, (B, c, z, d))
//! balance B (R, a, x, (R, b, y, c)) z d = (R, (B, a, x, b), y, (B, c, z, d))
//! balance B a x (R, b, y (R, c, z, d))  = (R, (B, a, x, b), y, (B, c, z, d))
//! balance B a x (R, (R, b, y, c), z, d) = (R, (B, a, x, b), y, (B, c, z, d))
//! balance T                             = T, 如果不满足上面4中模式, 不对T做变换
//!
//! makeBlack (C, l, k, r) = (B, l, k, r)
//!
//! balance的四种情况都把红色向上移动一层。如果进行自底向上的递归修复，可能会把根节点染成红色。
//! 根据性质2，最后需要把根节点变回黑色。

use crate::tree::binary::Node;
use std::ptr::NonNull;

pub trait RedBlackTree<T>
where
    T: std::cmp::PartialOrd,
{
    /// return
    ///   Some(NonNull<Node<T>>): insert success and return inserted node,
    ///   None: not insert, exist
    fn insert(&mut self, element: T) -> Option<NonNull<Node<T>>>;

    /// 此操作不改变rb tree结构，复用bst find
    fn find(&self, element: T) -> Option<NonNull<Node<T>>>;
    /// 此操作不改变rb tree结构，复用bst min
    fn min(&self) -> Option<T>;
    /// 此操作不改变rb tree结构，复用bst max
    fn max(&self) -> Option<T>;
}

// todo: if element exist, just return
unsafe fn insert<T>(mut p: Option<NonNull<Node<T>>>, element: T)
where
    T: std::cmp::PartialOrd + Copy,
{
    let mut root = p;
    let mut x = Some(Node::from_element(element));
    let mut parent = None;

    //寻找插入点
    loop {
        match p {
            Some(node) => {
                parent = p;
                p = if element < node.as_ref().element {
                    node.as_ref().left
                } else {
                    node.as_ref().right
                };
            }
            None => break,
        }
    }

    //插入x
    match parent {
        None => root = x,
        Some(mut parent) => {
            if element < parent.as_ref().element {
                parent.as_mut().left = x;
            } else {
                parent.as_mut().right = x;
            }
            x.unwrap().as_mut().parent = Some(parent);
        }
    }

    insert_fix(root.unwrap(), x.unwrap())
}

unsafe fn insert_fix<T>(mut p: NonNull<Node<T>>, mut x: NonNull<Node<T>>)
where
    T: std::cmp::PartialOrd + Copy,
{
}
