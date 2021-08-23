// n, item length
// n = 1, vec!["a", "b", ... "z"]
// n = 2, vec!["aa", "bb", ... "zz"]
pub fn vec_alphabet(n: usize) -> Vec<String> {
    debug_assert!(n > 0);
    "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .map(|c| String::from(c).repeat(n))
        .collect()
}

#[test]
fn t_vec_alphabet() {
    let vec = vec_alphabet(1);
    assert_eq!("a", vec[0]);

    let vec = vec_alphabet(10);
    assert_eq!("aaaaaaaaaa", vec[0]);
}
