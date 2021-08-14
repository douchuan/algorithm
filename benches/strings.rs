#![feature(test)]
extern crate test;

use algo::strings::LSD;
use test::Bencher;

const WORDS3: &'static str = include_str!("../res/strings/words3.txt");

#[allow(non_snake_case)]
#[bench]
fn std_Vec_sort(b: &mut Bencher) {
    let i = WORDS3;
    let mut a_words = Vec::new();
    for line in i.lines() {
        for s in line.split_whitespace() {
            a_words.push(s);
        }
    }

    b.iter(|| {
        let mut a = a_words.clone();
        a.sort();
    });
}

#[allow(non_snake_case)]
#[bench]
fn LSD_radix_sort(b: &mut Bencher) {
    let i = WORDS3;
    let mut a_words = Vec::new();
    for line in i.lines() {
        for s in line.split_whitespace() {
            a_words.push(s);
        }
    }

    b.iter(|| {
        let mut a = a_words.clone();
        let w = a[0].len();
        LSD::sort(&mut a, w);
    });
}
