//! Three-way string quicksort
//!
//! Adapt quicksort to MSD string sorting by using 3-way
//! partitioning on the leading character of the keys,
//! moving to the next character on only the middle
//! subarray (keys with leading character equal to the
//! partitioning character).
//!
//! ### Compare to MSD
//!
//! Three-way string quicksort divides the array into
//! only three parts, so it involves more data movement
//! than MSD string sort when the number of nonempty
//! partitions is large because it has to
//! do a series of 3-way partitions to get the effect of
//! the multiway partition. On the other hand, MSD string
//! sort can create large numbers of (empty) subarrays,
//! whereas 3-way string quicksort always has just three.
//! Thus, 3-way string quicksort adapts well to handling
//! equal keys, keys with long common prefixes, keys that
//! fall into a small range, and small arrays— all situations
//! where MSD string sort runs slowly. Of particular importance
//! is that the partitioning adapts to different kinds of structure
//! in different parts of the key. Also, like quicksort, 3-way string
//! quicksort does not use extra space (other than the implicit stack
//! to support recursion), which is an important advantage over MSD
//! string sort, which requires space for both frequency counts and
//! an auxiliary array.

#![allow(clippy::many_single_char_names)]
use crate::{common, sort};
use std::cmp::Ordering;
use std::marker::PhantomData;

const CUTOFF: usize = 15; // cutoff to insertion sort

/// The Quick3String provides static methods for sorting an
/// array of strings using 3-way radix quicksort.
pub struct Quick3String<T> {
    _marker: PhantomData<T>,
}

/// The Quick3Way provides static methods for sorting an
/// array using quicksort with 3-way partitioning.
pub struct Quick3Way<T> {
    _marker: PhantomData<T>,
}

impl<T> Quick3String<T>
where
    T: AsRef<str>,
{
    /// Rearranges the array of strings in ascending order.
    pub fn sort(a: &mut [T]) {
        // Randomization.
        // As with any quicksort, it is generally worthwhile to
        // shuffle the array beforehand or to use a random paritioning
        // item by swapping the first item with a random one. The primary
        // reason to do so is to protect against worst-case performance
        // in the case that the array is already sorted or nearly sorted.
        common::util::shuffle(a);
        let n = a.len();
        Self::do_sort(a, 0, n.saturating_sub(1), 0);
    }

    /// 3-way string quicksort a[lo..hi] starting at d-th character
    fn do_sort(a: &mut [T], lo: usize, hi: usize, d: usize) {
        if hi <= lo + CUTOFF {
            sort::insert::sort_dth(a, lo, hi, d);
            return;
        }

        let (mut lt, mut gt, mut i) = (lo, hi, lo + 1);
        let v = common::util::char_at(a[lo].as_ref(), d);
        while i <= gt {
            let t = common::util::char_at(a[i].as_ref(), d);
            match t.cmp(&v) {
                Ordering::Less => {
                    a.swap(lt, i);
                    lt += 1;
                    i += 1;
                }
                Ordering::Greater => {
                    a.swap(i, gt);
                    gt -= 1;
                }
                Ordering::Equal => i += 1,
            }
        }

        // a[lo..lt-1] < v = a[lt..gt] < a[gt+1..hi]
        Self::do_sort(a, lo, lt.saturating_sub(1), d);
        if v >= 0 {
            // moving to the next character on only the middle 1ubarray
            // (keys with leading character equal to the partitioning character)
            Self::do_sort(a, lt, gt, d + 1);
        }
        Self::do_sort(a, gt + 1, hi, d);
    }
}

impl<T> Quick3Way<T>
where
    T: Ord + Clone,
{
    /// Rearranges the array in ascending order, using the natural order.
    pub fn sort(a: &mut [T]) {
        common::util::shuffle(a);
        let n = a.len();
        Self::do_sort(a, 0, n.saturating_sub(1));
    }

    /// quicksort the subarray a[lo .. hi] using 3-way partitioning
    fn do_sort(a: &mut [T], lo: usize, hi: usize) {
        if hi <= lo {
            return;
        }

        let (mut lt, mut gt, mut i) = (lo, hi, lo + 1);
        let v = a[lo].clone();
        while i <= gt {
            match a[i].cmp(&v) {
                Ordering::Less => {
                    a.swap(lt, i);
                    lt += 1;
                    i += 1;
                }
                Ordering::Greater => {
                    a.swap(i, gt);
                    gt -= 1;
                }
                Ordering::Equal => i += 1,
            }
        }

        Self::do_sort(a, lo, lt.saturating_sub(1));
        Self::do_sort(a, gt + 1, hi);
    }
}
