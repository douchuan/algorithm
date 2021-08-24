//! 插入排序
//!
//! 基本思想是将一个value插入到有序表中

use std::cmp::Ordering;

pub fn sort<T>(a: &mut [T])
where
    T: Ord,
{
    let len = a.len();
    // 注意起始索引
    for i in 1..len {
        // 将a[i]插入到a[i-1]，a[i-2]，a[i-3]……之中
        let mut j = i;
        while j > 0 && a[j] < a[j - 1] {
            a.swap(j, j - 1);
            j -= 1;
        }
    }
}

/// insertion sort a[lo..=hi], starting at d-th character
pub fn sort_dth<T>(a: &mut [T], lo: usize, hi: usize, d: usize)
where
    T: AsRef<str>,
{
    for i in lo..=hi {
        let mut j = i;
        while j > lo && is_less(a[j].as_ref(), a[j - 1].as_ref(), d) {
            a.swap(j, j - 1);
            j -= 1;
        }
    }
}

/// is v less than w, starting at character d
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
