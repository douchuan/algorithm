use crate::ll::reverse::do_reverse;
use std::marker::PhantomData;
use std::ptr::NonNull;

pub struct Node<T> {
    pub element: T,
    pub next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(element: T) -> NonNull<Self> {
        let v = Box::new(Node {
            element,
            next: None,
        });
        Box::leak(v).into()
    }
}

pub struct LinkedList<T> {
    pub head: Option<NonNull<Node<T>>>,
    pub tail: Option<NonNull<Node<T>>>,
    len: usize,
}

pub struct Iter<'a, T: 'a> {
    head: Option<NonNull<Node<T>>>,
    // tail: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<&'a Node<T>>,
}

pub struct IntoIter<T> {
    list: LinkedList<T>,
}

impl<T> LinkedList<T> {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn push_back(&mut self, element: T) {
        let mut node = Node::new(element);
        unsafe {
            node.as_mut().next = None;

            match self.tail {
                Some(mut tail) => tail.as_mut().next = Some(node),
                None => self.head = Some(node),
            }
        }

        self.tail = Some(node);
        self.len += 1;
    }

    pub fn push_front(&mut self, element: T) {
        let mut node = Node::new(element);
        unsafe {
            node.as_mut().next = self.head;
        }

        if self.head.is_none() {
            self.tail = Some(node);
        }

        self.head = Some(node);
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        unsafe {
            self.head.map(|head| {
                let node = Box::from_raw(head.as_ptr());
                self.head = node.next;

                if self.head.is_none() {
                    self.tail = None;
                }

                self.len -= 1;
                node.element
            })
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            head: self.head,
            // tail: self.tail,
            len: self.len,
            marker: PhantomData,
        }
    }

    pub fn first(&self) -> Option<&T> {
        unsafe { self.head.map(|node| &node.as_ref().element) }
    }

    pub fn last(&self) -> Option<&T> {
        unsafe { self.tail.map(|node| &node.as_ref().element) }
    }

    /// 反转链表
    pub fn reverse(&mut self) {
        let head = self.head;
        let new_head = unsafe { do_reverse(head) };
        self.head = new_head;
        self.tail = head;
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    /// Consumes the list into an iterator yielding elements by value.
    #[inline]
    fn into_iter(self) -> IntoIter<T> {
        IntoIter { list: self }
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

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<&'a T> {
        if self.len == 0 {
            None
        } else {
            self.head.map(|node| unsafe {
                // Need an unbound lifetime to get 'a
                let node = &*node.as_ptr();
                self.len -= 1;
                self.head = node.next;
                &node.element
            })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.list.pop_front()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.list.len, Some(self.list.len))
    }
}

unsafe fn do_drop<T>(head: Option<NonNull<Node<T>>>) {
    let mut p = head;
    while let Some(node) = p {
        let mut node = Box::from_raw(node.as_ptr());
        p = node.next.take();
    }
}
