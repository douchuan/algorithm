#![allow(non_snake_case)]
use crate::common;

pub fn search(pat: &str, txt: &str) -> Option<usize> {
    let M = pat.len();
    let N = txt.len();
    let mut i = 0;
    let mut j = 0;
    while i < N && j < M {
        let ic = common::util::byte_at(txt, i) as usize;
        let jc = common::util::byte_at(pat, j) as usize;
        if ic == jc {
            j += 1;
        } else {
            i -= j;
            j = 0;
        }
        i += 1;
    }
    if j == M {
        Some(i - M)
    } else {
        None
    }
}
