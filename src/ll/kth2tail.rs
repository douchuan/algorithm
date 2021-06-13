//! 链表中倒数最后k个结点
//!
//! 输入一个链表，输出一个链表，该输出链表包含原链表中从倒数第k个结点至尾节点的全部节点。
//! 如果该链表长度小于k，请返回一个长度为 0 的链表。

use crate::ll::{LinkedList, Node};
use std::ptr::NonNull;

pub fn find<T>(l: &LinkedList<T>, k: usize) -> Option<NonNull<Node<T>>> {
    unsafe { do_find(l.head, k) }
}

unsafe fn do_find<T>(node: Option<NonNull<Node<T>>>, mut k: usize) -> Option<NonNull<Node<T>>> {
    let mut p1 = node;
    loop {
        match p1 {
            Some(node) if k > 0 => {
                p1 = node.as_ref().next;
                k -= 1;
            }
            _ => break,
        }
    }

    if k > 0 {
        return None;
    }

    let mut p2 = node;
    loop {
        match p1 {
            Some(node) => {
                p1 = node.as_ref().next;
                p2 = p2.unwrap().as_ref().next;
            }
            None => break,
        }
    }

    p2
}
