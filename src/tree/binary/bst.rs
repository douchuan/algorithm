//! 一棵二叉搜索树是一棵满足下面条件的二叉树
//!   1. 所有左侧分支的值都小于本节点的值
//!   2. 本节点的值小于所有右侧分支的值

use crate::tree::binary::{Node, Tree};
use std::cmp::Ordering;
use std::ptr::NonNull;

pub trait BSTree<T>
where
    T: std::cmp::PartialOrd,
{
    /// return
    ///   Some(NonNull<Node<T>>): insert success and return inserted node,
    ///   None: not insert, exist
    fn insert(&mut self, element: T) -> Option<NonNull<Node<T>>>;
    fn delete(&mut self, element: T) -> bool;
    /// return node index
    fn find(&self, element: T) -> Option<NonNull<Node<T>>>;
    fn min(&self) -> Option<NonNull<Node<T>>>;
    fn max(&self) -> Option<NonNull<Node<T>>>;
    /// 查找后继元素
    fn succ(&self, element: T) -> Option<NonNull<Node<T>>>;
    /// 寻找前驱元素
    fn pred(&self, elementx: T) -> Option<NonNull<Node<T>>>;
}

impl<T> BSTree<T> for Tree<T>
where
    T: std::cmp::PartialOrd + Copy,
{
    fn insert(&mut self, element: T) -> Option<NonNull<Node<T>>> {
        unsafe { insert(self, element, None, self.root) }
    }

    fn delete(&mut self, element: T) -> bool {
        unsafe { delete(element, self.root) }
    }

    fn find(&self, element: T) -> Option<NonNull<Node<T>>> {
        unsafe { find(element, self.root) }
    }

    fn min(&self) -> Option<NonNull<Node<T>>> {
        unsafe { find_min(self.root) }
    }

    fn max(&self) -> Option<NonNull<Node<T>>> {
        unsafe { find_max(self.root) }
    }

    fn succ(&self, element: T) -> Option<NonNull<Node<T>>> {
        unsafe { succ(self.root, element) }
    }

    fn pred(&self, element: T) -> Option<NonNull<Node<T>>> {
        unsafe { pred(self.root, element) }
    }
}

unsafe fn insert<T>(
    tree: &mut Tree<T>,
    element: T,
    parent: Option<NonNull<Node<T>>>,
    node: Option<NonNull<Node<T>>>,
) -> Option<NonNull<Node<T>>>
where
    T: std::cmp::PartialOrd,
{
    match (parent, node) {
        //empty tree
        (None, None) => {
            let node = Node::from_element(element);
            tree.root = Some(node);
            Some(node)
        }
        (_, Some(node)) => match node.as_ref().element.partial_cmp(&element) {
            Some(Ordering::Less) => {
                let r = node.as_ref().right;
                insert(tree, element, Some(node), r)
            }
            Some(Ordering::Greater) => {
                let l = node.as_ref().left;
                insert(tree, element, Some(node), l)
            }
            _ => None,
        },
        (Some(mut node), None) => match node.as_ref().element.partial_cmp(&element) {
            Some(Ordering::Less) => {
                let child = Node::new_leaf(element, Some(node));
                node.as_mut().right = Some(child);
                Some(child)
            }
            Some(Ordering::Greater) => {
                let child = Node::new_leaf(element, Some(node));
                node.as_mut().left = Some(child);
                Some(child)
            }
            _ => None,
        },
    }
}

unsafe fn find<T>(element: T, node: Option<NonNull<Node<T>>>) -> Option<NonNull<Node<T>>>
where
    T: std::cmp::PartialOrd,
{
    node.and_then(|node| match node.as_ref().element.partial_cmp(&element) {
        Some(Ordering::Less) => find(element, node.as_ref().right),
        Some(Ordering::Greater) => find(element, node.as_ref().left),
        Some(Ordering::Equal) => Some(node),
        None => None,
    })
}

unsafe fn find_min<T>(node: Option<NonNull<Node<T>>>) -> Option<NonNull<Node<T>>>
where
    T: std::cmp::PartialOrd,
{
    node.and_then(|node| node.as_ref().left.map_or(Some(node), |l| find_min(Some(l))))
}

unsafe fn find_max<T>(node: Option<NonNull<Node<T>>>) -> Option<NonNull<Node<T>>>
where
    T: std::cmp::PartialOrd,
{
    node.and_then(|node| {
        node.as_ref()
            .right
            .map_or(Some(node), |r| find_max(Some(r)))
    })
}

unsafe fn succ<T>(node: Option<NonNull<Node<T>>>, mut element: T) -> Option<NonNull<Node<T>>>
where
    T: std::cmp::PartialOrd + Copy,
{
    find(element, node).and_then(|node| {
        match node.as_ref().right {
            //右分支的最小值
            Some(r) => find_min(Some(r)),
            None => {
                //右分支为空，向上找
                let mut parent = node.as_ref().parent;
                loop {
                    match Node::right_node(parent) {
                        Some(r) if (*r.as_ptr()).element == element => {
                            let the_parent = parent.unwrap();
                            element = the_parent.as_ref().element;
                            parent = the_parent.as_ref().parent;
                        }
                        _ => return parent,
                    }
                }
            }
        }
    })
}

unsafe fn pred<T>(node: Option<NonNull<Node<T>>>, mut element: T) -> Option<NonNull<Node<T>>>
where
    T: std::cmp::PartialOrd + Copy,
{
    find(element, node).and_then(|node| {
        match (*node.as_ptr()).left {
            //左分支的最大值
            Some(l) => find_max(Some(l)),
            None => {
                //左分支为空，向上找
                let mut parent = node.as_ref().parent;
                loop {
                    match Node::left_node(parent) {
                        Some(l) if l.as_ref().element == element => {
                            let the_parent = parent.unwrap();
                            element = the_parent.as_ref().element;
                            parent = the_parent.as_ref().parent;
                        }
                        _ => return parent,
                    }
                }
            }
        }
    })
}

/// 从二叉搜索树中删除节点 x 的方法如下:
///   如果 x 没有子节点，或者只有一个孩子，直接将 x“切下”;
///   否则，x 有两个孩子，我们用其右子树中的最小值替换掉 x，然后将右子树中的这一最小值“切掉”。
///
/// idx, 起始node
unsafe fn delete<T>(element: T, node: Option<NonNull<Node<T>>>) -> bool
where
    T: Copy + std::cmp::PartialOrd,
{
    find(element, node).map_or(false, |node| {
        match Node::children_count(node) {
            0 => {
                let mut parent = node.as_ref().parent.unwrap();
                if parent.as_ref().left == Some(node) {
                    parent.as_mut().left = None;
                } else if parent.as_ref().right == Some(node) {
                    parent.as_mut().right = None;
                }

                Node::release(node);
                true
            }
            1 => {
                //backup node child
                let child = if node.as_ref().left.is_some() {
                    node.as_ref().left
                } else {
                    node.as_ref().right
                };

                // rm child, setup child node
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
                let min_node = find_min(right).unwrap();
                let min = min_node.as_ref().element;
                (*node.as_ptr()).element = min;

                //右子树中的这一最小值“切掉”
                delete(min, right)
            }
        }
    })
}
