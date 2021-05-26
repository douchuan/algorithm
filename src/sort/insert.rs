//! 插入排序
//!
//! 基本思想是将一个value插入到有序表中

pub fn sort<T, F>(a: &mut [T], compare: F)
where
    F: Fn(&T, &T) -> bool,
{
    let len = a.len();
    // 注意起始索引
    for i in 1..len {
        // 将a[i]插入到a[i-1]，a[i-2]，a[i-3]……之中
        let mut j = i;
        while j > 0 && compare(&a[j], &a[j - 1]) {
            a.swap(j, j - 1);
            j -= 1;
        }
    }
}
