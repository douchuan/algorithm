#![feature(test)]
#![allow(non_snake_case)]
extern crate test;

use algo::common;
use algo::strings::{brute_force, Quick3String, KMP, LSD, MSD};
use test::Bencher;

const WORDS3: &'static str = include_str!("../res/strings/words3.txt");
const SHELLS: &'static str = include_str!("../res/strings/shells.txt");

#[bench]
fn sort_str_std_Vec(b: &mut Bencher) {
    let i = WORDS3;
    let mut words = extract_words(i);
    b.iter(|| {
        words.sort();
    });
}

#[bench]
fn sort_i32_std_Vec(b: &mut Bencher) {
    let mut nums: Vec<i32> = (0..1000).rev().collect();
    b.iter(|| {
        nums.sort();
    });
}

#[bench]
fn sort_str_LSD_radix(b: &mut Bencher) {
    let i = WORDS3;
    let mut words = extract_words(i);
    let w = words[0].len();
    b.iter(|| {
        LSD::sort(&mut words, w);
    });
}

#[bench]
fn sort_i32_LSD_radix(b: &mut Bencher) {
    let mut nums: Vec<i32> = (0..1000).rev().collect();
    b.iter(|| {
        LSD::sort_i32(&mut nums);
    });
}

#[bench]
fn sort_str_MSD_radix(b: &mut Bencher) {
    let i = SHELLS;
    let mut words = extract_words(i);
    b.iter(|| {
        MSD::sort(&mut words);
    });
}

#[bench]
fn sort_str_quick3strings(b: &mut Bencher) {
    let i = SHELLS;
    let mut words = extract_words(i);
    b.iter(|| {
        Quick3String::sort(&mut words);
    });
}

#[bench]
fn MSD_worst_case(b: &mut Bencher) {
    // examines just 1 char to distinguish among the keys
    let mut words = vec!["1DNB377"; 26];
    b.iter(|| {
        MSD::sort(&mut words);
    });
}

#[bench]
fn MSD_best_case(b: &mut Bencher) {
    // all strings equal, need check all chars
    let words = common::util::vec_alphabet("1DNB377".len());
    let mut words: Vec<&str> = words.iter().map(|it| it.as_str()).collect();
    b.iter(|| {
        MSD::sort(&mut words);
    });
}

#[bench]
fn sub_search_kmp(b: &mut Bencher) {
    let mut pat = "A".repeat(10);
    pat.push('B');
    let txt = "A".repeat(10000);
    let kmp = KMP::from(pat.as_str());
    b.iter(|| kmp.search(txt.as_str()));
}

#[bench]
fn sub_search_brute_force(b: &mut Bencher) {
    // worst case for brute force search
    let mut pat = "A".repeat(10);
    pat.push('B');
    let txt = "A".repeat(10000);
    b.iter(|| brute_force::search1(pat.as_str(), txt.as_str()));
}

fn extract_words(i: &str) -> Vec<&str> {
    i.split_whitespace().collect()
}
