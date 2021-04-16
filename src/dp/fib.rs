#![allow(unused)]
//! fib

use std::cell::RefCell;

/// classic impl
#[allow(unused)]
fn fib_classic_recursive(n: usize) -> usize {
    match n {
        0 => 0,
        1 | 2 => 1,
        _ => fib_classic_recursive(n - 1) + fib_classic_recursive(n - 2),
    }
}

thread_local!(static MEMO: RefCell<Vec<usize>> = RefCell::new(vec![0; 1000]));

/// 缓存中间结果
#[allow(unused)]
fn fib_cache_result(n: usize) -> usize {
    match n {
        0 => 0,
        1 | 2 => 1,
        _ => match MEMO.with(|memo| memo.borrow()[n]) {
            0 => {
                let v = fib_cache_result(n - 1) + fib_cache_result(n - 2);
                MEMO.with(|memo| memo.borrow_mut()[n] = v);
                v
            }
            memo => memo,
        },
    }
}

/// 只保存前两个值，最节省内存和最快的方式
#[allow(unused)]
fn fib_classic_iteration_loop(n: usize) -> usize {
    match n {
        0 => 0,
        1 | 2 => 1,
        _ => {
            let mut prev = 1;
            let mut cur = 1;
            let mut i = 3;
            loop {
                let sum = prev + cur;
                prev = cur;
                cur = sum;

                i += 1;
                if i > n {
                    return cur;
                }
            }
        }
    }
}

/// 只保存前两个值，最节省内存和最快的方式
/// 但实际基准测试结果并没有预期的那么快, 有可能是for的原因
#[allow(unused)]
fn fib_classic_iteration_for(n: usize) -> usize {
    match n {
        0 => 0,
        1 | 2 => 1,
        _ => {
            let mut prev = 1;
            let mut cur = 1;
            for _ in 3..=n {
                let sum = prev + cur;
                prev = cur;
                cur = sum;
            }

            cur
        }
    }
}

extern "C" {
    fn fib_cache_result_c(v: usize) -> usize;
    fn fib_classic_iteration_for_c(v: usize) -> usize;
}

//////////testcase & benchmarks
use test::Bencher;

#[test]
fn t_fib_classic_recursive() {
    let values = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144];
    for (i, expect) in values.iter().enumerate() {
        assert_eq!(*expect, fib_classic_recursive(i));
    }
}

#[test]
fn t_fib_cache_result() {
    let values = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144];
    for (i, expect) in values.iter().enumerate() {
        assert_eq!(*expect, fib_cache_result(i));
    }
}

#[test]
fn t_fib_classic_iteration_loop() {
    let values = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144];
    for (i, expect) in values.iter().enumerate() {
        assert_eq!(*expect, fib_classic_iteration_loop(i));
    }
}

#[test]
fn t_fib_classic_iteration_for() {
    let values = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144];
    for (i, expect) in values.iter().enumerate() {
        assert_eq!(*expect, fib_classic_iteration_for(i));
    }
}

#[test]
fn t_fib_classic_recursive_c() {
    let values = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144];
    for (i, expect) in values.iter().enumerate() {
        assert_eq!(*expect, unsafe { fib_cache_result_c(i) });
    }
}

#[test]
fn t_fib_classic_iteration_for_c() {
    let values = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144];
    for (i, expect) in values.iter().enumerate() {
        assert_eq!(*expect, unsafe { fib_classic_iteration_for_c(i) });
    }
}

static BENCH_N: usize = 40;

#[bench]
fn bench_fib_classic_recursive(b: &mut Bencher) {
    b.iter(|| fib_classic_recursive(BENCH_N));
}

#[bench]
fn bench_fib_cache_result(b: &mut Bencher) {
    b.iter(|| fib_cache_result(BENCH_N));
}

#[bench]
fn bench_fib_classic_iteration_loop(b: &mut Bencher) {
    b.iter(|| fib_classic_iteration_loop(BENCH_N));
}

#[bench]
fn bench_fib_classic_iteration_for(b: &mut Bencher) {
    b.iter(|| fib_classic_iteration_for(BENCH_N));
}

#[bench]
fn bench_fib_cache_result_c(b: &mut Bencher) {
    b.iter(|| unsafe { fib_cache_result_c(BENCH_N) });
}

#[bench]
fn bench_fib_classic_iteration_for_c(b: &mut Bencher) {
    b.iter(|| unsafe { fib_classic_iteration_for_c(BENCH_N) });
}
