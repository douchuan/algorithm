//! A data type for alphabets, for use with string-processing code
//! that must convert between an alphabet of size R and the integers
//! 0 through R-1.
//!
//! Warning: supports only the basic multilingual plane (BMP), i.e,
//!          Unicode characters between U+0000 and U+FFFF.
use std::convert::TryFrom;

// const MAX_VALUE: usize = char::MAX as usize;
const MAX_VALUE: usize = 65535;

macro_rules! declare {
    ($name:ident, $v:expr) => {
        lazy_static! {
            pub static ref $name: Alphabet = Alphabet::try_from($v).unwrap();
        }
    };
}

// The binary alphabet { 0, 1 }
declare!(BINARY, "01");
// The octal alphabet { 0, 1, 2, 3, 4, 5, 6, 7 }
declare!(OCTAL, "01234567");
// The decimal alphabet { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9 }
declare!(DECIMAL, "0123456789");
//  The hexadecimal alphabet { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, A, B, C, D, E, F }
declare!(HEXADECIMAL, "0123456789ABCDEF");
// The DNA alphabet { A, C, T, G }
declare!(DNA, "ACGT");
// The lowercase alphabet { a, b, c, ..., z }
declare!(LOWERCASE, "abcdefghijklmnopqrstuvwxyz");
// The uppercase alphabet { A, B, C, ..., Z }
declare!(UPPERCASE, "ABCDEFGHIJKLMNOPQRSTUVWXYZ");
// The protein alphabet { A, C, D, E, F, G, H, I, K, L, M, N, P, Q, R, S, T, V, W, Y }
declare!(PROTEIN, "ACDEFGHIKLMNPQRSTVWY");
// The base-64 alphabet (64 characters)
declare!(
    BASE64,
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
);
// The ASCII alphabet (0-127)
declare!(ASCII, 128);
// The extended ASCII alphabet (0-255)
declare!(EXTENDED_ASCII, 256);
// The Unicode 16 alphabet (0-65,535)
declare!(UNICODE16, 65535);

pub struct Alphabet {
    alphabet: Vec<char>,
    inverse: Vec<i32>,
    radix: usize,
}

impl TryFrom<&str> for Alphabet {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let alphabet: Vec<char> = s.chars().collect();
        let mut unicode = vec![false; MAX_VALUE];
        for &c in &alphabet {
            if unicode[c as usize] {
                return Err(format!("Illegal alphabet: repeated character = {:?}", c));
            }
            unicode[c as usize] = true; //set code point
        }

        let radix = alphabet.len();
        let mut inverse = vec![-1; MAX_VALUE];
        for i in 0..radix {
            inverse[alphabet[i] as usize] = i as i32;
        }

        Ok(Self {
            alphabet,
            inverse,
            radix,
        })
    }
}

impl TryFrom<u16> for Alphabet {
    type Error = ();

    fn try_from(radix: u16) -> Result<Self, Self::Error> {
        let inverse = (0..=radix as i32).collect();
        let alphabet: Vec<_> = (0..=radix).collect();
        let alphabet = String::from_utf16_lossy(&alphabet).chars().collect();
        Ok(Self {
            alphabet,
            inverse,
            radix: radix as usize,
        })
    }
}

impl Alphabet {
    /// Returns the index corresponding to the argument character
    pub fn to_index(&self, c: char) -> Option<&i32> {
        self.inverse.get(c as usize)
    }

    /// Returns the indices corresponding to the argument characters
    pub fn to_indices(&self, s: &str) -> Vec<i32> {
        s.chars()
            .map(|c| self.to_index(c).unwrap())
            .cloned()
            .collect()
    }

    /// Returns the character corresponding to the argument index
    pub fn to_char(&self, i: i32) -> Option<&char> {
        self.alphabet.get(i as usize)
    }

    /// Returns the characters corresponding to the argument indices
    pub fn to_chars(&self, indices: &[i32]) -> String {
        let mut buf = String::new();
        for &i in indices {
            if let Some(&c) = self.to_char(i) {
                buf.push(c);
            }
        }
        buf
    }

    /// Returns true if the argument is a character in this alphabet
    pub fn contains(&self, c: char) -> bool {
        self.inverse.get(c as usize) != Some(&-1)
    }

    /// Returns the number of characters in this alphabet (the radix)
    pub fn radix(&self) -> usize {
        self.radix
    }

    /// Returns the binary logarithm of the number of characters in this alphabet
    pub fn lg_r(&self) -> usize {
        let mut lg_r = 0;
        let mut t = self.radix - 1;
        while t >= 1 {
            lg_r += 1;
            t /= 2;
        }
        lg_r
    }
}
