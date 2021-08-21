#![feature(is_sorted)]

use algo::sort;

#[test]
fn bubble() {
    let mut data = sort::util::vec_data();
    data.iter_mut().for_each(|t| {
        sort::bubble::sort(t);
        assert!(t.is_sorted());
    });
}

#[test]
fn insert() {
    let mut data = sort::util::vec_data();
    data.iter_mut().for_each(|t| {
        sort::insert::sort(t);
        assert!(t.is_sorted());
    });
}

#[test]
fn selection() {
    let mut data = sort::util::vec_data();
    data.iter_mut().for_each(|t| {
        sort::selection::sort(t);
        assert!(t.is_sorted());
    });
}

#[test]
fn selection_cocktail() {
    let mut data = sort::util::vec_data();
    data.iter_mut().for_each(|t| {
        sort::selection::sort_cocktail(t);
        assert!(t.is_sorted());
    });
}

#[test]
fn tournament_tree() {
    let mut data = sort::util::vec_data();
    data.iter_mut().for_each(|t| {
        let mut t = sort::tree_selection::sort_desc(t);
        t.reverse();
        assert!(t.is_sorted());
    });
}

#[test]
fn merge_v1() {
    let mut data = sort::util::vec_data();
    data.iter_mut().for_each(|t| {
        let t = sort::merge::v1::sort(t);
        assert!(t.is_sorted());
    });
}

#[test]
fn merge_v2() {
    let mut data = sort::util::vec_data();
    data.iter_mut().for_each(|t| {
        sort::merge::v2::sort(t);
        assert!(t.is_sorted());
    });
}

#[test]
fn merge_v3() {
    let mut data = sort::util::vec_data();
    data.iter_mut().for_each(|t| {
        sort::merge::v3::sort(t);
        assert!(t.is_sorted());
    });
}

#[test]
fn quick() {
    let mut data = sort::util::vec_data();
    data.iter_mut().for_each(|t| {
        sort::quick::sort(t);
        assert!(t.is_sorted());
    });
}

#[test]
fn shell() {
    let mut data = sort::util::vec_data();
    data.iter_mut().for_each(|t| {
        sort::shell::sort(t);
        assert!(t.is_sorted());
    });
}

#[test]
fn heap_sort_floyd() {
    let mut data = sort::util::vec_data();
    data.iter_mut().for_each(|t| {
        sort::floyd::sort(t);
        assert!(t.is_sorted());
    });
}
