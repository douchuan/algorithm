#![feature(test)]
extern crate test;

use algo::math;
use test::Bencher;

static X: f32 = 999999999.0f32;

#[bench]
fn sqrt_std(b: &mut Bencher) {
    b.iter(|| X.sqrt());
}

#[bench]
fn sqrt_binary_search(b: &mut Bencher) {
    b.iter(|| math::mysqrt::sqrt_binary_search(X));
}

#[bench]
fn sqrt_newton(b: &mut Bencher) {
    b.iter(|| math::mysqrt::sqrt_newton(X));
}
