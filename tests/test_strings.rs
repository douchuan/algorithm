use algo::strings::{alphabet, Alphabet, Count, LSD};

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
fn lsd_radix_sort() {
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
}

#[test]
fn lsd_radix_sort_i32() {
    let mut a: Vec<i32> = (0..10).rev().collect();
    LSD::sort_i32(&mut a);
    assert_eq!((0..10).collect::<Vec<i32>>(), a);
}

fn extract_words(i: &str) -> Vec<&str> {
    i.split_whitespace().collect()
}
