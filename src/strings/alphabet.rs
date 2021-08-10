//! A data type for alphabets, for use with string-processing code
//! that must convert between an alphabet of size R and the integers
//! 0 through R-1.
//!
//! Warning: supports only the basic multilingual plane (BMP), i.e,
//!          Unicode characters between U+0000 and U+FFFF.

// const MAX_VALUE: usize = char::MAX as usize;
const MAX_VALUE: usize = 65535;

macro_rules! declare {
    ($name: ident, $s: expr) => {
        lazy_static! {
            pub static ref $name: Alphabet<'static> = Alphabet::new($s).unwrap();
        }
    };
}

// The decimal alphabet { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9 }
declare!(DECIMAL, "0123456789");
// The DNA alphabet { A, C, T, G }
declare!(DNA, "ACGT");
// The base-64 alphabet (64 characters)
declare!(
    BASE64,
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
);

pub struct Alphabet<'a> {
    alphabet: &'a str,
    inverse: Vec<i32>,
    radix: usize,
}

impl<'a> Alphabet<'a> {
    pub fn new(alpha: &'a str) -> Result<Self, String> {
        let mut unicode = vec![false; MAX_VALUE];
        for c in alpha.chars() {
            if unicode[c as usize] {
                return Err(format!("Illegal alphabet: repeated character = {:?}", c));
            }
            unicode[c as usize] = true; //set code point
        }

        let radix = alpha.len();
        let mut inverse = vec![-1; MAX_VALUE];
        for i in 0..radix {
            inverse[alpha.chars().nth(i).unwrap() as usize] = i as i32;
        }

        Ok(Self {
            alphabet: alpha,
            inverse,
            radix,
        })
    }

    pub fn to_index(&self, c: char) -> Option<&i32> {
        self.inverse.get(c as usize)
    }

    pub fn to_indices(&self, s: &str) -> Vec<i32> {
        s.chars()
            .map(|c| self.to_index(c).unwrap())
            .cloned()
            .collect()
    }

    pub fn to_char(&self, index: i32) -> Option<char> {
        self.alphabet.chars().nth(index as usize)
    }

    pub fn to_chars(&self, indices: &[i32]) -> String {
        let mut buf = String::new();
        for &i in indices {
            if let Some(c) = self.to_char(i) {
                buf.push(c);
            }
        }
        buf
    }
}
