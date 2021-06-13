use crate::ll::Node;
use std::ptr::NonNull;

pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        unsafe { do_drop(self.head) }
    }
}

unsafe fn do_drop<T>(head: Option<NonNull<Node<T>>>) {
    let mut p = head;
    loop {
        match p {
            Some(node) => {
                let node = Box::from_raw(node.as_ptr());
                p = node.next;
            }
            None => break,
        }
    }
}
