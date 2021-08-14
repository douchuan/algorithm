/// The LSD provides static methods for sorting an
/// array of w-character strings or 32-bit integers
/// using LSD radix sort.
pub struct LSD;

impl LSD {
    /// Rearranges the array of w-character strings in ascending order.
    /// `a` the array to be sorted
    /// `w` the number of characters per string
    ///
    /// todo: support elements generic type, sort<T: AsRef<str>>
    pub fn sort(a: &mut [&str], w: usize) {
        let n = a.len();
        #[allow(non_snake_case)]
        let R = 256; // extend ASCII alphabet size
        let mut aux = vec![""; n];

        for d in (0..w).rev() {
            // sort by key-indexed counting on dth character

            // compute frequency counts
            let mut count = vec![0; R + 1];
            for i in 0..n {
                let c = a[i].chars().nth(d).unwrap();
                count[c as usize + 1] += 1;
            }

            // compute cumulates
            for r in 0..R {
                count[r + 1] += count[r];
            }

            // move data
            for i in 0..n {
                let c = a[i].chars().nth(d).unwrap();
                aux[count[c as usize]] = a[i];
                count[c as usize] += 1;
            }

            // copy back
            for i in 0..n {
                a[i] = aux[i];
            }
        }
    }
}
