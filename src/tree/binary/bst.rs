//! 一棵二叉搜索树是一棵满足下面条件的二叉树
//!   1. 所有左侧分支的值都小于本节点的值
//!   2. 本节点的值小于所有右侧分支的值

use crate::tree::binary::{Node, NodeQuery, Tree};
use std::cmp::Ordering;
use std::ptr::NonNull;

pub trait BSTree<K, V>
where
    K: Ord,
{
    fn insert(&mut self, key: K, val: V);
    fn delete(&mut self, key: &K);
    fn find(&self, key: &K) -> Option<&V>;
    fn min(&self) -> Option<&V>;
    fn max(&self) -> Option<&V>;
    /// 查找后继元素
    fn succ(&self, key: &K) -> Option<&V>;
    /// 寻找前驱元素
    fn pred(&self, key: &K) -> Option<&V>;
}

impl<K, V> BSTree<K, V> for Tree<K, V>
where
    K: Ord,
{
    fn insert(&mut self, key: K, val: V) {
        if let Ok(x) = unsafe { insert(self.root, key, val) } {
            if self.root.is_none() {
                self.root = Some(x);
            }
        }
    }

    fn delete(&mut self, key: &K) {
        unsafe { delete(key, self.root) }
    }

    fn find(&self, key: &K) -> Option<&V> {
        unsafe { find(self.root, key).and_then(|p| p.as_ref().val.as_ref()) }
    }

    fn min(&self) -> Option<&V> {
        unsafe { find_min(self.root).and_then(|p| p.as_ref().val.as_ref()) }
    }

    fn max(&self) -> Option<&V> {
        unsafe { find_max(self.root).and_then(|p| p.as_ref().val.as_ref()) }
    }

    fn succ(&self, key: &K) -> Option<&V> {
        unsafe { succ(self.root, key).and_then(|p| p.as_ref().val.as_ref()) }
    }

    fn pred(&self, key: &K) -> Option<&V> {
        unsafe { pred(self.root, key).and_then(|p| p.as_ref().val.as_ref()) }
    }
}

/// Ok(inserted node)
/// Err(()): element exists
///
/// # Safety
///
/// This is highly unsafe, due to pointer
pub unsafe fn insert<K, V>(
    root: Option<NonNull<Node<K, V>>>,
    key: K,
    val: V,
) -> Result<NonNull<Node<K, V>>, ()>
where
    K: Ord,
{
    let mut nq = NodeQuery::new(root);
    let mut parent = None;
    while nq.is_some() {
        parent = nq.node;
        match key.cmp(nq.get_key().unwrap()) {
            Ordering::Less => nq = nq.left(),
            Ordering::Greater => nq = nq.right(),
            _ => return Err(()),
        }
    }

    //插入x
    let mut x;
    if let Some(mut node) = parent {
        if key < node.as_ref().key {
            x = Node::new_entry(key, val);
            node.as_mut().left = Some(x);
        } else {
            x = Node::new_entry(key, val);
            node.as_mut().right = Some(x);
        }
        x.as_mut().parent = parent;
    } else {
        x = Node::new_entry(key, val);
    }

    Ok(x)
}

/// # Safety
///
/// This is highly unsafe, due to pointer
pub unsafe fn find<K, V>(node: Option<NonNull<Node<K, V>>>, key: &K) -> Option<NonNull<Node<K, V>>>
where
    K: Ord,
{
    node.and_then(|node| match node.as_ref().key.cmp(key) {
        Ordering::Less => find(node.as_ref().right, key),
        Ordering::Greater => find(node.as_ref().left, key),
        Ordering::Equal => Some(node),
    })
}

/// # Safety
///
/// This is highly unsafe, due to pointer
pub unsafe fn find_min<K, V>(node: Option<NonNull<Node<K, V>>>) -> Option<NonNull<Node<K, V>>>
where
    K: Ord,
{
    node.and_then(|node| node.as_ref().left.map_or(Some(node), |l| find_min(Some(l))))
}

/// # Safety
///
/// This is highly unsafe, due to pointer
pub unsafe fn find_max<K, V>(node: Option<NonNull<Node<K, V>>>) -> Option<NonNull<Node<K, V>>>
where
    K: Ord,
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
unsafe fn succ<'a, K: 'a, V: 'a>(
    p: Option<NonNull<Node<K, V>>>,
    mut key: &'a K,
) -> Option<NonNull<Node<K, V>>>
where
    K: Ord,
{
    find(p, key).and_then(|node| {
        let mut nq = NodeQuery::new(Some(node));
        match nq.right().node {
            //右分支的最小值
            Some(r) => find_min(Some(r)),
            None => {
                //右分支为空，向上找
                loop {
                    nq = nq.parent();
                    match nq.right_key() {
                        Some(r) if r == key => key = nq.get_key().unwrap(),
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
unsafe fn pred<'a, K: 'a, V: 'a>(
    node: Option<NonNull<Node<K, V>>>,
    mut key: &'a K,
) -> Option<NonNull<Node<K, V>>>
where
    K: Ord,
{
    find(node, key).and_then(|node| {
        let mut nq = NodeQuery::new(Some(node));
        match nq.left().node {
            //左分支的最大值
            Some(l) => find_max(Some(l)),
            None => {
                //左分支为空，向上找
                loop {
                    nq = nq.parent();
                    match nq.left_key() {
                        Some(l) if l == key => key = nq.get_key().unwrap(),
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
unsafe fn delete<K, V>(key: &K, node: Option<NonNull<Node<K, V>>>)
where
    K: Ord,
{
    find(node, key).map_or((), |mut node| {
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
            }
            _ => {
                //我们用其右子树中的最小值替换掉 x
                let right = node.as_ref().right;
                let min = find_min(right);
                NodeQuery::new(Some(node)).copy_entry(min.unwrap());

                //右子树中的这一最小值“切掉”
                delete(&min.unwrap().as_ref().key, right)
            }
        }
    });
}

/// is the tree rooted at x a BST with all keys strictly between min and max
/// (if min or max is null, treat as empty constraint)
pub fn is_bst<K, V>(x: Option<NonNull<Node<K, V>>>, min: Option<&K>, max: Option<&K>) -> bool
where
    K: Ord,
{
    x.map_or(true, |x| {
        let node = NodeQuery::new(Some(x));
        let key = node.get_key();
        if (min.is_some() && key.lt(&min)) || (max.is_some() && key.gt(&max)) {
            false
        } else {
            is_bst(node.left().node, min, key) && is_bst(node.right().node, key, max)
        }
    })
}
