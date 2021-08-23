//! Performance
//!
//! The running time of MSD string sort depends on the data.
//!
//! For random inputs, MSD string sort examines just enough
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

#![allow(clippy::many_single_char_names)]
use std::cmp::Ordering;
use std::marker::PhantomData;

const R: usize = 256; // extended ASCII alphabet size
const CUTOFF: usize = 15; // cutoff to insertion sort

pub struct MSD<T> {
    _marker: PhantomData<T>,
}

impl<T> MSD<T>
where
    T: AsRef<str> + Copy,
{
    pub fn sort(a: &mut [T]) {
        let n = a.len();
        if n > 0 {
            let mut aux = vec![a[0].clone(); n];
            Self::do_sort(a, 0, n - 1, 0, &mut aux);
        }
    }

    fn do_sort(a: &mut [T], lo: usize, hi: usize, d: usize, aux: &mut [T]) {
        // cutoff to insertion sort for small subarrays
        if hi <= lo + CUTOFF {
            Self::insertion(a, lo, hi, d);
            return;
        }

        // compute frequency counts
        let mut count = [0; R + 2];
        for it in a.iter().take(hi + 1).skip(lo) {
            let c = Self::char_at(*it, d);
            count[(c + 2) as usize] += 1;
        }

        // transform counts to indicies
        for r in 0..R + 1 {
            count[r + 1] += count[r];
        }

        // distribute
        for it in a.iter().take(hi + 1).skip(lo) {
            let c = Self::char_at(*it, d);
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

    fn char_at(s: T, d: usize) -> i32 {
        let s = s.as_ref();
        let len = s.as_bytes().len();
        if d >= len {
            -1
        } else {
            s.as_bytes()[d] as i32
        }
    }

    fn insertion(a: &mut [T], lo: usize, hi: usize, d: usize) {
        for i in lo..=hi {
            let mut j = i;
            while j > lo && Self::less(a[j], a[j - 1], d) {
                a.swap(j, j - 1);
                j -= 1;
            }
        }
    }

    fn less(v: T, w: T, d: usize) -> bool {
        let v = v.as_ref();
        let w = w.as_ref();
        for (a, b) in v.bytes().zip(w.bytes()).skip(d) {
            match a.cmp(&b) {
                Ordering::Less => return true,
                Ordering::Equal => (),
                Ordering::Greater => return false,
            }
        }
        v.as_bytes().len() < w.as_bytes().len()
    }
}

#[test]
fn less() {
    assert!(MSD::less("aaa", "aaaa", 0)); // len less
    assert!(MSD::less("aaa", "aaaa", 1)); // len less
    assert!(MSD::less("aaa", "abaa", 1)); // 'a' < 'b'
}

#[test]
fn char_at() {
    assert_eq!(b'a' as i32, MSD::char_at("abc", 0));
    assert_eq!(b'b' as i32, MSD::char_at("abc", 1));
    assert_eq!(b'c' as i32, MSD::char_at("abc", 2));
    assert_eq!(-1, MSD::char_at("abc", 3));
}
