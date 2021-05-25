//! fib

use std::cell::RefCell;

/// classic impl
#[allow(unused)]
pub fn fib_classic_recursive(n: usize) -> usize {
    match n {
        0 => 0,
        1 | 2 => 1,
        _ => fib_classic_recursive(n - 1) + fib_classic_recursive(n - 2),
    }
}

thread_local!(static MEMO: RefCell<Vec<usize>> = RefCell::new(vec![0; 1000]));

/// 缓存中间结果
#[allow(unused)]
pub fn fib_cache_result(n: usize) -> usize {
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
pub fn fib_classic_iteration_loop(n: usize) -> usize {
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
pub fn fib_classic_iteration_for(n: usize) -> usize {
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
    pub fn fib_cache_result_c(v: usize) -> usize;
    pub fn fib_classic_iteration_for_c(v: usize) -> usize;
}
