//! A generic stack, implemented using a singly linked list.

use crate::ll::linked_list::Iter;
use crate::ll::LinkedList;

pub struct Stack<T> {
    ll: LinkedList<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        let ll = LinkedList::default();
        Self { ll }
    }

    /// Adds the item to this stack.
    pub fn push(&mut self, v: T) {
        self.ll.push_front(v);
    }

    /// Removes and returns the item most recently added to this stack.
    pub fn pop(&mut self) -> Option<T> {
        self.ll.pop_front()
    }

    /// Returns (but does not remove) the item most recently added to this stack.
    pub fn peek(&self) -> Option<&T> {
        self.ll.first()
    }

    /// Returns an iterator to this stack that iterates through the items in LIFO order.
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
