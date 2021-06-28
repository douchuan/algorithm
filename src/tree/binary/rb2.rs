//! 从2-3-4树的角度，理解rb tree
//!
//! Algorithms 4th Edition by Robert Sedgewick, Kevin Wayne
//!
//! 3.3 Balanced Search Trees, Red-black BSTs
//!
//! Encoding 3-nodes. The basic idea behind red-black
//! BSTs is to encode 2-3 trees by starting with standard
//! BSTs (which are made up of 2-nodes) and adding extra
//! information to encode 3-nodes. We think of the links
//! as being of two different types: red links, which bind
//! together two 2-nodes to represent 3-nodes, and black
//! links, which bind together the 2-3 tree. Specifically,
//! we represent 3-nodes as two 2-nodes connected by a
//! single red link that leans left (one of the 2-nodes is the
//! left child of the other). One advantage of using such a
//! representation is that it allows us to use our get() code
//! for standard BST search without modification. Given
//! any 2-3 tree, we can immediately derive a corresponding BST,
//! just by converting each node as specified. We refer to BSTs that
//! represent 2-3 trees in this way as red-black BSTs.
//!
//! Encoding a 3-node with two 2-nodes connected by a left-leaning red link
//
//             a    b
//          /    |     \
//       less  between   greater than b
//        a    a and b
//
//
//               b
//             /    \
//           R(a)      greater than b
//         /     \
//       less    between a and b
//        a
//
//! RB tree definition.
//! 1. Red links lean left.
//! 2. No node has two red links connected to it.
//! 3. The tree has perfect black balance : every path from the root to a
//!   null link has the same number of black links.
//!
//! Insert:
// 1. Insert Into a single 2-node (2 cases)
//
//     case1: (insert 'a')
//
//     b   |        b
//         |      /
//         |    R(a)
//
//    ------------------------
//     case2: (inset 'b')
//
//     a   |    a        |        b
//         |      \      |      /
//         |       R(b)  |    R(a)
//         |             |
//              rotate
//               left   =>
//
// 2. Insert into a single 3-node (3 cases)
//
//     case1: larger, insert 'c'
//
//     b  |     b        |      b
//   /    |   /   \      |    /    \
// R(a)   | R(a)   R(c)  |   a      c
//
//           color flip  =>
//
//     case2: smaller, insert 'a'
//
//     c           c           b                b
//   /            /          /   \            /  \
//  R(b)        R(b)       R(a)   R(c)       a    c
//             /
//            R(a)
//
//           rotate           color
//           right    =>       flip    =>
//
//     case3: between, insert 'b'
//
//     c       c          c          b               b
//    /       /          /         /  \            /  \
//  R(a)    R(a)       R(b)     R(a)   R(c)       a    c
//            \        /
//            R(b)    R(a)
//
//          left       right       color
//         rotate  => rotate  =>   flip    =>
//

use crate::tree::binary::node::Color;
use crate::tree::binary::{Node, NodeQuery, Tree};
use std::cmp::Ordering;
use std::ptr::NonNull;

pub trait RedBlackTreeV2<T> {
    fn insert(&mut self, element: T);
    fn delete_min(&mut self);
}

impl<T> RedBlackTreeV2<T> for Tree<T>
where
    T: std::cmp::PartialOrd + std::marker::Copy,
{
    fn insert(&mut self, element: T) {
        let new_root = put(self.root, element);
        NodeQuery::new(new_root).set_color(Color::Black);
        self.root = new_root;
    }

    fn delete_min(&mut self) {
        let mut root = NodeQuery::new(self.root);
        if root.is_some() {
            if !root.left().is_red() && !root.right().is_red() {
                root.set_color(Color::Red);
            }
            root.node = delete_min(root.node);
            root.set_color(Color::Black);
            self.root = root.node;
        }
    }
}

/*
     h                          x
  /      \                  /       \
 A        R(x)     =>     R(h)       C
         /    \         /      \
        B      C       A        B

旋转操作会改变红链接的指向，所以旋转之后 h 变为 R(h)
旋转操作可以保持rb tree的两个重要性质：有序性(中序)和完美平衡性
*/
/// make a right-leaning link lean to the left
fn rotate_left<T>(h: Option<NonNull<Node<T>>>) -> Option<NonNull<Node<T>>> {
    let mut h = NodeQuery::new(h);
    let mut x = h.right();
    h.set_right(x.left().node);
    x.set_left(h.node);
    x.set_color(h.color().unwrap());
    h.set_color(Color::Red);
    x.node
}

/*
       h                x
     /   \            /   \
   R(x)   C    =>    A    R(h)
  /    \                 /     \
 A      B               B       C

*/
/// make a left-leaning link lean to the right
fn rotate_right<T>(h: Option<NonNull<Node<T>>>) -> Option<NonNull<Node<T>>> {
    let mut h = NodeQuery::new(h);
    let mut x = h.left();
    h.set_left(x.right().node);
    x.set_right(h.node);
    x.set_color(h.color().unwrap());
    h.set_color(Color::Red);
    x.node
}

/// flip the colors of a node and its two children
fn flip_colors<T>(h: Option<NonNull<Node<T>>>) {
    let mut h = NodeQuery::new(h);
    h.flip_color();
    h.left().flip_color();
    h.right().flip_color();
}

fn put<T>(h: Option<NonNull<Node<T>>>, element: T) -> Option<NonNull<Node<T>>>
where
    T: std::cmp::PartialOrd + Copy,
{
    let mut h = NodeQuery::new(h);

    match h.get_element() {
        None => return Some(Node::new_leaf(element, None)),
        Some(v) => match element.partial_cmp(&v) {
            None | Some(Ordering::Equal) => (),
            Some(Ordering::Less) => h.set_left(put(h.left().node, element)),
            Some(Ordering::Greater) => h.set_right(put(h.right().node, element)),
        },
    }

    balance(h.node)
}

fn balance<T>(h: Option<NonNull<Node<T>>>) -> Option<NonNull<Node<T>>> {
    let mut h = NodeQuery::new(h);

    if h.right().is_red() && !h.left().is_red() {
        h.node = rotate_left(h.node);
    }

    if h.left().is_red() && h.left().left().is_red() {
        h.node = rotate_right(h.node);
    }

    if h.left().is_red() && h.right().is_red() {
        flip_colors(h.node);
    }

    h.node
}

/// Assuming that h is red and both h.left and h.left.left
/// are black, make h.left or one of its children red.
fn move_red_left<T>(h: Option<NonNull<Node<T>>>) -> Option<NonNull<Node<T>>> {
    let mut h = NodeQuery::new(h);

    flip_colors(h.node);
    if h.right().left().is_red() {
        h.set_right(rotate_right(h.right().node));
        h.node = rotate_left(h.node);
        flip_colors(h.node);
    }

    h.node
}

fn delete_min<T>(h: Option<NonNull<Node<T>>>) -> Option<NonNull<Node<T>>> {
    let mut h = NodeQuery::new(h);

    if h.left().is_none() {
        if let Some(h) = h.node.take() {
            Node::release(h);
        }
        None
    } else {
        if !h.left().is_red() && !h.left().left().is_red() {
            h.node = move_red_left(h.node);
        }
        let new_left = delete_min(h.left().node);
        h.set_left(new_left);
        balance(h.node)
    }
}
