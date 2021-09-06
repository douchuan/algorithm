use rand;
use rand::prelude::SliceRandom;

pub fn shuffle<T>(data: &mut [T]) {
    let mut rng = rand::thread_rng();
    data.shuffle(&mut rng);
}

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
#[inline(always)]
pub fn byte_at_checked(s: &str, d: usize) -> i32 {
    if let Some(v) = s.as_bytes().get(d) {
        *v as i32
    } else {
        -1
    }
}

/// uncheck version of byte_at
#[inline(always)]
pub fn byte_at(s: &str, d: usize) -> usize {
    s.as_bytes()[d] as usize
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
    assert_eq!(b'a' as i32, byte_at_checked("abc", 0));
    assert_eq!(b'b' as i32, byte_at_checked("abc", 1));
    assert_eq!(b'c' as i32, byte_at_checked("abc", 2));
    assert_eq!(-1, byte_at_checked("abc", 3));
}
