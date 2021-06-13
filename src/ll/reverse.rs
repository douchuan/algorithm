use crate::ll::Node;
use std::ptr::NonNull;

pub fn reverse<T>(node: Option<NonNull<Node<T>>>) -> Option<NonNull<Node<T>>> {
    unsafe { do_reverse(node) }
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
