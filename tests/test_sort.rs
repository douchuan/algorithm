#![feature(is_sorted)]

use algo::sort;

macro_rules! case {
    // to test fn is "sort::$p::sort"
    // testcase name is "fn $p()"
    ($p: ident) => {
        #[test]
        fn $p() {
            let mut data = sort::util::vec_data();
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
            let mut data = sort::util::vec_data();
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
fn tournament_tree() {
    let mut data = sort::util::vec_data();
    for t in data.iter_mut() {
        let mut t = sort::tree_selection::sort_desc(t);
        t.reverse();
        assert!(t.is_sorted());
    }
}

#[test]
fn merge_v1() {
    let mut data = sort::util::vec_data();
    for t in data.iter_mut() {
        let t = sort::merge::v1::sort(t);
        assert!(t.is_sorted());
    }
}

#[test]
fn merge_v2() {
    let mut data = sort::util::vec_data();
    for t in data.iter_mut() {
        sort::merge::v2::sort(t);
        assert!(t.is_sorted());
    }
}

#[test]
fn merge_v3() {
    let mut data = sort::util::vec_data();
    data.iter_mut().for_each(|t| {
        sort::merge::v3::sort(t);
        assert!(t.is_sorted());
    });
}
