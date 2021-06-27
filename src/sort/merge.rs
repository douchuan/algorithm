#![allow(clippy::many_single_char_names)]
//! 归并排序（Merge Sort）
//!
//! 建立在归并操作上的一种有效，稳定的排序算法，该算法是采用分治法（Divide and Conquer）
//! 的一个非常典型的应用。将已有序的子序列合并，得到完全有序的序列；
//! 即先使每个子序列有序，再使子序列段间有序。
//!
//! 虽然快速排序在大多数情况下表现出众，但是在最坏情况下性能无法得到保证。
//! 即使各种工程上实践上的改进，也无法完全避免最坏情况。
//! 归并排序，能够在所有情况下都保证 O(n * log2(n)) 的性能。在算法的
//! 理论设计和分析上特别重要。此外，归并排序特别适于空间上链接的场景，可
//! 以对非连续存储的序列进行的排序。
//! 某些函数式编程环境和动态编程环境，往往使用归并排序作为标准库中的排序
//! 方案，包括 Haskell、 Python 和 Java(Java 7 之后)。
//!
//! v1：
//!   基本实现方案
//!
//! v2：
//!   对v1的改进，v1的问题是每次merge时需要分配空间存储归并结果
//!   v2在sort开始处创建一个与待排序数组同样大小的work space
//!
//! v3:
//!   对v2改进，避免分配与待排序数组同样大小的work space
//!   merge in place
//!   https://stackoverflow.com/questions/2571049/how-to-sort-in-place-using-the-merge-sort-algorithm
//!   https://github.com/liuxinyu95/AlgoXY/blob/algoxy/sorting/merge-sort/src/mergesort.c

pub mod v1 {
    fn merge<T, F>(mut l: &[T], mut r: &[T], compare: &F) -> Vec<T>
    where
        T: Copy,
        F: Fn(T, T) -> bool,
    {
        //存放归并结果
        //每次merge时分配work space，这是一个问题！v2对这个问题做了改进
        let mut ws = Vec::with_capacity(l.len() + r.len());

        while !l.is_empty() && !r.is_empty() {
            if compare(l[0], r[0]) {
                ws.push(l[0]);
                l = &l[1..];
            } else {
                ws.push(r[0]);
                r = &r[1..];
            }
        }

        //copy剩余的有序数组
        ws.extend_from_slice(l);
        ws.extend_from_slice(r);

        ws
    }

    pub fn sort<T, F>(a: &[T], compare: &F) -> Vec<T>
    where
        T: Copy,
        F: Fn(T, T) -> bool,
    {
        let len = a.len();
        match len {
            0 => vec![],
            1 => vec![a[0]],
            _ => {
                let (l, r) = a.split_at(len / 2);
                let l = sort(l, compare);
                let r = sort(r, compare);
                merge(&l, &r, compare)
            }
        }
    }
}

pub mod v2 {
    use std::ptr;

    //ws为辅助space
    fn merge<T, F>(a: &mut [T], l: usize, mid: usize, u: usize, compare: &F, ws: &mut [T])
    where
        T: Copy,
        F: Fn(T, T) -> bool,
    {
        //left部分索引
        let mut il = l;
        //right部分索引
        let mut iu = mid;
        //辅助work space索引
        let mut i = l;

        while il < mid && iu < u {
            if compare(a[il], a[iu]) {
                ws[i] = a[il];
                il += 1;
            } else {
                ws[i] = a[iu];
                iu += 1;
            }
            i += 1;
        }

        unsafe {
            //copy剩余的有序数组
            ptr::copy((&a[il..]).as_ptr(), (&mut ws[i..]).as_mut_ptr(), mid - il);
            ptr::copy((&a[iu..]).as_ptr(), (&mut ws[i..]).as_mut_ptr(), u - iu);

            //把归并结果复制回原数组
            ptr::copy((&ws[l..]).as_ptr(), (&mut a[l..]).as_mut_ptr(), u - l);
        }
    }

    fn do_sort<T, F>(a: &mut [T], l: usize, u: usize, compare: &F, ws: &mut [T])
    where
        T: Copy,
        F: Fn(T, T) -> bool,
    {
        if u - l > 1 {
            let mid = (u + l) / 2;
            do_sort(a, l, mid, compare, ws);
            do_sort(a, mid, u, compare, ws);
            merge(a, l, mid, u, compare, ws);
        }
    }

    pub fn sort<T, F>(a: &mut [T], compare: &F)
    where
        T: Copy + Default,
        F: Fn(T, T) -> bool,
    {
        let len = a.len();
        if len > 1 {
            // 分配一个与a同样大小的Vec作为辅助work space
            let mut ws = vec![T::default(); len];
            do_sort(a, 0, len, compare, &mut ws);
        }
    }
}

// ref, https://github.com/liuxinyu95/AlgoXY/blob/algoxy/sorting/merge-sort/src/mergesort.c
pub mod v3 {
    // merge two sorted subs xs[i, m) and xs[j...n) to working area xs[w...]
    fn wmerge<T, F>(
        xs: &mut [T],
        mut i: usize,
        m: usize,
        mut j: usize,
        n: usize,
        compare: &F,
        mut w: usize,
    ) where
        F: Fn(&T, &T) -> bool,
    {
        while i < m && j < n {
            if compare(&xs[i], &xs[j]) {
                xs.swap(w, i);
                i += 1;
            } else {
                xs.swap(w, j);
                j += 1;
            }
            w += 1;
        }

        while i < m {
            xs.swap(w, i);
            i += 1;
            w += 1;
        }

        while j < n {
            xs.swap(w, j);
            j += 1;
            w += 1;
        }
    }

    /// sort xs[l, u), and put result to working area w.
    /// constraint, len(w) == u - l
    fn wsort<T, F>(xs: &mut [T], mut l: usize, u: usize, compare: &F, mut w: usize)
    where
        F: Fn(&T, &T) -> bool,
    {
        if u - l > 1 {
            let m = (u + l) / 2;
            do_sort(xs, l, m, compare);
            do_sort(xs, m, u, compare);
            wmerge(xs, l, m, m, u, compare, w);
        } else {
            while l < u {
                xs.swap(l, w);
                l += 1;
                w += 1;
            }
        }
    }

    fn do_sort<T, F>(a: &mut [T], l: usize, u: usize, compare: &F)
    where
        F: Fn(&T, &T) -> bool,
    {
        if u - l > 1 {
            let mut m = (u + l) / 2;
            let mut w = l + u - m;
            // the last half contains sorted elements
            wsort(a, l, m, compare, w);
            while w - l > 2 {
                let n = w;
                w = l + (n - l + 1) / 2;
                // the first half of the previous working area contains sorted elements
                wsort(a, w, n, compare, l);
                wmerge(a, l, l + n - w, n, u, compare, w);
            }

            // switch to insertion sort
            let mut n = w;
            while n > l {
                m = n;
                while m < u && compare(&a[m], &a[m - 1]) {
                    a.swap(m, m - 1);
                    m += 1;
                }
                n -= 1;
            }
        }
    }

    pub fn sort<T, F>(a: &mut [T], compare: &F)
    where
        F: Fn(&T, &T) -> bool,
    {
        let len = a.len();
        do_sort(a, 0, len, compare);
    }
}
