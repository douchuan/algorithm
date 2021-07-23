//! 堆排序
/// Robert. W. Floyd 给出了一个堆排序的高效实现。
/// 思路是构建一个最大堆，接下来，将最大的元素和数组末尾的元素交换，
/// 这样最大元素就存储到了排序后的正确位置。然后将堆的大小减一，执
/// 行 Heapify 恢复堆的性质。我们重复这一过程，直到堆中仅剩下一个元素。
/// 这一算法是原地排序的，无需使用额外的空间来存储结果
///
/// 就地排序，小 -> 大
use crate::common::max_heap;

pub fn floyd_sort<T>(a: &mut [T])
where
    T: Ord,
{
    // 构建最大堆
    max_heap::build_heap(a);

    let mut i = a.len();
    while i > 1 {
        i -= 1;
        a.swap(0, i);
        max_heap::heapify(&mut a[0..i], 0);
    }
}
