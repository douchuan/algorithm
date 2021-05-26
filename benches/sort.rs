#![feature(test)]
extern crate test;

use algo::sort;
use algo::sort::util;
use test::Bencher;

#[bench]
fn small_merge_v1(b: &mut Bencher) {
    b.iter(|| {
        let mut numbs = [1, 2, 4, 8, 9, 9, 13, 17, 22];
        let f = |x: i32, y: i32| x > y;
        sort::merge::v1::sort(&mut numbs, &f);
    });
}

#[bench]
fn large_merge_v1(b: &mut Bencher) {
    let data = util::random_data(util::DATA_LEN);
    b.iter(|| {
        let mut numbs = data.clone();
        let f = |x: i32, y: i32| x > y;
        sort::merge::v1::sort(&mut numbs, &f);
    });
}

#[bench]
fn large_sorted_asc_merge_v1(b: &mut Bencher) {
    let data = util::sorted_data_asc(util::DATA_LEN);
    b.iter(|| {
        let mut numbs = data.clone();
        let f = |x: i32, y: i32| x > y;
        sort::merge::v1::sort(&mut numbs, &f);
    });
}

#[bench]
fn large_sorted_desc_merge_v1(b: &mut Bencher) {
    let data = util::sorted_data_desc(util::DATA_LEN);
    b.iter(|| {
        let mut numbs = data.clone();
        let f = |x: i32, y: i32| x > y;
        sort::merge::v1::sort(&mut numbs, &f);
    });
}

#[bench]
fn eq_data_merge_v1(b: &mut Bencher) {
    let data = util::eq_data(util::DATA_LEN);
    b.iter(|| {
        let mut numbs = data.clone();
        let f = |x: i32, y: i32| x > y;
        sort::merge::v1::sort(&mut numbs, &f);
    });
}

#[bench]
fn small_merge_v2(b: &mut Bencher) {
    b.iter(|| {
        let mut numbs = [1, 2, 4, 8, 9, 9, 13, 17, 22];
        let f = |x: i32, y: i32| x > y;
        sort::merge::v2::sort(&mut numbs, &f);
    });
}

#[bench]
fn large_merge_v2(b: &mut Bencher) {
    let data = util::random_data(util::DATA_LEN);
    b.iter(|| {
        let mut numbs = data.clone();
        let f = |x: i32, y: i32| x > y;
        sort::merge::v2::sort(&mut numbs, &f);
    });
}

#[bench]
fn large_sorted_asc_merge_v2(b: &mut Bencher) {
    let data = util::sorted_data_asc(util::DATA_LEN);
    b.iter(|| {
        let mut numbs = data.clone();
        let f = |x: i32, y: i32| x > y;
        sort::merge::v2::sort(&mut numbs, &f);
    });
}

#[bench]
fn large_sorted_desc_merge_v2(b: &mut Bencher) {
    let data = util::sorted_data_desc(util::DATA_LEN);
    b.iter(|| {
        let mut numbs = data.clone();
        let f = |x: i32, y: i32| x > y;
        sort::merge::v2::sort(&mut numbs, &f);
    });
}

#[bench]
fn eq_data_merge_v2(b: &mut Bencher) {
    let data = util::eq_data(util::DATA_LEN);
    b.iter(|| {
        let mut numbs = data.clone();
        let f = |x: i32, y: i32| x > y;
        sort::merge::v2::sort(&mut numbs, &f);
    });
}

#[bench]
fn small_merge_v3(b: &mut Bencher) {
    b.iter(|| {
        let mut numbs = [1, 2, 4, 8, 9, 9, 13, 17, 22];
        let f = |x: &i32, y: &i32| x > y;
        sort::merge::v3::sort(&mut numbs, &f);
    });
}

#[bench]
fn large_merge_v3(b: &mut Bencher) {
    let data = util::random_data(util::DATA_LEN);
    b.iter(|| {
        let mut numbs = data.clone();
        let f = |x: &i32, y: &i32| x > y;
        sort::merge::v3::sort(&mut numbs, &f);
    });
}

#[bench]
fn large_sorted_asc_merge_v3(b: &mut Bencher) {
    let data = util::sorted_data_asc(util::DATA_LEN);
    b.iter(|| {
        let mut numbs = data.clone();
        let f = |x: &i32, y: &i32| x > y;
        sort::merge::v3::sort(&mut numbs, &f);
    });
}

#[bench]
fn large_sorted_desc_merge_v3(b: &mut Bencher) {
    let data = util::sorted_data_desc(util::DATA_LEN);
    b.iter(|| {
        let mut numbs = data.clone();
        let f = |x: &i32, y: &i32| x > y;
        sort::merge::v3::sort(&mut numbs, &f);
    });
}

#[bench]
fn eq_data_merge_v3(b: &mut Bencher) {
    let data = util::eq_data(util::DATA_LEN);
    b.iter(|| {
        let mut numbs = data.clone();
        let f = |x: &i32, y: &i32| x > y;
        sort::merge::v3::sort(&mut numbs, &f);
    });
}

#[bench]
fn small_quick(b: &mut Bencher) {
    b.iter(|| {
        let mut numbs = [1, 2, 4, 8, 9, 9, 13, 17, 22];
        let f = |x: &i32, y: &i32| x > y;
        sort::quick::sort(&mut numbs, &f);
    });
}

#[bench]
fn large_quick(b: &mut Bencher) {
    let data = util::random_data(util::DATA_LEN);
    b.iter(|| {
        let mut numbs = data.clone();
        let f = |x: &i32, y: &i32| x > y;
        sort::quick::sort(&mut numbs, &f);
    });
}

#[bench]
fn large_sorted_asc_quick(b: &mut Bencher) {
    let data = util::sorted_data_asc(util::DATA_LEN);
    b.iter(|| {
        let mut numbs = data.clone();
        let f = |x: &i32, y: &i32| x > y;
        sort::quick::sort(&mut numbs, &f);
    });
}

#[bench]
fn large_sorted_desc_quick(b: &mut Bencher) {
    let data = util::sorted_data_desc(util::DATA_LEN);
    b.iter(|| {
        let mut numbs = data.clone();
        let f = |x: &i32, y: &i32| x > y;
        sort::quick::sort(&mut numbs, &f);
    });
}

#[bench]
fn eq_data_quick(b: &mut Bencher) {
    let data = util::eq_data(util::DATA_LEN);
    b.iter(|| {
        let mut numbs = data.clone();
        let f = |x: &i32, y: &i32| x > y;
        sort::quick::sort(&mut numbs, &f);
    });
}

#[bench]
fn small_selection_sort(b: &mut Bencher) {
    b.iter(|| {
        let mut numbs = [1, 2, 4, 8, 9, 9, 13, 17, 22];
        sort::selection::sort(&mut numbs);
    });
}

#[bench]
fn small_selection_tree_selection(b: &mut Bencher) {
    b.iter(|| {
        let numbs = [1, 2, 4, 8, 9, 9, 13, 17, 22];
        sort::tree_selection::sort_desc(&numbs);
    });
}

#[bench]
fn large_selection_sort(b: &mut Bencher) {
    let data = util::random_data(util::DATA_LEN);
    b.iter(|| {
        let mut numbs = data.clone();
        sort::selection::sort(&mut numbs);
    });
}

#[bench]
fn large_selection_tree_selection(b: &mut Bencher) {
    let data = util::random_data(util::DATA_LEN);
    b.iter(|| {
        let numbs = data.clone();
        sort::tree_selection::sort_desc(&numbs);
    });
}
