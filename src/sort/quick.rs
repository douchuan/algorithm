//! 快速排序
//!
//! C. A. R. Hoare在1960年提出。
//! 它的基本思想是：通过一趟排序将要排序的数据分割成独立的两部分，
//! 其中一部分的所有数据都比另外一部分的所有数据都要小，然后再按
//! 此方法对这两部分数据分别进行快速排序，整个排序过程可以递归进行，
//! 以此达到整个数据变成有序序列

pub fn sort<T>(a: &mut [T])
where
    T: Ord,
{
    let len = a.len();
    if len > 0 {
        let (l, _, r) = a.select_nth_unstable(len / 2);
        sort(l);
        sort(r)
    }
}
