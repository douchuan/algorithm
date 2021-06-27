//! 红黑树
//!
//! 红黑树是一种自平衡二叉搜索树，通过对节点进行着色和旋转，红黑树可以很容易地保持树的平衡。
//! 我们需要在二叉搜索树上增加一个额外的颜色信息。节点可以被涂成红色或黑色。如果一棵二叉搜
//! 索树满足下面的全部5条性质，我们称之为红黑树。
//!   1. 任一节点要么是红色，要么是黑色
//!   2. 根节点为黑色
//!   3. 所有的叶节点(NIL 节点)为黑色
//!   4. 如果一个节点为红色，则它的两个子节点都是黑色 (red-violation)
//!   5. 对任一节点，从它出发到所有叶子节点的路径上包含相同数量的黑色节点 (black-violation)
//!
//! 这5条性质保证: 从根节点出发到达叶节点的所有路径中，最长路径不会超过最短路径两倍
//!
//! 注：
//! 第4条不允许两个连续的红色节点
//! 只有第5条，并不能保证rb tree平衡，比如：
//!             B
//!           /   \
//!         B       B
//!       /           \
//!      B              B
//!
//! 证明：含有n个节点的红黑树，其高度不会超过 2 * lg(n + 1)
//!
//! 红黑树，一种被广泛使用的自平衡二叉搜索树(self-adjusting balanced binary search tree)。
//! 另外一种自平衡树――AVL树。
//! splay树，它能够随着操作，逐渐把树变得越来越平衡。
//! 红黑树是最广泛使用的一种平衡二叉搜索树。另外一种自平衡二叉树是AVL树。
//! 红黑树可以帮助我们了解其它更复杂的数据结构。如果我们将子节点的数目从两个扩展到 k 个，
//! 并且保持树的平衡，就可以演化到B树。如果我们在边上，而非在节点中存储数据，我们就得到了Trie
//! 有很多集合(set)和 map 容器是使用红黑树来实现的。包括C++标准库STL
//!
//! 保证树的平衡
//! 通过二叉树旋转，使二叉搜索树保持平衡。旋转操作可以在保持元素顺序(中序遍历结果不变)的
//! 前提下，改变树的结构，因此可以用来提高平衡性。
//!
//! 令节点的颜色变量为C，取值为B或R。非空节点表达为一个四元组:
//! T = (C, l, k, r), 其中l、r是左右子树，k是值
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

/*

当插入一个 key 时，我们可以把新节点一律染成红色。只要它不是根节点，除了第四条外的所有红黑
树性质都可以满足。唯一的问题就是可能引入两个相邻的红色节点。
Chris Okasaki 指出，共有四种情况会违反红黑树的第四条性质。它们都带有两个相邻的红色节点。
非常关键的一点是: 它们可以被修复为一个统一形式。


插入后需要修复的四种情况

1.

            B(z)
           /    \
         R(y)    D
        /   \
      R(x)   C
    /     \
   A      B


2.

    B(x)
  /     \
 A      R(y)
       /    \
      B     R(z)
            /  \
           C    D

3.

        B(z)
      /      \
    R(x)      D
   /   \
  A    R(y)
      /    \
     B      C

4.
        B(x)
      /      \
     A       R(z)
            /    \
          R(y)    D
        /     \
       B       C

=============================================

被修复为一个统一形式

            R(y)
          /     \
        B(x)    B(z)
       /  \     /   \
      A   B    C     D

*/

use crate::tree::binary::node::Color;
use crate::tree::binary::{bst, Node, NodeQuery, Tree};
use std::ptr::NonNull;

//todo: rb tree delete, Chris Okasaki insert-fix
pub trait RedBlackTree<T> {
    fn insert(&mut self, element: T);
}

impl<T> RedBlackTree<T> for Tree<T>
where
    T: std::cmp::PartialOrd + std::marker::Copy,
{
    fn insert(&mut self, element: T) {
        self.root = insert(self.root, element);
    }
}

fn insert<T>(root: Option<NonNull<Node<T>>>, element: T) -> Option<NonNull<Node<T>>>
where
    T: std::cmp::PartialOrd + Copy,
{
    // 插入过程与bst是一样的
    if let Ok(x) = unsafe { bst::insert(root, element) } {
        let root = if root.is_none() { Some(x) } else { root };
        // 修正，使树恢复平衡
        insert_fix(root, Some(x))
    } else {
        None
    }
}

fn insert_fix<T>(
    root: Option<NonNull<Node<T>>>,
    x: Option<NonNull<Node<T>>>,
) -> Option<NonNull<Node<T>>> {
    let mut t = NodeQuery::new(root);
    let mut x = NodeQuery::new(x);
    while x.parent().color() == Some(Color::Red) {
        if x.uncle().color() == Some(Color::Red) {
            // case 1: ((a:R x:R b) y:B c:R) => ((a:R x:B b) y:R c:B)
            x.parent().set_color(Color::Black);
            x.grandparent().set_color(Color::Red);
            x.uncle().set_color(Color::Black);
            x = x.grandparent();
        } else {
            if x.parent().i_am_left() {
                if x.i_am_right() {
                    // case 2: ((a x:R b:R) y:B c) => case 3
                    x = x.parent();
                    t.node = rotate_left(t.node, x.node.unwrap());
                }
                // case 3: ((a:R x:R b) y:B c) => (a:R x:B (b y:R c))
                x.parent().set_color(Color::Black);
                x.grandparent().set_color(Color::Red);
                t.node = rotate_right(t.node, x.grandparent().node.unwrap());
            } else {
                if x.i_am_left() {
                    // case 2': (a x:B (b:R y:R c)) => case 3'
                    x = x.parent();
                    t.node = rotate_right(t.node, x.node.unwrap());
                }
                // case 3': (a x:B (b y:R c:R)) => ((a x:R b) y:B c:R)
                x.parent().set_color(Color::Black);
                x.grandparent().set_color(Color::Red);
                t.node = rotate_left(t.node, x.grandparent().node.unwrap());
            }
        }
    }
    t.set_color(Color::Black);
    t.node
}

/*
左旋操作变换为:

        X                        Y
      /   \                   /     \
     a     Y        =>       X       c
          /  \             /   \
         b    c           a     b

 */
pub fn rotate_left<T>(
    mut root: Option<NonNull<Node<T>>>,
    x: NonNull<Node<T>>,
) -> Option<NonNull<Node<T>>> {
    let mut x = NodeQuery::new(Some(x));
    let p = x.parent();
    let mut y = x.right();
    let a = x.left();
    let b = y.left();
    let c = y.right();
    x.replace(y.node);
    x.set_children(a.node, b.node);
    y.set_children(x.node, c.node);
    if p.is_none() {
        root = y.node;
    }
    root
}

/*
右旋操作变换为:

         Y                  X
      /     \            /     \
     X       c   =>     a       Y
   /   \                      /   \
  a     b                    b     c

 */
pub fn rotate_right<T>(
    mut root: Option<NonNull<Node<T>>>,
    y: NonNull<Node<T>>,
) -> Option<NonNull<Node<T>>> {
    let mut y = NodeQuery::new(Some(y));
    let p = y.parent();
    let mut x = y.left();
    let a = x.left();
    let b = x.right();
    let c = y.right();
    y.replace(x.node);
    y.set_children(b.node, c.node);
    x.set_children(a.node, y.node);
    if p.is_none() {
        root = x.node;
    }
    root
}

#[test]
fn t_insert() {
    use crate::tree::binary::traverse::{InOrderVisitor, PreOrderVisitor};
    use crate::tree::binary::Tree;

    /*
    [15, 14, 11, 2, 1, 7, 5, 4, 8]

                 7
            /        \
           2         14
         /  \       /  \
        1    5     11   15
            /     /
           4     8

    */
    let mut tree = Tree::default();
    for v in vec![15, 14, 11, 2, 1, 7, 5, 4, 8] {
        tree.root = insert(tree.root, v);
    }
    unsafe {
        assert_eq!(
            vec![7, 2, 1, 5, 4, 14, 11, 8, 15],
            PreOrderVisitor::recursive(&tree)
        );
        assert_eq!(
            vec![1, 2, 4, 5, 7, 8, 11, 14, 15],
            InOrderVisitor::recursive(&tree)
        );
    }

    /*
     [1, 2, 3, 4, 5, 6, 7, 8]

             4
         /       \
        2         6
      /   \      /  \
     1    3     5    7
                      \
                       8
    */
    let mut tree = Tree::default();
    for v in 1..9 {
        tree.root = insert(tree.root, v);
    }
    unsafe {
        assert_eq!(
            vec![4, 2, 1, 3, 6, 5, 7, 8],
            PreOrderVisitor::recursive(&tree)
        );
        assert_eq!(
            vec![1, 2, 3, 4, 5, 6, 7, 8],
            InOrderVisitor::recursive(&tree)
        );
    }
}
