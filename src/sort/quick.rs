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

use crate::sort::util;
use test::Bencher;

pub fn sort<T: Copy, F>(a: &mut [T], test: F)
where
    F: Fn(T, T) -> bool + Copy,
{
    if let Some((pivot, elements)) = a.split_last_mut() {
        let mid = elements
            .iter_mut()
            .partition_in_place(|it| test(*it, *pivot));
        //"pivot"就位
        a.swap(mid, a.len() - 1);

        sort(&mut a[0..mid], test);
        sort(&mut a[mid + 1..], test);
    }
}

#[bench]
fn bench_small(b: &mut Bencher) {
    b.iter(|| {
        let mut numbs = [1, 2, 4, 8, 9, 9, 13, 17, 22];
        let test = |x: i32, y: i32| x > y;
        sort(&mut numbs, test);
    });
}

#[bench]
fn bench_large(b: &mut Bencher) {
    let data = util::random_data(util::DATA_LEN);
    b.iter(|| {
        let mut numbs = data.clone();
        let test = |x: i32, y: i32| x > y;
        sort(&mut numbs, test);
    });
}

#[bench]
fn bench_large_sorted_asc(b: &mut Bencher) {
    let data = util::sorted_data_asc(util::DATA_LEN);
    b.iter(|| {
        let mut numbs = data.clone();
        let test = |x: i32, y: i32| x > y;
        sort(&mut numbs, test);
    });
}

#[bench]
fn bench_large_sorted_desc(b: &mut Bencher) {
    let data = util::sorted_data_desc(util::DATA_LEN);
    b.iter(|| {
        let mut numbs = data.clone();
        let test = |x: i32, y: i32| x > y;
        sort(&mut numbs, test);
    });
}

#[bench]
fn bench_eq_data(b: &mut Bencher) {
    let data = util::eq_data(util::DATA_LEN);
    b.iter(|| {
        let mut numbs = data.clone();
        let test = |x: i32, y: i32| x > y;
        sort(&mut numbs, test);
    });
}
