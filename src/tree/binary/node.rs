use std::ptr::NonNull;

pub struct Node<T> {
    pub element: T,
    pub left: Option<NonNull<Node<T>>>,
    pub right: Option<NonNull<Node<T>>>,
    pub parent: Option<NonNull<Node<T>>>,
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
    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    /// 分支节点
    pub fn is_branch(&self) -> bool {
        !self.is_leaf()
    }

    /// 直接子节点个数，不包括孙子...
    pub fn children_count(&self) -> usize {
        match (self.left, self.right) {
            (Some(_), Some(_)) => 2,
            (Some(_), None) | (None, Some(_)) => 1,
            (None, None) => 0,
        }
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
