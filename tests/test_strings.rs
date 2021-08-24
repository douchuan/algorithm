#![feature(is_sorted)]
#![allow(non_snake_case)]
use algo::strings::{Quick3String, Quick3Way, LSD, MSD};

const WORDS3: &'static str = include_str!("../res/strings/words3.txt");
const SHELLS: &'static str = include_str!("../res/strings/shells.txt");

#[test]
fn LSD_radix_sort() {
    let i = WORDS3;
    let mut a = extract_words(i);
    let w = a[0].len();
    LSD::sort(&mut a, w);
    assert!(a.is_sorted());

    // license plate data
    let mut a = vec![
        "4PGC938", "2IYE230", "3CIO720", "1ICK750", "1OHV845", "4JZY524", "1ICK750", "3CIO720",
        "1OHV845", "1OHV845", "2RLA629", "2RLA629", "3ATW723",
    ];
    let w = a[0].len();
    LSD::sort(&mut a, w);
    assert!(a.is_sorted());
}

#[test]
fn LSD_radix_sort_i32() {
    let mut a: Vec<i32> = (0..10).rev().collect();
    LSD::sort_i32(&mut a);
    assert!(a.is_sorted());

    let mut a = vec![1, 2, 3, -1, -2, -3];
    LSD::sort_i32(&mut a);
    assert!(a.is_sorted());
}

#[test]
fn MSD_radix_sort() {
    // empty
    let mut data: Vec<&str> = vec![];
    MSD::sort(&mut data);
    assert!(data.is_sorted());

    // normal
    let i = SHELLS;
    let mut data = extract_words(i);
    MSD::sort(&mut data);
    assert!(data.is_sorted());
}

#[test]
fn quick3str() {
    // empty
    let mut data: Vec<&str> = vec![];
    Quick3String::sort(&mut data);
    assert!(data.is_sorted());

    // normal
    let i = SHELLS;
    let mut data = extract_words(i);
    Quick3String::sort(&mut data);
    assert!(data.is_sorted());
}

#[test]
fn quick3way() {
    // empty
    let mut data: Vec<&str> = vec![];
    Quick3Way::sort(&mut data);
    assert!(data.is_sorted());

    // normal
    let i = SHELLS;
    let mut data = extract_words(i);
    Quick3Way::sort(&mut data);
    assert!(data.is_sorted());
}

// also fine for sorted data
#[test]
fn sorted_data() {
    let mut a = vec![
        "4PGC938", "2IYE230", "3CIO720", "1ICK750", "1OHV845", "4JZY524", "1ICK750", "3CIO720",
        "1OHV845", "1OHV845", "2RLA629", "2RLA629", "3ATW723",
    ];
    a.sort();

    // lsd
    let w = a[0].len();
    LSD::sort(&mut a, w);
    assert!(a.is_sorted());

    // msd
    MSD::sort(&mut a);
    assert!(a.is_sorted());

    // three-way quick
    Quick3String::sort(&mut a);
    assert!(a.is_sorted());

    // lsd32
    let mut a: Vec<i32> = (0..10).collect();
    LSD::sort_i32(&mut a);
    assert!(a.is_sorted());
}

fn extract_words(i: &str) -> Vec<&str> {
    i.split_whitespace().collect()
}
