//! Complete binary tree in array
//!
//! Initial sequence number begin with 0

/// left child of i
#[inline(always)]
pub fn left(i: usize) -> usize {
    (i << 1) + 1
}

/// right child of i
#[inline(always)]
pub fn right(i: usize) -> usize {
    (i + 1) << 1
}

/// parent of i
#[inline(always)]
pub fn parent(i: usize) -> usize {
    ((i + 1) >> 1) - 1
}
