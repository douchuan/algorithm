/// The SparseVector represents a d dimensional mathematical vector.
/// Vectors are mutable: their values can be changed after they are created.
/// It includes methods for addition, subtraction,
/// dot product, scalar product, unit vector, and Euclidean norm.
///
/// The implementation is a symbol table of indices and values for which the vector
/// coordinates are nonzero. This makes it efficient when most of the vector coordindates
/// are zero.
///
/// ref: https://github.com/kevin-wayne/algs4.git
use crate::tree::binary::rb2::RedBlackTreeV2;
use crate::tree::binary::Tree;

#[derive(Debug)]
pub enum Err {
    Dimension,
}

pub struct SparseVector {
    d: usize,
    st: Tree<usize, f64>,
}

impl SparseVector {
    /// Initializes a d-dimensional zero vector.
    pub fn new(d: usize) -> Self {
        let st = Tree::default();
        Self { d, st }
    }

    /// Sets the ith coordinate of this vector to the specified value.
    pub fn put(&mut self, i: usize, v: f64) {
        if v == 0.0 {
            self.st.delete(&i);
        } else {
            self.st.insert(i, v);
        }
    }

    /// Returns the ith coordinate of this vector
    pub fn get(&self, i: usize) -> f64 {
        self.st.get(&i).cloned().unwrap_or(0.0)
    }

    /// Returns the number of nonzero entries in this vector.
    pub fn nnz(&self) -> usize {
        self.st.size()
    }

    /// Returns the dimension of this vector.
    pub fn dimension(&self) -> usize {
        self.d
    }

    pub fn dot(&self, that: &Self) -> Result<f64, Err> {
        if self.d != that.d {
            Err(Err::Dimension)
        } else {
            let keys = if self.st.size() <= that.st.size() {
                self.st.keys()
            } else {
                that.st.keys()
            };

            let sum = keys.iter().fold(0.0, |acc, &i| {
                let delta = match (self.st.get(i), that.st.get(i)) {
                    (Some(a), Some(b)) => a * b,
                    _ => 0.0,
                };
                acc + delta
            });

            /*
            let mut sum = 0.0;
            for i in keys {
                match (self.st.get(i), that.st.get(i)) {
                    (Some(a), Some(b)) => sum += a * b,
                    _ => (),
                }
            }
            */

            Ok(sum)
        }
    }
}
