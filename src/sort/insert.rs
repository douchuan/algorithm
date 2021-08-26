//! 插入排序
//!
//! 基本思想是将一个value插入到有序表中

use std::cmp::Ordering;

pub fn sort<T>(a: &mut [T])
where
    T: Ord,
{
    let len = a.len();
    // i begins with `1`
    for i in 1..len {
        // insert a[i] into a[0..i-1]
        let mut j = i;
        while j > 0 && a[j] < a[j - 1] {
            a.swap(j, j - 1);
            j -= 1;
        }
    }
}

/// insertion sort a[lo..=hi], starting at d-th character
/// lo & hi, is inclusive
pub fn sort_dth<T>(a: &mut [T], lo: usize, hi: usize, d: usize)
where
    T: AsRef<str>,
{
    // i begin with `lo + 1`
    for i in lo + 1..=hi {
        let mut j = i;
        while j > lo && is_less(a[j].as_ref(), a[j - 1].as_ref(), d) {
            a.swap(j, j - 1);
            j -= 1;
        }
    }
}

/// is v less than w, starting at d-th character
fn is_less(v: &str, w: &str, d: usize) -> bool {
    for (a, b) in v.bytes().zip(w.bytes()).skip(d) {
        match a.cmp(&b) {
            Ordering::Less => return true,
            Ordering::Equal => (),
            Ordering::Greater => return false,
        }
    }
    v.as_bytes().len() < w.as_bytes().len()
}

#[test]
fn t_less() {
    assert!(is_less("aaa", "aaaa", 0)); // len less
    assert!(is_less("aaa", "aaaa", 1)); // len less
    assert!(is_less("aaa", "abaa", 1)); // 'a' < 'b'
}
