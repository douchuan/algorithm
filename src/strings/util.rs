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

/// return d-th character of s, -1 if d = length of string
pub fn char_at(s: &str, d: usize) -> i32 {
    let len = s.as_bytes().len();
    if d >= len {
        -1
    } else {
        s.as_bytes()[d] as i32
    }
}

#[test]
fn t_vec_alphabet() {
    let vec = vec_alphabet(1);
    assert_eq!("a", vec[0]);

    let vec = vec_alphabet(10);
    assert_eq!("aaaaaaaaaa", vec[0]);
}

#[test]
fn t_char_at() {
    assert_eq!(b'a' as i32, char_at("abc", 0));
    assert_eq!(b'b' as i32, char_at("abc", 1));
    assert_eq!(b'c' as i32, char_at("abc", 2));
    assert_eq!(-1, char_at("abc", 3));
}
