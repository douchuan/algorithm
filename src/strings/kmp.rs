#![allow(non_snake_case)]

//! worst case for brute force search
//! compare with KMP:
//!
//! let pat = "AAAAAAAAAB";
//! let txt = "A".repeat(10000);
//!
//! test sub_search_brute_force ... bench:      68,616 ns/iter (+/- 9,302)
//! test sub_search_kmp         ... bench:      17,805 ns/iter (+/- 2,240)

use crate::common::util::byte_at;

pub struct KMP {
    m: usize,             // length of pattern
    dfa: Vec<Vec<usize>>, // the KMP automoton
}

impl KMP {
    pub fn search(&self, txt: &str) -> Option<usize> {
        let n = txt.len();
        let m = self.m;
        let mut i = 0;
        let mut j = 0;
        while i < n && j < m {
            j = self.dfa[byte_at(txt, i) as usize][j];
            i += 1;
        }
        if j == m {
            Some(i - m)
        } else {
            None
        }
    }
}

impl From<&str> for KMP {
    fn from(pat: &str) -> Self {
        let R = 256;
        let m = pat.len();

        // build DFA from pattern
        let mut dfa = vec![vec![0; m]; R];
        dfa[byte_at(pat, 0) as usize][0] = 1;
        let mut x = 0;
        for j in 1..m {
            for c in 0..R {
                dfa[c][j] = dfa[c][x]; // Copy mismatch cases.
            }
            dfa[byte_at(pat, j) as usize][j] = j + 1; // Set match case.
            x = dfa[byte_at(pat, j) as usize][x]; // Update restart state.
        }

        Self { m, dfa }
    }
}
