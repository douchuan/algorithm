use crate::ll::Node;
use std::ptr::NonNull;

pub fn reverse<T>(head: Option<NonNull<Node<T>>>) -> Option<NonNull<Node<T>>> {
    unsafe { do_reverse(head) }
}

unsafe fn do_reverse<T>(head: Option<NonNull<Node<T>>>) -> Option<NonNull<Node<T>>> {
    match head {
        None => head,
        Some(head) => match (*head.as_ptr()).next {
            None => Some(head),
            Some(next) => {
                (*next.as_ptr()).next = Some(head);
                do_reverse(Some(next))
            }
        },
    }
}
