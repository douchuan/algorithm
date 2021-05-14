//! 希尔排序 (Shell's Sort)
//!
//! 又称“缩小增量排序”（Diminishing Increment Sort）,
//! 是直接插入排序算法的一种更高效的改进版本。希尔排序是
//! 非稳定排序算法。该方法因 D.L.Shell 于 1959 年提出而得名。

use crate::sort::util;
use std::fmt::Debug;

pub fn sort<T: Copy + Default + Debug, F>(a: &mut [T], test: F)
where
    F: Fn(T, T) -> bool + Copy,
{
    let len = a.len();
    if len > 1 {
        let mut gap = len;
        loop {
            gap = gap / 2;

            for i in 0..gap {
                let mut j = i + gap;
                while j < len {
                    let mut k = j as i32 - gap as i32;
                    while k >= 0 && !test(a[k as usize], a[k as usize + gap]) {
                        a.swap(k as usize, k as usize + gap);
                        k -= gap as i32;
                    }

                    j += gap;
                }
            }

            if gap == 1 {
                break;
            }
        }
    }
}

#[test]
fn t() {
    let test = |x: i32, y: i32| x < y;
    let data = util::plan_data();
    for (t, expect) in data {
        let mut tt = t.clone();
        sort(&mut tt, test);
        assert_eq!(tt, expect, "t = {:?}, expect = {:?}", t, expect);
    }
}
