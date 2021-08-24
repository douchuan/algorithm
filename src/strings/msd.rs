//! Sort an array of strings using MSD radix sort.
//!
//! # Performance
//!
//! - The running time of MSD string sort depends on the data,
//! key length does not play a role. Indeed, the random string
//! model allows key length to approach infinity.
//! - For random inputs, MSD string sort examines just enough
//! characters to distinguish among the keys, and the running
//! time is sublinear in the number of characters in the data
//! (it examines a small fraction of the input characters).
//!
//! In the worst case, MSD string sort examines all the characters
//! in the keys, so the running time is linear in the number of
//! characters in the data (like LSD string sort). A worst-case
//! input is one with all strings equal.
//!
//! Example-1 (best case, examines just 1 char to distinguish among the keys)
//! ["aaaaa", ... "zzzzz"]
//!
//! d = 0,
//! count = [
//!  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//!  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//!  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//!  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//!  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11,
//!  12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 26,
//!  26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26,
//!  26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26,
//!  26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26,
//!  26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26,
//!  26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26,
//!  26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26,
//!  26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26,
//!  26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26,
//!  26, 26, 26, 26, 26]
//!
//! Example-2 (worst case, all strings equal, need check all chars):
//! ["aaaaa", ... "aaaaa"]
//!
//! The benchmark data of best and worst cases:
//!   test MSD_best_case      ... bench:         602 ns/iter (+/- 53)
//!   test MSD_worst_case     ... bench:       3,767 ns/iter (+/- 157)
//!
//! No absolute rules.
//! Some applications involve distinct keys that are well-modeled
//! by the random string model; others have significant numbers of
//! equal keys or long common prefixes, so the sort time is closer
//! to the worst case. Our license-plate-processing application, f
//! or example, can fall anywhere between these extremes: if our
//! engineer takes an hour of data from a busy interstate, there
//! will not be many duplicates and the random model will apply;
//! for a week’s worth of data on a local road, there will be numerous
//! duplicates and performance will be closer to the worst case.
//!
//! The main challenge in getting maximum efficiency from MSD string
//! sort on keys that are long strings is to deal with lack of randomness
//! in the data. Typically, keys may have long stretches of equal data,
//! or parts of them might fall in only a narrow range. For example,
//! an information-processing application for student data might have
//! keys that include graduation year (4 bytes, but one of four different
//! values), state names (perhaps 10 bytes, but one of 50 different values),
//! and gender (1 byte with one of two given values), as well as a person’s
//! name (more similar to random strings, but probably not short, with
//! nonuniform letter distributions, and with trailing blanks in a fixed-length
//! field). Restrictions like these lead to large numbers of empty subarrays
//! during the MSD string sort.

#![allow(clippy::many_single_char_names)]
use crate::sort;
use crate::strings::util;
use std::marker::PhantomData;

const R: usize = 256; // extended ASCII alphabet size
const CUTOFF: usize = 15; // cutoff to insertion sort

/// The MSD provides static methods for sorting an
/// array of extended ASCII strings using MSD radix
/// sort.
pub struct MSD<T> {
    _marker: PhantomData<T>,
}

impl<T> MSD<T>
where
    T: AsRef<str> + Copy,
{
    /// Rearranges the array of extended ASCII strings in ascending order.
    pub fn sort(a: &mut [T]) {
        let n = a.len();
        if n > 0 {
            let mut aux = vec![a[0]; n];
            Self::do_sort(a, 0, n - 1, 0, &mut aux);
        }
    }

    /// sort from a[lo] to a[hi], starting at the d-th character
    fn do_sort(a: &mut [T], lo: usize, hi: usize, d: usize, aux: &mut [T]) {
        // cutoff to insertion sort for small subarrays
        if hi <= lo + CUTOFF {
            sort::insert::sort_dth(a, lo, hi, d);
            return;
        }

        // compute frequency counts
        let mut count = [0; R + 2];
        for it in a.iter().take(hi + 1).skip(lo) {
            let c = util::char_at(it.as_ref(), d);
            count[(c + 2) as usize] += 1;
        }

        // transform counts to indicies
        for r in 0..R + 1 {
            count[r + 1] += count[r];
        }

        // distribute
        for it in a.iter().take(hi + 1).skip(lo) {
            let c = util::char_at(it.as_ref(), d);
            aux[count[(c + 1) as usize]] = *it;
            count[(c + 1) as usize] += 1;
        }

        // copy back
        a[lo..(hi + 1)].clone_from_slice(&aux[..((hi - lo) + 1)]);

        // sort substring
        // recursively sort for each character (excludes sentinel -1)
        for r in 0..R {
            let l = lo + count[r];
            let h = (lo + count[r + 1]).saturating_sub(1);
            // if [l..h] empty, avoid do_sort
            if h > l {
                Self::do_sort(a, l, h, d + 1, aux);
            }
        }
    }
}
