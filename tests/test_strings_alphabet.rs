use algo::strings::alphabet;

#[test]
fn t_alphabet() {
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
