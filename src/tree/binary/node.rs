//
// NODE relation:
//
//              grandparent
//              /         \
//           uncle       parent
//                      /     \
//                  (NODE)    sibling
//                       \
//                      child

use std::ptr;
use std::ptr::NonNull;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Color {
    Red,
    Black,
}

pub struct Node<K, V> {
    pub key: K,
    pub val: Option<V>,
    pub left: Option<NonNull<Node<K, V>>>,
    pub right: Option<NonNull<Node<K, V>>>,
    pub parent: Option<NonNull<Node<K, V>>>,
    pub color: Color, // used by red black tree
    pub delta: i32,   // 平衡因子, used by avl tree
}

impl<K, V> Node<K, V> {
    pub fn new(
        key: K,
        val: Option<V>,
        left: Option<NonNull<Node<K, V>>>,
        right: Option<NonNull<Node<K, V>>>,
        parent: Option<NonNull<Node<K, V>>>,
    ) -> NonNull<Self> {
        let v = Box::new(Node {
            key,
            val,
            left,
            right,
            parent,
            color: Color::Red,
            delta: 0,
        });
        Box::leak(v).into()
    }

    pub fn new_leaf(key: K, val: Option<V>, parent: Option<NonNull<Node<K, V>>>) -> NonNull<Self> {
        Self::new(key, val, None, None, parent)
    }

    pub fn new_key(key: K) -> NonNull<Self> {
        Self::new_leaf(key, None, None)
    }

    pub fn new_entry(key: K, val: V) -> NonNull<Self> {
        Self::new_leaf(key, Some(val), None)
    }

    pub fn release(node: NonNull<Node<K, V>>) {
        unsafe {
            let _ = Box::from_raw(node.as_ptr());
        }
    }

    /// 直接子节点个数，不包括孙子...
    pub fn children_count(node: NonNull<Self>) -> usize {
        unsafe { node.as_ref().left.map_or(0, |_| 1) + node.as_ref().right.map_or(0, |_| 1) }
    }
}

// relation
impl<K, V> Node<K, V> {
    pub fn left(node: Option<NonNull<Self>>) -> Option<NonNull<Self>> {
        unsafe { node.and_then(|node| node.as_ref().left) }
    }

    pub fn right(node: Option<NonNull<Self>>) -> Option<NonNull<Self>> {
        unsafe { node.and_then(|node| node.as_ref().right) }
    }

    pub fn parent(node: Option<NonNull<Self>>) -> Option<NonNull<Self>> {
        unsafe { node.and_then(|node| node.as_ref().parent) }
    }

    pub fn sibling(node: Option<NonNull<Self>>) -> Option<NonNull<Self>> {
        unsafe {
            node.and_then(|node| {
                node.as_ref().parent.and_then(|parent| {
                    if parent.as_ref().left == Some(node) {
                        parent.as_ref().right
                    } else {
                        parent.as_ref().left
                    }
                })
            })
        }
    }

    pub fn uncle(node: Option<NonNull<Self>>) -> Option<NonNull<Self>> {
        unsafe {
            node.and_then(|node| {
                node.as_ref().parent.and_then(|parent| {
                    parent.as_ref().parent.and_then(|grandparent| {
                        if grandparent.as_ref().left == Some(parent) {
                            grandparent.as_ref().right
                        } else {
                            grandparent.as_ref().left
                        }
                    })
                })
            })
        }
    }
}

/// Node proxy, like jQuery
#[derive(Copy, Clone)]
pub struct NodeQuery<K, V> {
    pub node: Option<NonNull<Node<K, V>>>,
}

impl<'a, K: 'a, V: 'a> NodeQuery<K, V> {
    pub fn new(node: Option<NonNull<Node<K, V>>>) -> Self {
        Self { node }
    }

    /// create NodeQuery from node parent
    pub fn new_parent(node: Option<NonNull<Node<K, V>>>) -> Self {
        Self::new(node).parent()
    }

    pub fn set_left(&mut self, node: Option<NonNull<Node<K, V>>>) {
       
            if let Some(mut p) = self.node {
                unsafe { p.as_mut().left = node };
            }

            if let Some(mut p) = node {
                unsafe { p.as_mut().parent = self.node };
            }
        
    }

    pub fn set_right(&mut self, node: Option<NonNull<Node<K, V>>>) {
        
            if let Some(mut p) = self.node {
                unsafe { p.as_mut().right = node };
            }

            if let Some(mut p) = node {
                unsafe { p.as_mut().parent = self.node };
            }
        
    }

    pub fn set_children(&mut self, l: Option<NonNull<Node<K, V>>>, r: Option<NonNull<Node<K, V>>>) {
        self.set_left(l);
        self.set_right(r);
    }

    pub fn replace(&mut self, node: Option<NonNull<Node<K, V>>>) {
        if self.parent().is_none() {
            if let Some(mut node) = node {
                unsafe { node.as_mut().parent = None }
            }
        } else if self.i_am_left() {
            self.parent().set_left(node);
        } else {
            self.parent().set_right(node);
        }
    }

    pub fn set_entry(&mut self, (key, val): (K, Option<V>)) {
        if let Some(mut node) = self.node {
            unsafe {
                node.as_mut().key = key;
                node.as_mut().val = val;
            }
        }
    }

    pub fn set_color(&mut self, v: Color) {
        if let Some(mut node) = self.node {
            unsafe { node.as_mut().color = v }
        }
    }

    pub fn copy_entry(&mut self, src: NonNull<Node<K, V>>) {
        if let Some(mut node) = self.node {
            unsafe {
                ptr::copy_nonoverlapping(&src.as_ref().key, &mut node.as_mut().key, 1);
                ptr::copy_nonoverlapping(&src.as_ref().val, &mut node.as_mut().val, 1);
            }
        }
    }

    pub fn flip_color(&mut self) {
        if let Some(mut node) = self.node {
            unsafe { node.as_mut().color = node.as_ref().color.flip() }
        }
    }

    pub fn color(&self) -> Option<Color> {
        unsafe { self.node.map(|node| node.as_ref().color) }
    }

    pub fn is_red(&self) -> bool {
        self.color() == Some(Color::Red)
    }

    pub fn is_black(&self) -> bool {
        !self.is_red()
    }

    pub fn child_count(&self) -> usize {
        self.node.map_or(0, Node::children_count)
    }

    pub fn is_some(&self) -> bool {
        self.node.is_some()
    }

    pub fn is_none(&self) -> bool {
        self.node.is_none()
    }

    pub fn i_am_left(&self) -> bool {
        self.is_some() && self.parent().left().node == self.node
    }

    pub fn i_am_right(&self) -> bool {
        self.is_some() && self.parent().right().node == self.node
    }

    pub fn is_leaf(&self) -> bool {
        self.node
            .map_or(true, |node| Node::children_count(node) == 0)
    }

    pub fn is_branch(&self) -> bool {
        !self.is_leaf()
    }

    pub fn left(&self) -> Self {
        let v = Node::left(self.node);
        Self::new(v)
    }

    pub fn right(&self) -> Self {
        let v = Node::right(self.node);
        Self::new(v)
    }

    pub fn sibling(&self) -> Self {
        let v = Node::sibling(self.node);
        Self::new(v)
    }

    pub fn parent(&self) -> Self {
        let v = Node::parent(self.node);
        Self::new(v)
    }

    pub fn grandparent(&self) -> Self {
        self.parent().parent()
    }

    pub fn uncle(&self) -> Self {
        let v = Node::uncle(self.node);
        Self::new(v)
    }

    pub fn left_key(&self) -> Option<&K> {
        self.left().get_key()
    }

    pub fn right_key(&self) -> Option<&K> {
        self.right().get_key()
    }

    pub fn get_key(&self) -> Option<&'a K> {
        unsafe { self.node.map(|node| &node.as_ref().key) }
    }

    pub fn get_entry(&self) -> Option<(&K, Option<&V>)> {
        unsafe {
            self.node
                .map(|node| (&node.as_ref().key, node.as_ref().val.as_ref()))
        }
    }
}

impl Color {
    pub fn flip(&self) -> Self {
        match self {
            Color::Red => Color::Black,
            Color::Black => Color::Red,
        }
    }
}
