//! 冒泡排序（Bubble Sort）
//!
//! 它重复地走访过要排序的元素列，依次比较两个相邻的元素，如果顺
//! 序（如从大到小、首字母从Z到A）错误就把他们交换过来。走访元素
//! 的工作是重复地进行直到没有相邻元素需要交换，也就是说该元素列
//! 已经排序完成。
//!
//! 这个算法的名字由来是因为越小的元素会经由交换慢慢“浮”到数列的
//! 顶端（升序或降序排列），就如同碳酸饮料中二氧化碳的气泡最终会
//! 上浮到顶端一样，故名“冒泡排序”。
//!

pub fn sort<T>(a: &mut [T])
where
    T: Ord,
{
    let len = a.len();
    for i in 0..len.saturating_sub(1) {
        let mut swapped = false;

        for j in 0..(len - 1 - i) {
            if a[j] > a[j + 1] {
                a.swap(j, j + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}
