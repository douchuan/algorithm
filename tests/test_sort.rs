#![feature(is_sorted)]

use algo::common;
use algo::sort;

macro_rules! case {
    // to test fn is "sort::$p::sort"
    // testcase name is "fn $p()"
    ($p: ident) => {
        #[test]
        fn $p() {
            let mut data = gen_vec_data();
            for t in data.iter_mut() {
                sort::$p::sort(t);
                assert!(t.is_sorted());
            }
        }
    };
    // to test fn is "sort::$p::$f"
    // testcase name is "fn $f()"
    ($p: ident, $f: ident) => {
        #[test]
        fn $f() {
            let mut data = gen_vec_data();
            for t in data.iter_mut() {
                sort::$p::$f(t);
                assert!(t.is_sorted());
            }
        }
    };
}

case!(bubble);
case!(insert);
case!(selection);
case!(selection, sort_cocktail);
case!(quick);
case!(shell);
case!(floyd);

#[test]
fn insert_sort_dth() {
    let mut data = common::util::vec_alphabet(1);
    common::util::shuffle(&mut data);
    let len = data.len();
    sort::insert::sort_dth(&mut data, 0, len - 1, 0);
    assert!(data.is_sorted());
}

#[test]
fn tournament_tree() {
    let mut data = gen_vec_data();
    for t in data.iter_mut() {
        let t = sort::tree_selection::sort_desc(t);
        assert!(t.iter().rev().is_sorted());
    }
}

#[test]
fn merge() {
    let mut data = gen_vec_data();
    for t in data.iter_mut() {
        let t = sort::merge::v1::sort(t);
        assert!(t.is_sorted());
    }

    let mut data = gen_vec_data();
    for t in data.iter_mut() {
        sort::merge::v2::sort(t);
        assert!(t.is_sorted());
    }

    let mut data = gen_vec_data();
    for t in data.iter_mut() {
        sort::merge::v3::sort(t);
        assert!(t.is_sorted());
    }
}

// utils

fn gen_vec_data() -> Vec<Vec<i32>> {
    let sorted: Vec<i32> = (1..100).collect();
    let mut unsorted = sorted.clone();
    common::util::shuffle(&mut unsorted);

    vec![
        //empty
        vec![],
        //only 1
        vec![1],
        sorted,
        unsorted,
    ]
}
