//!
//! Proposition H.
//! The average number of nodes examined for search miss in a trie
//! built from N random keys over an alphabet of size R is ~ log(N, R).
//!
//! From a practical standpoint, the most important implication of this
//! proposition is that search miss does not depend on the key length.
//! For example, it says that unsuccessful search in a trie built with 1
//! million random keys will require examining only three or four nodes,
//! whether the keys are 7-digit license plates or 20-digit account numbers.
//!
//! Proposition I.
//! The number of links in a trie is between R*N and R*N*w, where w is
//! the average key length.
//!
//! It illustrates the following rules of thumb for tries:
//!  - When keys are short, the number of links is close to R*N.
//!  - When keys are long, the number of links is close to R*N*w.
//!  - Therefore, decreasing R can save a huge amount of space.
//!
//! for large numbers of long keys taken from large alphabets,
//! because it will require space proportional to R times the total
//! number of key characters. Otherwise, if you can afford the space,
//! trie performance is difficult to beat.

use crate::common;
use crate::common::Queue;
use std::ptr::NonNull;

const R: usize = 256;

/// The TrieST represents an symbol table of key-value
/// pairs, with string keys and generic values.
/// It supports the usual put, get, contains,
/// delete, len, and is-empty, methods.
/// It also provides character-based methods for finding the string
///  in the symbol table that is the *longest prefix* of a given prefix,
///  finding all strings in the symbol table that s*tart with* a given prefix,
/// and finding all strings in the symbol table that *match* a given pattern.
/// A symbol table implements the *associative array* abstraction:
/// when associating a value with a key that is already in the symbol table,
/// the convention is to replace the old value with the new value.
/// This struct uses the convention that
/// values cannot be None—setting the
/// value associated with a key to None is equivalent to deleting the key
/// from the symbol table.
/// This implementation uses a 256-way trie.
/// The put, contains, delete, and
/// longest prefix operations take time proportional to the length
/// of the key (in the worst case). Construction takes constant time.
/// The len, and is-empty operations take constant time.
/// Construction takes constant time.
pub struct TrieST<T> {
    root: Option<NonNull<Node<T>>>,
    n: usize,
}

struct Node<T> {
    val: Option<T>,
    next: Vec<Option<NonNull<Node<T>>>>,
}

impl<T> TrieST<T> {
    /// Returns the number of key-value pairs in this symbol table.
    pub fn len(&self) -> usize {
        self.n
    }

    /// Is this symbol table empty
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// Does this symbol table contain the given key?
    pub fn contains(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    /// Returns the value associated with the given key.
    pub fn get(&self, key: &str) -> Option<&T> {
        get_dth(self.root, key, 0).and_then(|p| unsafe { p.as_ref().val.as_ref() })
    }

    /// Inserts the key-value pair into the symbol table, overwriting
    /// the old value with the new value if the key is already in the
    /// symbol table. If the value is None, this effectively
    /// deletes the key from the symbol table.
    pub fn put(&mut self, key: &str, val: Option<T>) {
        if val.is_none() {
            self.delete(key);
        } else {
            let mut root = self.root;
            root = unsafe { self.put_dth(root, key, val, 0) };
            self.root = root;
        }
    }

    /// Removes the key from the set if the key is present
    pub fn delete(&mut self, key: &str) {
        let mut root = self.root;
        root = unsafe { self.delete_dth(root, key, 0) };
        self.root = root;
    }

    /// Returns all keys in the symbol table
    pub fn keys(&self) -> Queue<String> {
        self.keys_with_prefix("")
    }

    /// Returns all of the keys in the set that start with *prefix*
    pub fn keys_with_prefix(&self, prefix: &str) -> Queue<String> {
        let mut results = Queue::default();
        let x = get_dth(self.root, prefix, 0);
        let mut prefix = prefix.to_string();
        unsafe { collect_prefix(x, &mut prefix, &mut results) };
        results
    }

    /// Returns all of the keys in the symbol table that match *pattern*,
    /// where the character '.' is interpreted as a wildcard character.
    pub fn keys_that_match(&self, pattern: &str) -> Queue<String> {
        let mut results = Queue::default();
        let mut prefix = String::new();
        unsafe { collect_match(self.root, &mut prefix, pattern, &mut results) };
        results
    }

    /// Returns the string in the symbol table that is the longest prefix of *query*,
    /// or None, if no such string.
    pub fn longest_prefix_of<'a>(&self, query: &'a str) -> Option<&'a str> {
        let root = self.root;
        let length = unsafe { longest_prefix_of_dth(root, query, 0, -1) };
        if length == -1 {
            None
        } else {
            Some(&query[0..length as usize])
        }
    }

    unsafe fn put_dth(
        &mut self,
        x: Option<NonNull<Node<T>>>,
        key: &str,
        val: Option<T>,
        d: usize,
    ) -> Option<NonNull<Node<T>>> {
        let mut x = x.unwrap_or_else(|| Node::new(None));

        if d == key.len() {
            if x.as_ref().val.is_none() {
                self.n += 1;
            }
            x.as_mut().val = val;
        } else {
            let i = common::util::byte_at(key, d);
            let next = x.as_ref().next[i];
            x.as_mut().next[i] = self.put_dth(next, key, val, d + 1);
        }

        Some(x)
    }

    unsafe fn delete_dth(
        &mut self,
        x: Option<NonNull<Node<T>>>,
        key: &str,
        d: usize,
    ) -> Option<NonNull<Node<T>>> {
        x.and_then(|mut x| {
            if d == key.len() {
                let val = x.as_mut().val.take();
                if val.is_some() {
                    self.n -= 1;
                }
            } else {
                let i = common::util::byte_at(key, d);
                let next = x.as_ref().next[i];
                x.as_mut().next[i] = self.delete_dth(next, key, d + 1);
            }

            // remove subtrie rooted at x if it is completely empty
            if x.as_ref().val.is_some() {
                return Some(x);
            }
            if x.as_ref().next.iter().any(|it| it.is_some()) {
                return Some(x);
            }

            // x.val is None and next all None, just release x itself
            let _ = Box::from_raw(x.as_ptr());

            None
        })
    }
}

fn get_dth<T>(x: Option<NonNull<Node<T>>>, key: &str, d: usize) -> Option<NonNull<Node<T>>> {
    x.and_then(|x| {
        if d == key.len() {
            Some(x)
        } else {
            let i = common::util::byte_at(key, d);
            let next = unsafe { x.as_ref().next[i] };
            get_dth(next, key, d + 1)
        }
    })
}

unsafe fn collect_prefix<T>(
    x: Option<NonNull<Node<T>>>,
    prefix: &mut String,
    results: &mut Queue<String>,
) {
    if let Some(x) = x {
        if x.as_ref().val.is_some() {
            results.enqueue(prefix.to_string());
        }
        for c in 0..R {
            prefix.push(c as u8 as char);
            collect_prefix(x.as_ref().next[c], prefix, results);
            let _ = prefix.pop();
        }
    }
}

unsafe fn collect_match<T>(
    x: Option<NonNull<Node<T>>>,
    prefix: &mut String,
    pattern: &str,
    results: &mut Queue<String>,
) {
    if let Some(x) = x {
        let d = prefix.len();
        if d == pattern.len() && x.as_ref().val.is_some() {
            results.enqueue(prefix.clone());
        }
        if d == pattern.len() {
            return;
        }
        let i = common::util::byte_at(pattern, d);
        if i == b'.' as usize {
            for ch in 0..R {
                prefix.push(ch as u8 as char);
                collect_match(x.as_ref().next[ch], prefix, pattern, results);
                prefix.pop();
            }
        } else {
            prefix.push(i as u8 as char);
            collect_match(x.as_ref().next[i], prefix, pattern, results);
            prefix.pop();
        }
    }
}

// returns the length of the longest string key in the subtrie
// rooted at x that is a prefix of the query string,
// assuming the first d character match and we have already
// found a prefix match of given length (-1 if no such match)
unsafe fn longest_prefix_of_dth<T>(
    x: Option<NonNull<Node<T>>>,
    query: &str,
    d: usize,
    mut length: i32,
) -> i32 {
    if let Some(x) = x {
        if x.as_ref().val.is_some() {
            length = d as i32;
        }
        if d == query.len() {
            return length;
        }
        let i = common::util::byte_at(query, d);
        let next = x.as_ref().next[i];
        longest_prefix_of_dth(next, query, d + 1, length)
    } else {
        length
    }
}

impl<T> Default for TrieST<T> {
    fn default() -> Self {
        Self { root: None, n: 0 }
    }
}

impl<T> Node<T> {
    fn new(val: Option<T>) -> NonNull<Self> {
        let v = Box::new(Self {
            val,
            next: vec![None; R],
        });
        Box::leak(v).into()
    }
}

impl<T> Drop for TrieST<T> {
    fn drop(&mut self) {
        fn visitor<T>(p: Option<NonNull<Node<T>>>) {
            if let Some(p) = p {
                let next = unsafe { Box::from_raw(p.as_ptr()).next };
                next.iter().for_each(|it| visitor(*it));
            }
        }

        let root = self.root.take();
        visitor(root);
        self.n = 0;
    }
}
