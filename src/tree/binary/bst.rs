//! 一棵二叉搜索树是一棵满足下面条件的二叉树
//!   1. 所有左侧分支的值都小于本节点的值
//!   2. 本节点的值小于所有右侧分支的值

use crate::tree::binary::{Node, NodeQuery, Tree};
use std::cmp::Ordering;
use std::ptr::NonNull;

pub trait BSTree<T>
where
    T: std::cmp::PartialOrd,
{
    fn insert(&mut self, element: T);
    //todo: make element as &T
    fn delete(&mut self, element: T) -> bool;
    //todo: make element as &T
    fn find(&self, element: T) -> Option<NonNull<Node<T>>>;
    //todo: return Option<&T>
    fn min(&self) -> Option<T>;
    //todo: return Option<&T>
    fn max(&self) -> Option<T>;
    //todo: make element as &T
    /// 查找后继元素
    fn succ(&self, element: T) -> Option<T>;
    //todo: make element as &T
    /// 寻找前驱元素
    fn pred(&self, element: T) -> Option<T>;
}

impl<T> BSTree<T> for Tree<T>
where
    T: std::cmp::PartialOrd + Copy,
{
    fn insert(&mut self, element: T) {
        if let Ok(x) = unsafe { insert(self.root, element) } {
            if self.root.is_none() {
                self.root = Some(x);
            }
        }
    }

    fn delete(&mut self, element: T) -> bool {
        unsafe { delete(element, self.root) }
    }

    fn find(&self, element: T) -> Option<NonNull<Node<T>>> {
        unsafe { find(self.root, element) }
    }

    fn min(&self) -> Option<T> {
        unsafe { find_min(self.root).map(|p| p.as_ref().element) }
    }

    fn max(&self) -> Option<T> {
        unsafe { find_max(self.root).map(|p| p.as_ref().element) }
    }

    fn succ(&self, element: T) -> Option<T> {
        unsafe { succ(self.root, element).map(|p| p.as_ref().element) }
    }

    fn pred(&self, element: T) -> Option<T> {
        unsafe { pred(self.root, element).map(|p| p.as_ref().element) }
    }
}

/// Ok(inserted node)
/// Err(()): element exists
///
/// # Safety
///
/// This is highly unsafe, due to pointer
pub unsafe fn insert<T>(root: Option<NonNull<Node<T>>>, element: T) -> Result<NonNull<Node<T>>, ()>
where
    T: std::cmp::PartialOrd + Copy,
{
    let mut nq = NodeQuery::new(root);
    let mut parent = None;
    while nq.is_some() {
        parent = nq.node;
        match element.partial_cmp(nq.get_element().as_ref().unwrap()) {
            Some(Ordering::Less) => nq = nq.left(),
            Some(Ordering::Greater) => nq = nq.right(),
            _ => return Err(()),
        }
    }

    //插入x
    let mut x = Node::from_element(element);
    if let Some(mut node) = parent {
        if element < node.as_ref().element {
            node.as_mut().left = Some(x);
        } else {
            node.as_mut().right = Some(x);
        }
        x.as_mut().parent = parent;
    }

    Ok(x)
}

/// # Safety
///
/// This is highly unsafe, due to pointer
pub unsafe fn find<T>(node: Option<NonNull<Node<T>>>, element: T) -> Option<NonNull<Node<T>>>
where
    T: std::cmp::PartialOrd,
{
    node.and_then(|node| match node.as_ref().element.partial_cmp(&element) {
        Some(Ordering::Less) => find(node.as_ref().right, element),
        Some(Ordering::Greater) => find(node.as_ref().left, element),
        Some(Ordering::Equal) => Some(node),
        None => None,
    })
}

/// # Safety
///
/// This is highly unsafe, due to pointer
pub unsafe fn find_min<T>(node: Option<NonNull<Node<T>>>) -> Option<NonNull<Node<T>>>
where
    T: std::cmp::PartialOrd,
{
    node.and_then(|node| node.as_ref().left.map_or(Some(node), |l| find_min(Some(l))))
}

/// # Safety
///
/// This is highly unsafe, due to pointer
pub unsafe fn find_max<T>(node: Option<NonNull<Node<T>>>) -> Option<NonNull<Node<T>>>
where
    T: std::cmp::PartialOrd,
{
    node.and_then(|node| {
        node.as_ref()
            .right
            .map_or(Some(node), |r| find_max(Some(r)))
    })
}

/// # Safety
///
/// This is highly unsafe, due to pointer
unsafe fn succ<T>(p: Option<NonNull<Node<T>>>, mut element: T) -> Option<NonNull<Node<T>>>
where
    T: std::cmp::PartialOrd + Copy,
{
    find(p, element).and_then(|node| {
        let mut nq = NodeQuery::new(Some(node));
        match nq.right().node {
            //右分支的最小值
            Some(r) => find_min(Some(r)),
            None => {
                //右分支为空，向上找
                loop {
                    nq = nq.parent();
                    match nq.right_element() {
                        Some(r) if r == element => element = nq.get_element().unwrap(),
                        _ => return nq.node,
                    }
                }
            }
        }
    })
}

/// # Safety
///
/// This is highly unsafe, due to pointer
unsafe fn pred<T>(node: Option<NonNull<Node<T>>>, mut element: T) -> Option<NonNull<Node<T>>>
where
    T: std::cmp::PartialOrd + Copy,
{
    find(node, element).and_then(|node| {
        let mut nq = NodeQuery::new(Some(node));
        match nq.left().node {
            //左分支的最大值
            Some(l) => find_max(Some(l)),
            None => {
                //左分支为空，向上找
                loop {
                    nq = nq.parent();
                    match nq.left_element() {
                        Some(l) if l == element => element = nq.get_element().unwrap(),
                        _ => return nq.node,
                    }
                }
            }
        }
    })
}

/// # Safety
///
/// This is highly unsafe, due to pointer
/// 从二叉搜索树中删除节点 x 的方法如下:
///   如果 x 没有子节点，或者只有一个孩子，直接将 x“切下”;
///   否则，x 有两个孩子，我们用其右子树中的最小值替换掉 x，然后将右子树中的这一最小值“切掉”。
///
/// idx, 起始node
unsafe fn delete<T>(element: T, node: Option<NonNull<Node<T>>>) -> bool
where
    T: Copy + std::cmp::PartialOrd,
{
    find(node, element).map_or(false, |mut node| {
        match Node::children_count(node) {
            0 => {
                //leaf
                let mut parent = node.as_ref().parent.unwrap();
                let _ = if parent.as_ref().left == Some(node) {
                    parent.as_mut().left.take()
                } else {
                    parent.as_mut().right.take()
                };

                Node::release(node);
                true
            }
            1 => {
                // backup node child
                let child = if node.as_ref().left.is_some() {
                    node.as_mut().left.take()
                } else {
                    node.as_mut().right.take()
                };

                // setup child node
                let mut parent = node.as_ref().parent.unwrap();
                if parent.as_ref().left == Some(node) {
                    parent.as_mut().left = child;
                } else if parent.as_ref().right == Some(node) {
                    parent.as_mut().right = child;
                }

                Node::release(node);
                true
            }
            _ => {
                //我们用其右子树中的最小值替换掉 x
                let right = node.as_ref().right;
                let min = find_min(right).unwrap().as_ref().element;
                node.as_mut().element = min;

                //右子树中的这一最小值“切掉”
                delete(min, right)
            }
        }
    })
}

/// is the tree rooted at x a BST with all keys strictly between min and max
/// (if min or max is null, treat as empty constraint)
pub fn is_bst<T>(x: Option<NonNull<Node<T>>>, min: Option<T>, max: Option<T>) -> bool
where
    T: Copy + std::cmp::PartialOrd,
{
    if x.is_none() {
        true
    } else {
        let node = NodeQuery::new(x);
        let element = node.get_element();
        if (min.is_some() && element.lt(&min)) || (max.is_some() && element.gt(&max)) {
            false
        } else {
            is_bst(node.left().node, min, element) && is_bst(node.right().node, element, max)
        }
    }
}
