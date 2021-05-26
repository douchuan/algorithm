//! 快速排序
//!
//! C. A. R. Hoare在1960年提出。
//! 它的基本思想是：通过一趟排序将要排序的数据分割成独立的两部分，
//! 其中一部分的所有数据都比另外一部分的所有数据都要小，然后再按
//! 此方法对这两部分数据分别进行快速排序，整个排序过程可以递归进行，
//! 以此达到整个数据变成有序序列
//!
//! 理想的情况是，每次划分所选择的中间数恰好将当前序列几乎等分，
//! 经过log2(n)趟划分，便可得到长度为1的子表。这样，整个算法
//! 的时间复杂度为O(n * log2(n))

pub fn sort<T, F>(a: &mut [T], compare: &F)
where
    F: Fn(&T, &T) -> bool,
{
    if let Some((pivot, elements)) = a.split_last_mut() {
        let mid = elements
            .iter_mut()
            .partition_in_place(|it| compare(it, pivot));
        //"pivot"就位
        a.swap(mid, a.len() - 1);

        sort(&mut a[0..mid], compare);
        sort(&mut a[mid + 1..], compare);
    }
}
