//! 链表中倒数最后k个结点
//!
//! 输入一个链表，输出一个链表，该输出链表包含原链表中从倒数第k个结点至尾节点的全部节点。
//! 如果该链表长度小于k，请返回一个长度为 0 的链表。

use crate::ll::{LinkedList, Node};
use std::ptr::NonNull;

pub fn find<T>(l: &LinkedList<T>, k: usize) -> Option<NonNull<Node<T>>> {
    unsafe { do_find(l.head, k) }
}

/// 设链表的长度为 N。设置两个指针 P1 和 P2，先让 P1 移动 K 个节点，
/// 则还有 N - K 个节点可以移动。此时让 P1 和 P2 同时移动，可以知道
/// 当 P1 移动到链表结尾时，P2 移动到第 N - K 个节点处，该位置就是
/// 倒数第 K 个节点。
unsafe fn do_find<T>(node: Option<NonNull<Node<T>>>, mut k: usize) -> Option<NonNull<Node<T>>> {
    let mut p1 = node;

    // p1 move k nodes
    while p1.is_some() && k > 0 {
        p1 = p1.unwrap().as_ref().next;
        k -= 1;
    }
    if k > 0 {
        return None;
    }

    // p2 移动到第 N - K 个节点处
    let mut p2 = node;
    while p1.is_some() {
        p1 = p1.unwrap().as_ref().next;
        p2 = p2.unwrap().as_ref().next;
    }

    p2
}
