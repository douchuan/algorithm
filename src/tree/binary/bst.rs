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
    fn get(&self, key: &K) -> Option<&V>;
    fn min(&self) -> Option<&K>;
    fn max(&self) -> Option<&K>;
    /// 查找后继元素
    fn succ(&self, key: &K) -> Option<&K>;
    /// 寻找前驱元素
    fn pred(&self, key: &K) -> Option<&K>;
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
        let node = unsafe { find(self.root, key) };
        let new_root = unsafe { delete(self.root, node) };
        self.root = new_root;
    }

    fn get(&self, key: &K) -> Option<&V> {
        unsafe { find(self.root, key).and_then(|p| p.as_ref().val.as_ref()) }
    }

    fn min(&self) -> Option<&K> {
        unsafe { find_min(self.root).map(|p| &p.as_ref().key) }
    }

    fn max(&self) -> Option<&K> {
        unsafe { find_max(self.root).map(|p| &p.as_ref().key) }
    }

    fn succ(&self, key: &K) -> Option<&K> {
        unsafe { succ(self.root, key).map(|p| &p.as_ref().key) }
    }

    fn pred(&self, key: &K) -> Option<&K> {
        unsafe { pred(self.root, key).map(|p| &p.as_ref().key) }
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
            Ordering::Equal => {
                nq.set_entry((key, Some(val))); // update val
                return Err(());
            }
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
unsafe fn delete<K, V>(
    mut root: Option<NonNull<Node<K, V>>>,
    mut x: Option<NonNull<Node<K, V>>>,
) -> Option<NonNull<Node<K, V>>>
where
    K: Ord,
{
    if let Some(mut px) = x {
        let old_x = x;
        let parent = px.as_ref().parent;

        if px.as_ref().left.is_none() {
            x = px.as_ref().right;
        } else if px.as_ref().right.is_none() {
            x = px.as_ref().left;
        } else {
            let min = find_min(px.as_ref().right).unwrap();
            NodeQuery::new(x).copy_entry(min);
            let min_parent = min.as_ref().parent;
            if min_parent != x {
                let min_right = min.as_ref().right;
                min_parent.unwrap().as_mut().left = min_right;
            } else {
                px.as_mut().right = min.as_ref().right;
            }
            if let Some(mut min_right) = min.as_ref().right {
                let min_parent = min.as_ref().parent;
                min_right.as_mut().parent = min_parent;
            }
            Node::release(min);
            return root;
        }

        if let Some(mut px) = x {
            px.as_mut().parent = parent;
        }

        if let Some(mut parent) = parent {
            if parent.as_ref().left == old_x {
                parent.as_mut().left = x;
            } else {
                parent.as_mut().right = x;
            }
        } else {
            root = x;
        }

        Node::release(old_x.unwrap());

        root
    } else {
        root
    }
}

/// is the tree rooted at x a BST with all keys strictly between min and max
/// (if min or max is null, treat as empty constraint)
pub fn is_bst<K, V>(x: Option<NonNull<Node<K, V>>>, min: Option<&K>, max: Option<&K>) -> bool
where
    K: Ord,
{
    x.map_or(true, |x| {
        let x = NodeQuery::new(Some(x));
        let key = x.get_key();
        if (min.is_some() && key.lt(&min)) || (max.is_some() && key.gt(&max)) {
            false
        } else {
            is_bst(x.left().node, min, key) && is_bst(x.right().node, key, max)
        }
    })
}

/// Returns the number of key-value pairs
pub fn calc_size<K, V>(x: Option<NonNull<Node<K, V>>>) -> usize {
    x.map_or(0, |x| unsafe {
        1 + calc_size(x.as_ref().left) + calc_size(x.as_ref().right)
    })
}

/// add the keys between lo and hi in the subtree rooted at x
/// to the queue
pub fn keys<'a, K: 'a, V: 'a>(
    x: Option<NonNull<Node<K, V>>>,
    queue: &mut Vec<&'a K>,
    lo: &K,
    hi: &K,
) where
    K: Ord,
{
    let x = NodeQuery::new(x);
    if x.is_some() {
        let xkey = x.get_key().unwrap();
        let cmplo = lo.cmp(xkey);
        let cmphi = hi.cmp(xkey);
        if cmplo.is_lt() {
            keys(x.left().node, queue, lo, hi);
        }
        if cmplo.is_le() && cmphi.is_ge() {
            queue.push(xkey);
        }
        if cmphi.is_gt() {
            keys(x.right().node, queue, lo, hi);
        }
    }
}
