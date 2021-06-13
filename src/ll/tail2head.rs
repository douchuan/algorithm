//! 从尾到头打印链表
//!
//! 输入一个链表，按链表从尾到头的顺序返回一个ArrayList。
//!
//! 输入：
//!   {1,2,3}
//! 返回值：
//!   [3,2,1]

use crate::ll::LinkedList;

pub fn print<T>(ll: &LinkedList<T>) -> Vec<T>
where
    T: Copy,
{
    let mut stack = Vec::with_capacity(ll.len());
    let mut p = ll.head;
    loop {
        match p {
            Some(node) => {
                let element = unsafe { node.as_ref().element };
                stack.push(element);
                p = unsafe { node.as_ref().next };
            }
            None => break,
        }
    }

    let mut res = Vec::with_capacity(stack.len());
    loop {
        match stack.pop() {
            Some(element) => res.push(element),
            None => break,
        }
    }

    res
}
