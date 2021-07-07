//! 选择排序
//!
//! 从待排序的数据元素中选出最小（或最大）的一个元素，
//! 然后放到已排序的序列的末尾，直到全部待排序的数据元素的个数为零。
//!
//! 选择排序是不稳定的排序方法

pub fn sort<T>(a: &mut [T])
where
    T: Ord,
{
    let len = a.len();
    for i in 0..len {
        let mut m = i;
        for j in (i + 1)..len {
            if a[m] > a[j] {
                m = j;
            }
        }

        a.swap(i, m);
    }
}

/// 鸡尾酒排序 (Cock-tail sort)
/// 每次扫描可以同时查找最小值和最大值，将最小值放到开头，
/// 最大值放到末尾
pub fn sort_cocktail<T>(a: &mut [T])
where
    T: Ord,
{
    let n = a.len();
    let semi_n = n / 2;
    // 注意右边界是semi_n
    // i在区间[0, semi_n)迭代的过程中，
    // max = n - 1 - i, 也逐渐向semi_n靠拢
    for i in 0..semi_n {
        let mut min = i;
        let mut max = n - 1 - i;
        if a[min] > a[max] {
            a.swap(min, max);
        }
        for j in (i + 1)..(n - 1 - i) {
            if a[min] > a[j] {
                min = j;
            }
            if a[max] < a[j] {
                max = j;
            }
        }

        a.swap(i, min);
        a.swap(n - 1 - i, max);
    }
}
