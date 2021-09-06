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
    M: usize,             // length of pattern
    dfa: Vec<Vec<usize>>, // the KMP automaton
}

impl KMP {
    /// Returns the index of the first occurrence of the pattern string
    /// in the text string.
    pub fn search(&self, txt: &str) -> Option<usize> {
        let dfa = self.dfa.as_slice();
        let M = self.M;
        let N = txt.len();
        let mut i = 0;
        let mut j = 0;
        while i < N && j < M {
            j = dfa[byte_at(txt, i)][j];
            i += 1;
        }
        if j == M {
            Some(i - M)
        } else {
            None
        }
    }
}

impl From<&str> for KMP {
    fn from(pat: &str) -> Self {
        let R = 256;
        let M = pat.len();

        // build DFA from pattern
        let mut dfa = vec![vec![0; M]; R];
        dfa[byte_at(pat, 0)][0] = 1;
        let mut x = 0;
        for j in 1..M {
            // allow 'clippy::needless_range_loop' here,
            // 'for' style makes sense
            #[allow(clippy::needless_range_loop)]
            for c in 0..R {
                dfa[c][j] = dfa[c][x]; // Copy mismatch cases.
            }
            dfa[byte_at(pat, j)][j] = j + 1; // Set match case.
            x = dfa[byte_at(pat, j)][x]; // Update restart state.
        }

        Self { M, dfa }
    }
}
