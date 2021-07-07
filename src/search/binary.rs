//! 二分查找

use std::cmp::Ordering;

pub fn search<K>(xs: &[K], k: K) -> Option<usize>
where
    K: Ord,
{
    let mut l = 0;
    let mut u = xs.len();

    while l < u {
        let m = (l + u) / 2;
        match xs[m].cmp(&k) {
            Ordering::Equal => return Some(m),
            Ordering::Less => l = m + 1,
            _ => u = m,
        }
    }

    None
}
