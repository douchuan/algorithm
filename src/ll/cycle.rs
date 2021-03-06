#![allow(unused)]
//! 环形链表
//!
//! 给定一个链表，判断链表中是否有环。

use crate::ll::{LinkedList, Node};
use std::ptr::NonNull;

/// 使用双指针，一个指针每次移动一个节点，一个指针每次移动两个节点，
/// 如果存在环，那么这两个指针一定会相遇。
fn has_cycle<T>(p: Option<NonNull<Node<T>>>) -> bool {
    let mut fast = p;
    let mut slow = p;

    loop {
        unsafe {
            fast = fast.and_then(|v| v.as_ref().next.and_then(|next| next.as_ref().next));
            slow = slow.and_then(|v| v.as_ref().next);
        }

        if fast.is_none() || fast == slow {
            break;
        }
    }

    fast.is_some() && fast == slow
}

#[test]
fn t_has_cycle() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut ll = LinkedList::default();
    for v in data {
        ll.push_back(v);
    }

    //no cycle
    assert!(!has_cycle(ll.head));

    //create cycle by hand
    let mut tail = ll.tail.unwrap();
    unsafe {
        tail.as_mut().next = ll.head;
    }
    assert!(has_cycle(ll.head));

    //eliminate cycle, otherwise LinkedList drop failed
    unsafe {
        tail.as_mut().next = None;
    }
}
