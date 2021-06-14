//! 环形链表
//!
//! 给定一个链表，判断链表中是否有环。

use crate::ll::Node;
use std::ptr::NonNull;

/// 使用双指针，一个指针每次移动一个节点，一个指针每次移动两个节点，
/// 如果存在环，那么这两个指针一定会相遇。
pub fn has_cycle<T>(p: Option<NonNull<Node<T>>>) -> bool {
    let mut fast = p;
    let mut slow = p;

    unsafe {
        loop {
            fast = fast.and_then(|v| v.as_ref().next.and_then(|next| next.as_ref().next));
            slow = slow.and_then(|v| v.as_ref().next);

            if fast.is_none() || fast == slow {
                break;
            }
        }
    }

    return fast.is_some() && fast == slow;
}
