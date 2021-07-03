//! 反转链表
//!
//! 输入一个链表，反转链表后，输出新链表的表头。

use crate::ll::Node;
use std::ptr::NonNull;

// 递归，先找到tail，然后从后向前修正指针
pub(crate) unsafe fn do_reverse<T>(node: Option<NonNull<Node<T>>>) -> Option<NonNull<Node<T>>> {
    node.and_then(|mut node| match node.as_mut().next.take() {
        None => Some(node), // new_head, origin tail
        Some(mut next) => {
            let new_head = do_reverse(Some(next));
            next.as_mut().next = Some(node);
            new_head
        }
    })
}
