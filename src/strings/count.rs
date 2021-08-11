use crate::strings::Alphabet;

/// computing the frequency of occurrence of each
/// character over a given alphabet
pub struct Count;

impl Count {
    pub fn compute(alphabet: &Alphabet, s: &str) -> Vec<usize> {
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
