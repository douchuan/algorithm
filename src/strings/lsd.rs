#![allow(non_snake_case)]
//! LSD string sort
//!
//! The first string-sorting method that we consider is known
//! as least-significant-digit first (LSD) string sort. Consider
//! the following motivating application:
//! Suppose that a highway engineer sets up a device that records
//! the license plate numbers of all vehicles using a busy highway
//! for a given period of time and wants to know the number of
//! different vehicles that used the highway. As you know from
//! Section 2.1, one easy way to solve this problem is to sort the
//! numbers, then make a pass through to count the different values,
//! as in Dedup (page 490). License plates are a mixture of numbers
//! and letters, so it is natural to represent them as strings.
//! In the simplest situation (such as the California license
//! plate examples at right) the strings all have the same number
//! of characters. This situation is often found in sort
//! applications—for example, telephone numbers, bank account numbers,
//! and IP addresses are typically fixed-length strings.
//!
//! If the strings are each of length W,
//! we sort the strings W times with key-indexed counting, using
//! each of the positions as the key, proceeding from right to left.
//! It is not easy, at first, to be convinced that the method produces
//! a sorted array—in fact, it does not work at all unless the key-indexed
//! count implementation is stable.
//!
//! Proposition B. LSD string sort stably sorts fixed-length strings.
//! Proof: This fact depends crucially on the key-indexed counting
//! implementation being stable, as indicated in Proposition A.
//! After sorting keys on their i trailing characters (in a stable manner),
//! we know that any two keys appear in proper order in the array (
//! considering just those characters) either because the first of their
//! i trailing characters is different, in which case the sort on that
//! character puts them in order, or because the first of their ith
//! trailing characters is the same, in which case they are in order
//! because of stability (and by induction, for i-1).

const R_ASCII: usize = 256; // extend ASCII alphabet size
const BITS_PER_BYTE: usize = 8;
const R_I32: usize = 1 << BITS_PER_BYTE;

/// The LSD provides static methods for sorting an
/// array of w-character strings or 32-bit integers
/// using LSD radix sort.
pub struct LSD;

impl LSD {
    /// Rearranges the array of w-character strings in ascending order.
    /// `a` the array to be sorted
    /// `w` the number of characters per string
    pub fn sort<T: AsRef<str> + Copy>(a: &mut [T], w: usize) {
        let n = a.len();

        // a[0] just for init helper, no practical significance
        let mut aux = vec![a[0]; n];

        for d in (0..w).rev() {
            // sort by key-indexed counting on d-th character

            // compute frequency counts
            let mut count = [0; R_ASCII + 1];
            for it in a.iter().take(n) {
                let c = it.as_ref().as_bytes()[d];
                count[c as usize + 1] += 1;
            }

            // compute cumulates
            for r in 0..R_ASCII {
                count[r + 1] += count[r];
            }

            // move data
            for it in a.iter().take(n) {
                let c = it.as_ref().as_bytes()[d];
                aux[count[c as usize]] = *it;
                count[c as usize] += 1;
            }

            // copy back
            a[..n].clone_from_slice(&aux[..n]);
        }
    }

    pub fn sort_i32(a: &mut [i32]) {
        let BITS = 32;
        let MASK = R_I32 - 1;
        let w = BITS / BITS_PER_BYTE;

        let n = a.len();
        let mut aux = vec![0; n];
        for d in 0..w {
            // compute frequency counts
            let mut count = [0; R_I32 + 1];
            for it in a.iter().take(n) {
                let c = *it >> (BITS_PER_BYTE * d) & MASK as i32;
                count[c as usize + 1] += 1;
            }

            // compute cumulates
            for r in 0..R_I32 {
                count[r + 1] += count[r];
            }

            // for most significant byte, 0x80-0xFF comes before 0x00-0x7F
            if d == w - 1 {
                let shift1 = count[R_I32] - count[R_I32 / 2];
                let shift2 = count[R_I32 / 2];
                for it in count.iter_mut().take(R_I32 / 2) {
                    *it += shift1;
                }
                for it in count.iter_mut().take(R_I32).skip(R_I32 / 2) {
                    *it -= shift2;
                }
            }

            // move data
            for it in a.iter().take(n) {
                let c = *it >> (BITS_PER_BYTE * d) & MASK as i32;
                aux[count[c as usize]] = *it;
                count[c as usize] += 1;
            }

            // copy back
            a[..n].clone_from_slice(&aux[..n]);
        }
    }
}
