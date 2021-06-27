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

use std::ptr::NonNull;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Color {
    Red,
    Black,
}

pub struct Node<T> {
    pub element: T,
    pub left: Option<NonNull<Node<T>>>,
    pub right: Option<NonNull<Node<T>>>,
    pub parent: Option<NonNull<Node<T>>>,
    pub color: Color, // used by red black tree
    pub delta: i32,   // 平衡因子, used by avl tree
}

impl<T> Node<T> {
    pub fn new(
        element: T,
        left: Option<NonNull<Node<T>>>,
        right: Option<NonNull<Node<T>>>,
        parent: Option<NonNull<Node<T>>>,
    ) -> NonNull<Self> {
        let v = Box::new(Node {
            element,
            left,
            right,
            parent,
            color: Color::Red,
            delta: 0,
        });
        Box::leak(v).into()
    }

    pub fn new_leaf(element: T, parent: Option<NonNull<Node<T>>>) -> NonNull<Self> {
        Self::new(element, None, None, parent)
    }

    pub fn from_element(element: T) -> NonNull<Self> {
        Self::new_leaf(element, None)
    }

    pub fn release(node: NonNull<Node<T>>) {
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
impl<T> Node<T> {
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

impl<T> Node<T>
where
    T: std::str::FromStr,
{
    pub fn from_str(v: &str) -> Option<NonNull<Node<T>>> {
        v.parse().ok().map(Self::from_element)
    }
}

// rotate
impl<T> Node<T> {}

/// Node proxy, like jQuery
#[derive(Copy, Clone)]
pub struct NodeQuery<T> {
    pub node: Option<NonNull<Node<T>>>,
}

impl<T> NodeQuery<T> {
    pub fn new(node: Option<NonNull<Node<T>>>) -> Self {
        Self { node }
    }

    /// create NodeQuery from node parent
    pub fn new_parent(node: Option<NonNull<Node<T>>>) -> Self {
        Self::new(node).parent()
    }

    pub fn set_left(&mut self, node: Option<NonNull<Node<T>>>) {
        unsafe {
            if let Some(mut p) = self.node {
                p.as_mut().left = node;
            }

            if let Some(mut p) = node {
                p.as_mut().parent = self.node;
            }
        }
    }

    pub fn set_right(&mut self, node: Option<NonNull<Node<T>>>) {
        unsafe {
            if let Some(mut p) = self.node {
                p.as_mut().right = node;
            }

            if let Some(mut p) = node {
                p.as_mut().parent = self.node;
            }
        }
    }

    pub fn set_children(&mut self, l: Option<NonNull<Node<T>>>, r: Option<NonNull<Node<T>>>) {
        self.set_left(l);
        self.set_right(r);
    }

    pub fn replace(&mut self, node: Option<NonNull<Node<T>>>) {
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

    pub fn set_element(&mut self, element: T) {
        if let Some(mut node) = self.node {
            unsafe { node.as_mut().element = element }
        }
    }

    pub fn set_color(&mut self, v: Color) {
        if let Some(mut node) = self.node {
            unsafe { node.as_mut().color = v }
        }
    }

    pub fn child_count(&self) -> usize {
        self.node.map_or(0, |node| Node::children_count(node))
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

    pub fn color(&self) -> Option<Color> {
        unsafe { self.node.map(|node| node.as_ref().color) }
    }

    pub fn is_red(&self) -> bool {
        self.color() == Some(Color::Red)
    }

    pub fn is_black(&self) -> bool {
        !self.is_red()
    }
}

impl<T> NodeQuery<T>
where
    T: Copy,
{
    pub fn left_element(&self) -> Option<T> {
        self.left().get_element()
    }

    pub fn right_element(&self) -> Option<T> {
        self.right().get_element()
    }

    pub fn get_element(&self) -> Option<T> {
        unsafe { self.node.map(|node| node.as_ref().element) }
    }
}
