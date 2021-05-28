//! 算法新解, 刘新宇
//! Version: 0.6180339887498949
//! 8.4 本质改进
//!
//! 构建锦标赛树，复用比较结果，对Selection sort的性能做改进

use crate::tree::binary::builder::tournament::{BuildTournamentTree, Minimal};
use crate::tree::binary::builder::TreeBuilder;
use crate::tree::binary::{Tree, TreeNode};
use std::cmp::max;

/// 排序结果：大 -> 小
///
/// 构建tree的时间复杂度 O(n)
/// 每次pop的时间复杂度 O(log2(n))，所以弹出n个元素的的时间复杂度为 O(n * log2(n))
pub fn sort_desc<T>(data: &[T]) -> Vec<T>
where
    T: Copy + std::cmp::Ord + Minimal,
{
    let mut tree = TreeBuilder::build_tournament_tree(data);
    let mut r = Vec::with_capacity(data.len());
    while let Some(v) = TreeBuilder::tournament_tree_pop(&mut tree) {
        r.push(v);
    }
    r
}
