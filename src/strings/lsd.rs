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

/// The LSD provides static methods for sorting an
/// array of w-character strings or 32-bit integers
/// using LSD radix sort.
pub struct LSD;

impl LSD {
    /// Rearranges the array of w-character strings in ascending order.
    /// `a` the array to be sorted
    /// `w` the number of characters per string
    pub fn sort<T: AsRef<str> + ?Sized>(a: &mut [&T], w: usize) {
        let n = a.len();
        let R = 256; // extend ASCII alphabet size
                     // a[0] just for init helper, no practical significance
        let mut aux = vec![a[0]; n];

        for d in (0..w).rev() {
            // sort by key-indexed counting on d-th character

            // compute frequency counts
            // Notes: count.len() is R + 1
            let mut count = vec![0; R + 1];
            for i in 0..n {
                let c = a[i].as_ref().chars().nth(d).unwrap();
                count[c as usize + 1] += 1;
            }

            // compute cumulates
            for r in 0..R {
                count[r + 1] += count[r];
            }

            // move data
            for i in 0..n {
                let c = a[i].as_ref().chars().nth(d).unwrap();
                let j = &mut count[c as usize];
                aux[*j] = a[i];
                *j += 1;
            }

            // copy back
            for i in 0..n {
                a[i] = aux[i];
            }
        }
    }

    pub fn sort_i32(a: &mut [i32]) {
        let BITS_PER_BYTE = 8;
        let BITS = 32;
        let R = 1 << BITS_PER_BYTE;
        let MASK = R - 1;
        let w = BITS / BITS_PER_BYTE;

        // assert_eq!(256, R);
        // assert_eq!(255, MASK);
        // assert_eq!(4, w);

        let n = a.len();
        let mut aux = vec![0; n];
        for d in 0..w {
            // compute frequency counts
            let mut count = vec![0; R + 1];
            for i in 0..n {
                let c = (a[i] >> BITS_PER_BYTE * d) & MASK as i32;
                count[c as usize + 1] += 1;
            }

            // compute cumulates
            for r in 0..R {
                count[r + 1] += count[r];
            }

            // for most significant byte, 0x80-0xFF comes before 0x00-0x7F
            #[cfg(target_endian = "big")]
            if d == w - 1 {
                let shift1 = count[R] - count[R / 2];
                let shift2 = count[R / 2];
                for r in 0..R / 2 {
                    count[r] += shift1;
                }
                for r in R / 2..R {
                    count[r] -= shift2;
                }
            }

            // move data
            for i in 0..n {
                let c = (a[i] >> BITS_PER_BYTE * d) & MASK as i32;
                aux[count[c as usize]] = a[i];
                count[c as usize] += 1;
            }

            // copy back
            for i in 0..n {
                a[i] = aux[i];
            }
        }
    }
}
