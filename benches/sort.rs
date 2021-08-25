#![feature(test)]
extern crate test;

use algo::strings::Quick3Way;
use algo::{common, sort};
use test::Bencher;

static DATA_LEN: usize = 1000;

#[bench]
fn small_merge_v1(b: &mut Bencher) {
    b.iter(|| {
        let mut numbs = [1, 2, 4, 8, 9, 9, 13, 17, 22];
        sort::merge::v1::sort(&mut numbs);
    });
}

#[bench]
fn large_merge_v1(b: &mut Bencher) {
    let data = gen_random_data(DATA_LEN);
    b.iter(|| {
        let mut numbs = data.clone();
        sort::merge::v1::sort(&mut numbs);
    });
}

#[bench]
fn large_sorted_asc_merge_v1(b: &mut Bencher) {
    let data = gen_asc_data(DATA_LEN);
    b.iter(|| {
        let mut numbs = data.clone();
        sort::merge::v1::sort(&mut numbs);
    });
}

#[bench]
fn large_sorted_desc_merge_v1(b: &mut Bencher) {
    let data = gen_desc_data(DATA_LEN);
    b.iter(|| {
        let mut numbs = data.clone();
        sort::merge::v1::sort(&mut numbs);
    });
}

#[bench]
fn eq_data_merge_v1(b: &mut Bencher) {
    let data = gen_eq_data(DATA_LEN);
    b.iter(|| {
        let mut numbs = data.clone();
        sort::merge::v1::sort(&mut numbs);
    });
}

#[bench]
fn small_merge_v2(b: &mut Bencher) {
    b.iter(|| {
        let mut numbs = [1, 2, 4, 8, 9, 9, 13, 17, 22];
        sort::merge::v2::sort(&mut numbs);
    });
}

#[bench]
fn large_merge_v2(b: &mut Bencher) {
    let data = gen_random_data(DATA_LEN);
    b.iter(|| {
        let mut numbs = data.clone();
        sort::merge::v2::sort(&mut numbs);
    });
}

#[bench]
fn large_sorted_asc_merge_v2(b: &mut Bencher) {
    let data = gen_asc_data(DATA_LEN);
    b.iter(|| {
        let mut numbs = data.clone();
        sort::merge::v2::sort(&mut numbs);
    });
}

#[bench]
fn large_sorted_desc_merge_v2(b: &mut Bencher) {
    let data = gen_desc_data(DATA_LEN);
    b.iter(|| {
        let mut numbs = data.clone();
        sort::merge::v2::sort(&mut numbs);
    });
}

#[bench]
fn eq_data_merge_v2(b: &mut Bencher) {
    let data = gen_eq_data(DATA_LEN);
    b.iter(|| {
        let mut numbs = data.clone();
        sort::merge::v2::sort(&mut numbs);
    });
}

#[bench]
fn small_merge_v3(b: &mut Bencher) {
    b.iter(|| {
        let mut numbs = [1, 2, 4, 8, 9, 9, 13, 17, 22];
        sort::merge::v3::sort(&mut numbs);
    });
}

#[bench]
fn large_merge_v3(b: &mut Bencher) {
    let data = gen_random_data(DATA_LEN);
    b.iter(|| {
        let mut numbs = data.clone();
        sort::merge::v3::sort(&mut numbs);
    });
}

#[bench]
fn large_sorted_asc_merge_v3(b: &mut Bencher) {
    let data = gen_asc_data(DATA_LEN);
    b.iter(|| {
        let mut numbs = data.clone();
        sort::merge::v3::sort(&mut numbs);
    });
}

#[bench]
fn large_sorted_desc_merge_v3(b: &mut Bencher) {
    let data = gen_desc_data(DATA_LEN);
    b.iter(|| {
        let mut numbs = data.clone();
        sort::merge::v3::sort(&mut numbs);
    });
}

#[bench]
fn eq_data_merge_v3(b: &mut Bencher) {
    let data = gen_eq_data(DATA_LEN);
    b.iter(|| {
        let mut numbs = data.clone();
        sort::merge::v3::sort(&mut numbs);
    });
}

#[bench]
fn small_quick(b: &mut Bencher) {
    b.iter(|| {
        let mut numbs = [1, 2, 4, 8, 9, 9, 13, 17, 22];
        sort::quick::sort(&mut numbs);
    });
}

#[bench]
fn large_quick(b: &mut Bencher) {
    let data = gen_random_data(DATA_LEN);
    b.iter(|| {
        let mut numbs = data.clone();
        sort::quick::sort(&mut numbs);
    });
}

#[bench]
fn large_quick_3way(b: &mut Bencher) {
    let data = gen_random_data(DATA_LEN);
    b.iter(|| {
        let mut numbs = data.clone();
        Quick3Way::sort(&mut numbs);
    });
}

#[bench]
fn large_sorted_asc_quick(b: &mut Bencher) {
    let data = gen_asc_data(DATA_LEN);
    b.iter(|| {
        let mut numbs = data.clone();
        sort::quick::sort(&mut numbs);
    });
}

#[bench]
fn large_sorted_desc_quick(b: &mut Bencher) {
    let data = gen_desc_data(DATA_LEN);
    b.iter(|| {
        let mut numbs = data.clone();
        sort::quick::sort(&mut numbs);
    });
}

#[bench]
fn eq_data_quick(b: &mut Bencher) {
    let data = gen_eq_data(DATA_LEN);
    b.iter(|| {
        let mut numbs = data.clone();
        sort::quick::sort(&mut numbs);
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
    let data = gen_random_data(DATA_LEN);
    b.iter(|| {
        let mut numbs = data.clone();
        sort::selection::sort(&mut numbs);
    });
}

#[bench]
fn large_selection_tree_selection(b: &mut Bencher) {
    let data = gen_random_data(DATA_LEN);
    b.iter(|| {
        let numbs = data.clone();
        sort::tree_selection::sort_desc(&numbs);
    });
}

// utils

fn gen_random_data(len: usize) -> Vec<i32> {
    let mut data: Vec<i32> = (0..len as i32).collect();
    common::util::shuffle(&mut data);
    data
}

pub fn gen_asc_data(len: usize) -> Vec<i32> {
    (0..len as i32).collect()
}

pub fn gen_desc_data(len: usize) -> Vec<i32> {
    let mut data = gen_asc_data(len);
    data.reverse();
    data
}

pub fn gen_eq_data(len: usize) -> Vec<i32> {
    vec![100; len]
}
