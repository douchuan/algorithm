use algo::strings::{alphabet, Alphabet, Count};

const ABRA: &'static str = include_str!("../res/strings/abra.txt");
const PI: &'static str = include_str!("../res/strings/pi.txt");

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
