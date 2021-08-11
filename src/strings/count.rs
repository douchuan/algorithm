use crate::strings::Alphabet;
use std::convert::TryFrom;

/// computing the frequency of occurrence of each
/// character over a given alphabet
pub struct Count {
    alphabet: Alphabet,
}

impl Count {
    pub fn compute(&self, s: &str) -> Vec<usize> {
        let alphabet = &self.alphabet;
        let mut count = vec![0; alphabet.radix()];
        for c in s.chars() {
            if let Some(&i) = alphabet.to_index(c) {
                if i >= 0 {
                    count[i as usize] += 1;
                }
            }
        }
        count
    }
}

impl TryFrom<&str> for Count {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let alphabet = Alphabet::try_from(s)?;
        Ok(Self { alphabet })
    }
}
