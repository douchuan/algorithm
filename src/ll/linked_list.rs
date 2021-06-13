use crate::ll::Node;
use std::ptr::NonNull;

pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
}

impl<T> LinkedList<T> {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, node: NonNull<Node<T>>) {
        unsafe {
            (*node.as_ptr()).next = None;

            match self.tail {
                Some(tail) => (*tail.as_ptr()).next = Some(node),
                None => self.head = Some(node),
            }

            self.tail = Some(node);
            self.len += 1;
        }
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        unsafe { do_drop(self.head) }
        self.head = None;
        self.len = 0;
    }
}

unsafe fn do_drop<T>(head: Option<NonNull<Node<T>>>) {
    let mut p = head;
    loop {
        match p {
            Some(node) => {
                let mut node = Box::from_raw(node.as_ptr());
                p = node.next.take();
            }
            None => break,
        }
    }
}
