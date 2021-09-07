#![allow(non_snake_case)]
#![allow(clippy::needless_range_loop)]

//!
//! worst case for brute force search
//! compare with KMP:
//!
//! let pat = "AAAAAAAAAB";
//! let txt = "A".repeat(10000);
//!
//! test sub_search_brute_force ... bench:      68,616 ns/iter (+/- 9,302)
//! test sub_search_kmp         ... bench:      17,805 ns/iter (+/- 2,240)
//!
//!
//! In practice, the speedup over the brute-force method is not
//! often important because few applications involve searching
//! for highly self-repetitive patterns in highly self-repetitive
//! text. Still, the method has the practical advantage that it
//! never backs up in the input. This property makes KMP substring
//! search more convenient for use on an input stream of undetermined
//! length (such as standard input) than algorithms requiring backup,
//! which need some complicated buffering in this situation.

use crate::common::util::byte_at;

pub struct KMP {
    M: usize,             // length of pattern
    // dfa is R rows, pat.len() columns
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
            // Compute dfa[][j].
            for c in 0..R {
                dfa[c][j] = dfa[c][x]; // Copy mismatch cases.
            }
            // for match case, DFA step to j + 1
            dfa[byte_at(pat, j)][j] = j + 1; // Set match case.
            x = dfa[byte_at(pat, j)][x]; // Update restart state.
        }

        Self { M, dfa }
    }
}
