#![feature(test)]
extern crate test;

use algo::dp;
use test::Bencher;

static MAKE_CHANGES_AMOUNT: i32 = 20;

#[bench]
fn make_changes_classic(b: &mut Bencher) {
    let coins = vec![1, 2, 5];
    b.iter(|| dp::coin::make_change_classic(&coins, MAKE_CHANGES_AMOUNT));
}

#[bench]
fn make_changes_iter(b: &mut Bencher) {
    let coins = vec![1, 2, 5];
    b.iter(|| dp::coin::make_change_iter(&coins, MAKE_CHANGES_AMOUNT));
}

static BENCH_N: usize = 20;

#[bench]
fn fib_classic_recursive(b: &mut Bencher) {
    b.iter(|| dp::fib::fib_classic_recursive(BENCH_N));
}

#[bench]
fn fib_cache_result(b: &mut Bencher) {
    b.iter(|| dp::fib::fib_cache_result(BENCH_N));
}

#[bench]
fn fib_classic_iteration_loop(b: &mut Bencher) {
    b.iter(|| dp::fib::fib_classic_iteration_loop(BENCH_N));
}

#[bench]
fn fib_classic_iteration_for(b: &mut Bencher) {
    b.iter(|| dp::fib::fib_classic_iteration_for(BENCH_N));
}

#[bench]
fn fib_cache_result_c(b: &mut Bencher) {
    b.iter(|| unsafe { dp::fib::fib_cache_result_c(BENCH_N) });
}

#[bench]
fn fib_classic_iteration_for_c(b: &mut Bencher) {
    b.iter(|| unsafe { dp::fib::fib_classic_iteration_for_c(BENCH_N) });
}
