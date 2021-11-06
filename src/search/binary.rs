//! 二分查找

use std::cmp::Ordering;

pub fn search<K>(arr: &[K], k: K) -> Option<usize>
where
    K: Ord,
{
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = (left + right) >> 1;
        match arr[mid].cmp(&k) {
            Ordering::Less => left = mid + 1,
            Ordering::Equal => return Some(mid),
            Ordering::Greater => right = mid,
        }
    }

    None
}
