use crate::common;
use crate::common::Queue;
use std::cmp::Ordering;
use std::ptr::NonNull;

pub struct TST<T> {
    root: Option<NonNull<Node<T>>>,
    n: usize,
}

struct Node<T> {
    c: usize,
    subtries: [Option<NonNull<Node<T>>>; 3],
    val: Option<T>,
}

impl<T> TST<T> {
    pub fn len(&self) -> usize {
        self.n
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// Does this symbol table contain the given key?
    pub fn contains(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    /// Returns all keys in the symbol table as an Queue.
    pub fn keys(&self) -> Queue<String> {
        let mut queue = Queue::default();
        unsafe {
            collect_prefix(self.root, &mut String::new(), &mut queue);
        }
        queue
    }

    /// Returns the value associated with the given key.
    pub fn get(&self, key: &str) -> Option<&T> {
        if key.is_empty() {
            None
        } else {
            unsafe { get_dth(self.root, key, 0).and_then(|p| p.as_ref().val.as_ref()) }
        }
    }

    /// Inserts the key-value pair into the symbol table, overwriting the old value
    /// with the new value if the key is already in the symbol table.
    /// If the value is None, this effectively deletes the key from the symbol table.
    pub fn put(&mut self, key: &str, val: Option<T>) {
        if !self.contains(key) {
            self.n += 1;
        } else if val.is_none() {
            self.n -= 1;
        }
        unsafe {
            self.root = put_dth(self.root, key, val, 0);
        }
    }

    /// Returns all of the keys in the set that start with prefix.
    pub fn keys_with_prefix(&self, prefix: &str) -> Queue<String> {
        let mut queue = Queue::default();
        if !prefix.is_empty() {
            unsafe {
                let x = get_dth(self.root, prefix, 0);
                if let Some(x) = x {
                    let mut prefix = prefix.to_string();
                    if x.as_ref().val.is_some() {
                        queue.enqueue(prefix.clone());
                    }
                    collect_prefix(x.as_ref().mid(), &mut prefix, &mut queue);
                }
            }
        }
        queue
    }

    /// Returns the string in the symbol table that is the longest prefix of
    /// query, or None, if no such string.
    pub fn longest_prefix_of<'a>(&self, query: &'a str) -> Option<&'a str> {
        let mut length = -1;
        let mut x = self.root;
        let mut i = 0;
        while x.is_some() && i < query.len() {
            let p = x.unwrap();
            let c = common::util::byte_at(query, i) as usize;
            x = unsafe {
                match c.cmp(&p.as_ref().c) {
                    Ordering::Less => p.as_ref().left(),
                    Ordering::Greater => p.as_ref().right(),
                    Ordering::Equal => {
                        i += 1;
                        if p.as_ref().val.is_some() {
                            length = i as i32;
                        }
                        p.as_ref().mid()
                    }
                }
            };
        }
        if length == -1 {
            None
        } else {
            query.get(0..length as usize)
        }
    }

    /// Returns all of the keys in the symbol table that match pattern,
    /// where the character '.' is interpreted as a wildcard character.
    pub fn keys_that_match(&self, pattern: &str) -> Queue<String> {
        let mut results = Queue::default();
        let mut prefix = String::new();
        unsafe { collect_match(self.root, &mut prefix, 0, pattern, &mut results) };
        results
    }
}

// return subtrie corresponding to given key
unsafe fn get_dth<T>(x: Option<NonNull<Node<T>>>, key: &str, d: usize) -> Option<NonNull<Node<T>>> {
    x.and_then(|x| {
        let c = common::util::byte_at(key, d) as usize;
        match c.cmp(&x.as_ref().c) {
            Ordering::Less => get_dth(x.as_ref().left(), key, d),
            Ordering::Greater => get_dth(x.as_ref().right(), key, d),
            Ordering::Equal => {
                if d < key.len() - 1 {
                    get_dth(x.as_ref().mid(), key, d + 1)
                } else {
                    Some(x)
                }
            }
        }
    })
}

unsafe fn put_dth<T>(
    x: Option<NonNull<Node<T>>>,
    key: &str,
    val: Option<T>,
    d: usize,
) -> Option<NonNull<Node<T>>> {
    let c = common::util::byte_at(key, d) as usize;
    let mut x = x.unwrap_or_else(|| Node::new(c));
    match c.cmp(&x.as_ref().c) {
        Ordering::Less => {
            let p = put_dth(x.as_ref().left(), key, val, d);
            x.as_mut().set_left(p);
        }
        Ordering::Greater => {
            let p = put_dth(x.as_ref().right(), key, val, d);
            x.as_mut().set_right(p);
        }
        Ordering::Equal => {
            if d < key.len() - 1 {
                let p = put_dth(x.as_ref().mid(), key, val, d + 1);
                x.as_mut().set_mid(p);
            } else {
                x.as_mut().val = val;
            }
        }
    }

    if x.as_ref().is_empty() {
        // x.val is None and subtries all None, just release x itself
        // println!("release x c = {}", x.as_ref().c as u8 as char);
        let _ = Box::from_raw(x.as_ptr());
        None
    } else {
        Some(x)
    }
}

unsafe fn collect_prefix<T>(
    x: Option<NonNull<Node<T>>>,
    prefix: &mut String,
    results: &mut Queue<String>,
) {
    if let Some(x) = x {
        collect_prefix(x.as_ref().left(), prefix, results);
        if x.as_ref().val.is_some() {
            let mut prefix = prefix.clone();
            prefix.push(x.as_ref().c as u8 as char);
            results.enqueue(prefix);
        }
        prefix.push(x.as_ref().c as u8 as char);
        collect_prefix(x.as_ref().mid(), prefix, results);
        let _ = prefix.pop();
        collect_prefix(x.as_ref().right(), prefix, results);
    }
}

unsafe fn collect_match<T>(
    x: Option<NonNull<Node<T>>>,
    prefix: &mut String,
    i: usize,
    pattern: &str,
    results: &mut Queue<String>,
) {
    if let Some(x) = x {
        let c = common::util::byte_at(pattern, i) as usize;
        if c == b'.' as usize || c < x.as_ref().c {
            collect_match(x.as_ref().left(), prefix, i, pattern, results);
        }
        if c == b'.' as usize || c == x.as_ref().c {
            if i == pattern.len() - 1 && x.as_ref().val.is_some() {
                prefix.push(x.as_ref().c as u8 as char);
                results.enqueue(prefix.clone());
                let _ = prefix.pop();
            }
            if i < pattern.len() - 1 {
                prefix.push(x.as_ref().c as u8 as char);
                collect_match(x.as_ref().mid(), prefix, i + 1, pattern, results);
                let _ = prefix.pop();
            }
        }
        if c == b'.' as usize || c > x.as_ref().c {
            collect_match(x.as_ref().right(), prefix, i, pattern, results);
        }
    }
}

impl<T> Default for TST<T> {
    fn default() -> Self {
        Self { root: None, n: 0 }
    }
}

impl<T> Drop for TST<T> {
    fn drop(&mut self) {
        fn visitor<T>(p: Option<NonNull<Node<T>>>) {
            if let Some(p) = p {
                let subtries = unsafe { Box::from_raw(p.as_ptr()).subtries };
                subtries.iter().for_each(|it| visitor(*it));
            }
        }

        let root = self.root.take();
        visitor(root);
        self.n = 0;
    }
}

impl<T> Node<T> {
    fn new(c: usize) -> NonNull<Self> {
        let v = Box::new(Self {
            c,
            subtries: [None; 3],
            val: None,
        });
        Box::leak(v).into()
    }

    // test subtrie whether completely empty
    fn is_empty(&self) -> bool {
        self.val.is_none() && self.subtries.iter().all(|it| it.is_none())
    }

    // left subtries
    fn left(&self) -> Option<NonNull<Node<T>>> {
        self.subtries[0]
    }

    // middle subtries
    fn mid(&self) -> Option<NonNull<Node<T>>> {
        self.subtries[1]
    }

    // right subtries
    fn right(&self) -> Option<NonNull<Node<T>>> {
        self.subtries[2]
    }

    fn set_left(&mut self, x: Option<NonNull<Node<T>>>) {
        self.subtries[0] = x;
    }

    fn set_mid(&mut self, x: Option<NonNull<Node<T>>>) {
        self.subtries[1] = x;
    }

    fn set_right(&mut self, x: Option<NonNull<Node<T>>>) {
        self.subtries[2] = x;
    }
}
