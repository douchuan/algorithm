#![feature(test)]
extern crate test;

use algo::strings::{util, LSD, MSD};
use test::Bencher;

const WORDS3: &'static str = include_str!("../res/strings/words3.txt");

#[allow(non_snake_case)]
#[bench]
fn sort_str_std_Vec(b: &mut Bencher) {
    let i = WORDS3;
    let mut words = extract_words(i);
    b.iter(|| {
        words.sort();
    });
}

#[allow(non_snake_case)]
#[bench]
fn sort_i32_std_Vec(b: &mut Bencher) {
    let mut nums: Vec<i32> = (0..1000).rev().collect();
    b.iter(|| {
        nums.sort();
    });
}

#[allow(non_snake_case)]
#[bench]
fn sort_str_LSD_radix(b: &mut Bencher) {
    let i = WORDS3;
    let mut words = extract_words(i);
    let w = words[0].len();
    b.iter(|| {
        LSD::sort(&mut words, w);
    });
}

#[allow(non_snake_case)]
#[bench]
fn sort_str_MSD_radix(b: &mut Bencher) {
    let i = WORDS3;
    let mut words = extract_words(i);
    b.iter(|| {
        MSD::sort(&mut words);
    });
}

#[allow(non_snake_case)]
#[bench]
fn sort_i32_LSD_radix(b: &mut Bencher) {
    let mut nums: Vec<i32> = (0..1000).rev().collect();
    b.iter(|| {
        LSD::sort_i32(&mut nums);
    });
}

#[allow(non_snake_case)]
#[bench]
fn MSD_worst_case(b: &mut Bencher) {
    // examines just 1 char to distinguish among the keys
    let mut words = vec!["1DNB377"; 26];
    b.iter(|| {
        MSD::sort(&mut words);
    });
}

#[allow(non_snake_case)]
#[bench]
fn MSD_best_case(b: &mut Bencher) {
    // all strings equal, need check all chars
    let words = util::vec_alphabet("1DNB377".len());
    let mut words: Vec<&str> = words.iter().map(|it| it.as_str()).collect();
    b.iter(|| {
        MSD::sort(&mut words);
    });
}

fn extract_words(i: &str) -> Vec<&str> {
    i.split_whitespace().collect()
}
