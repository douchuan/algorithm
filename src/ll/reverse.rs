use crate::ll::{LinkedList, Node};
use std::ptr::NonNull;

pub fn reverse<T>(l: &mut LinkedList<T>) {
    let head = l.head;
    let new_head = unsafe { do_reverse(head) };
    l.head = new_head;
    l.tail = head;
}

unsafe fn do_reverse<T>(node: Option<NonNull<Node<T>>>) -> Option<NonNull<Node<T>>> {
    node.and_then(|node| match (*node.as_ptr()).next.take() {
        None => Some(node), // new_head, origin tail
        Some(next) => {
            let new_head = do_reverse(Some(next));
            (*next.as_ptr()).next = Some(node);
            new_head
        }
    })
}
