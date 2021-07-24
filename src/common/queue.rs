//! The Queue represents a first-in-first-out (FIFO)
//! queue of generic items.
//! It supports the usual enqueue and dequeue
//! operations, along with methods for peeking at the first item,
//! testing if the queue is empty, and iterating through
//! the items in FIFO order.
//! This implementation uses a singly linked list with a static nested class for
//! linked-list nodes.
//! The enqueue, dequeue, peek, size, and is-empty
//! operations all take constant time in the worst case.

use crate::ll::linked_list::Iter;
use crate::ll::LinkedList;

pub struct Queue<T> {
    ll: LinkedList<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        let ll = LinkedList::default();
        Self { ll }
    }

    pub fn enqueue(&mut self, v: T) {
        self.ll.push_back(v);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.ll.pop_front()
    }

    pub fn peek(&self) -> Option<&T> {
        self.ll.first()
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.ll.iter()
    }

    pub fn len(&self) -> usize {
        self.ll.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ll.is_empty()
    }
}
