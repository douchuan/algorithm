//! 插入排序
//!
//! 基本思想是将一个value插入到有序表中

use crate::sort::util;

pub fn sort<T: Copy, F>(a: &mut [T], test: F)
where
    F: Fn(T, T) -> bool + Copy,
{
    let len = a.len();
    // 注意起始索引
    for i in 1..len {
        // 将a[i]插入到a[i-1]，a[i-2]，a[i-3]……之中
        let mut j = i;
        while j > 0 && test(a[j], a[j - 1]) {
            a.swap(j, j - 1);
            j -= 1;
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
