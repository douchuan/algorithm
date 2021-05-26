//! 希尔排序 (Shell's Sort)
//!
//! 又称“缩小增量排序”（Diminishing Increment Sort）,
//! 是直接插入排序算法的一种更高效的改进版本。希尔排序是
//! 非稳定排序算法。该方法因 D.L.Shell 于 1959 年提出而得名。

pub fn sort<T, F>(a: &mut [T], compare: F)
where
    T: Copy,
    F: Fn(T, T) -> bool,
{
    let len = a.len();
    let mut gap = len;
    while gap > 1 {
        gap = gap / 2;

        for i in gap..len {
            let insert_v = a[i];
            let mut j = i;
            while j >= gap && compare(insert_v, a[j - gap]) {
                a[j] = a[j - gap];
                j -= gap;
            }
            a[j] = insert_v;
        }
    }
}
