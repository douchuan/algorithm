#![feature(test)]
extern crate test;

use algo::common::{QuickUnionUF, WeightedQuickUnionUF, UF};
use std::str::FromStr;
use test::Bencher;

// const TINY_UF: &'static str = include_str!("../res/common/tinyUF.txt");
const MEDIUM_UF: &'static str = include_str!("../res/common/mediumUF.txt");

#[bench]
fn weighted_quf(b: &mut Bencher) {
    b.iter(|| {
        let i = MEDIUM_UF;
        let _uf = WeightedQuickUnionUF::from_str(i).unwrap();
    });
}

#[bench]
fn quf(b: &mut Bencher) {
    b.iter(|| {
        let i = MEDIUM_UF;
        let _uf = QuickUnionUF::from_str(i).unwrap();
    });
}

#[bench]
fn uf(b: &mut Bencher) {
    b.iter(|| {
        let i = MEDIUM_UF;
        let _uf = UF::from_str(i).unwrap();
    });
}
