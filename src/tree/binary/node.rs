use std::ptr::NonNull;

pub enum Color {
    Red,
    Black,
}

pub struct Node<T> {
    pub element: T,
    pub left: Option<NonNull<Node<T>>>,
    pub right: Option<NonNull<Node<T>>>,
    pub parent: Option<NonNull<Node<T>>>,
    pub color: Color,
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

    /// 一个节点的左右子树都为空，称之为 叶子节点
    pub fn is_leaf(node: NonNull<Self>) -> bool {
        Self::children_count(node) == 0
    }

    /// 分支节点
    pub fn is_branch(node: NonNull<Self>) -> bool {
        !Self::is_leaf(node)
    }

    /// 直接子节点个数，不包括孙子...
    pub fn children_count(node: NonNull<Self>) -> usize {
        unsafe { node.as_ref().left.map_or(0, |_| 1) + node.as_ref().right.map_or(0, |_| 1) }
    }

    pub fn left_node(node: Option<NonNull<Self>>) -> Option<NonNull<Self>> {
        unsafe { node.and_then(|node| node.as_ref().left) }
    }

    pub fn right_node(node: Option<NonNull<Self>>) -> Option<NonNull<Self>> {
        unsafe { node.and_then(|node| node.as_ref().right) }
    }
}

impl<T> Node<T>
where
    T: std::str::FromStr,
{
    pub fn from_str(v: &str) -> Option<NonNull<Node<T>>> {
        v.parse().ok().and_then(|v| Some(Self::from_element(v)))
    }
}
