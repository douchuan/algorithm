#![allow(non_snake_case)]
use algo::strings::{alphabet, Alphabet, Count, LSD, MSD};

const ABRA: &'static str = include_str!("../res/strings/abra.txt");
const PI: &'static str = include_str!("../res/strings/pi.txt");
const WORDS3: &'static str = include_str!("../res/strings/words3.txt");

#[test]
fn alphabet() {
    let s = "NowIsTheTimeForAllGoodMen";
    let encoded = alphabet::BASE64.to_indices(s);
    let decoded = alphabet::BASE64.to_chars(&encoded);
    assert_eq!(s, decoded);

    let s = "AACGAACGGTTTACCCCG";
    let encoded = alphabet::DNA.to_indices(s);
    let decoded = alphabet::DNA.to_chars(&encoded);
    assert_eq!(s, decoded);

    let s = "01234567890123456789";
    let encoded = alphabet::DECIMAL.to_indices(s);
    let decoded = alphabet::DECIMAL.to_chars(&encoded);
    assert_eq!(s, decoded);
}

#[test]
fn count() {
    use std::convert::TryFrom;

    let alphabet = Alphabet::try_from("ABCDR").unwrap();
    let r = Count::compute(&alphabet, ABRA);
    assert_eq!(vec![5, 2, 1, 1, 2], r);

    let r = Count::compute(&alphabet::DECIMAL, PI);
    assert_eq!(
        vec![9999, 10137, 9908, 10026, 9971, 10026, 10028, 10025, 9978, 9902],
        r
    );
}

#[test]
fn LSD_radix_sort() {
    let i = WORDS3;
    let mut a = extract_words(i);
    let w = a[0].len();
    LSD::sort(&mut a, w);
    assert_eq!(
        vec![
            "all", "bad", "bed", "bug", "dad", "dim", "dug", "egg", "fee", "few", "for", "gig",
            "hut", "ilk", "jam", "jay", "jot", "joy", "men", "nob", "now", "owl", "rap", "sky",
            "sob", "tag", "tap", "tar", "tip", "wad", "was", "wee", "yes", "yet", "zoo"
        ],
        a
    );

    // license plate data
    let mut a = vec![
        "4PGC938", "2IYE230", "3CIO720", "1ICK750", "1OHV845", "4JZY524", "1ICK750", "3CIO720",
        "1OHV845", "1OHV845", "2RLA629", "2RLA629", "3ATW723",
    ];
    let w = a[0].len();
    LSD::sort(&mut a, w);
    let expect = vec![
        "1ICK750", "1ICK750", "1OHV845", "1OHV845", "1OHV845", "2IYE230", "2RLA629", "2RLA629",
        "3ATW723", "3CIO720", "3CIO720", "4JZY524", "4PGC938",
    ];
    assert_eq!(expect, a);
}

#[test]
fn LSD_radix_sort_opt() {
    let i = WORDS3;
    let mut a = extract_words(i);
    let w = a[0].len();
    LSD::sort_opt(&mut a, w);
    assert_eq!(
        vec![
            "all", "bad", "bed", "bug", "dad", "dim", "dug", "egg", "fee", "few", "for", "gig",
            "hut", "ilk", "jam", "jay", "jot", "joy", "men", "nob", "now", "owl", "rap", "sky",
            "sob", "tag", "tap", "tar", "tip", "wad", "was", "wee", "yes", "yet", "zoo"
        ],
        a
    );

    // license plate data
    let mut a = vec![
        "4PGC938", "2IYE230", "3CIO720", "1ICK750", "1OHV845", "4JZY524", "1ICK750", "3CIO720",
        "1OHV845", "1OHV845", "2RLA629", "2RLA629", "3ATW723",
    ];
    let w = a[0].len();
    LSD::sort_opt(&mut a, w);
    let expect = vec![
        "1ICK750", "1ICK750", "1OHV845", "1OHV845", "1OHV845", "2IYE230", "2RLA629", "2RLA629",
        "3ATW723", "3CIO720", "3CIO720", "4JZY524", "4PGC938",
    ];
    assert_eq!(expect, a);
}

#[test]
fn LSD_radix_sort_i32() {
    let mut a: Vec<i32> = (0..10).rev().collect();
    LSD::sort_i32(&mut a);
    assert_eq!((0..10).collect::<Vec<i32>>(), a);

    let mut a = vec![1, 2, 3, -1, -2, -3];
    LSD::sort_i32(&mut a);
    assert_eq!(vec![-3, -2, -1, 1, 2, 3], a);
}

#[test]
fn LSD_radix_sort_i32_opt() {
    let mut a: Vec<i32> = (0..10).rev().collect();
    LSD::sort_i32_opt(&mut a);
    assert_eq!((0..10).collect::<Vec<i32>>(), a);

    let mut a = vec![1, 2, 3, -1, -2, -3];
    LSD::sort_i32_opt(&mut a);
    assert_eq!(vec![-3, -2, -1, 1, 2, 3], a);
}

#[test]
fn MSD_radix_sort() {
    let i = WORDS3;
    let mut a = extract_words(i);
    MSD::sort(&mut a);
    assert_eq!(
        vec![
            "all", "bad", "bed", "bug", "dad", "dim", "dug", "egg", "fee", "few", "for", "gig",
            "hut", "ilk", "jam", "jay", "jot", "joy", "men", "nob", "now", "owl", "rap", "sky",
            "sob", "tag", "tap", "tar", "tip", "wad", "was", "wee", "yes", "yet", "zoo"
        ],
        a
    );

    // license plate data
    let mut a = vec![
        "4PGC938", "2IYE230", "3CIO720", "1ICK750", "1OHV845", "4JZY524", "1ICK750", "3CIO720",
        "1OHV845", "1OHV845", "2RLA629", "2RLA629", "3ATW723",
    ];
    MSD::sort(&mut a);
    let expect = vec![
        "1ICK750", "1ICK750", "1OHV845", "1OHV845", "1OHV845", "2IYE230", "2RLA629", "2RLA629",
        "3ATW723", "3CIO720", "3CIO720", "4JZY524", "4PGC938",
    ];
    assert_eq!(expect, a);
}

fn extract_words(i: &str) -> Vec<&str> {
    i.split_whitespace().collect()
}
