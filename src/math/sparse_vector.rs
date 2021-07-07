/// The SparseVector represents a d dimensional mathematical vector.
/// Vectors are mutable: their values can be changed after they are created.
/// It includes methods for addition, subtraction,
/// dot product, scalar product, unit vector, and Euclidean norm.
///
/// The implementation is a symbol table (Red Black Tree) of indices and values
/// for which the vector coordinates are nonzero. This makes it efficient when
/// most of the vector coordindates are zero.
///
/// ref: https://github.com/kevin-wayne/algs4.git
use crate::tree::binary::rb2::RedBlackTreeV2;
use crate::tree::binary::Tree;
use std::ops::{Add, Sub};

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
        *self.st.get(&i).unwrap_or(&0.0)
    }

    /// Returns the number of nonzero entries in this vector.
    pub fn nnz(&self) -> usize {
        self.st.size()
    }

    /// Returns the dimension of this vector.
    pub fn dimension(&self) -> usize {
        self.d
    }

    /// Returns the inner product of this vector with the specified vector.
    pub fn dot(&self, that: &Self) -> Result<f64, Err> {
        if self.d != that.d {
            Err(Err::Dimension)
        } else {
            let keys = if self.nnz() <= that.nnz() {
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

            Ok(sum)
        }
    }

    /// Returns the magnitude of this vector.
    /// This is also known as the L2 norm or the Euclidean norm.
    pub fn magnitude(&self) -> f64 {
        self.dot(self).unwrap().sqrt()
    }

    /// Returns the scalar-vector product of this vector with the specified scalar.
    pub fn scale(&self, alpha: f64) -> Self {
        let mut c = Self::new(self.d);
        for &i in self.st.keys() {
            c.put(i, alpha * self.get(i));
        }
        c
    }
}

impl Add for SparseVector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut c = self.clone();
        for &i in rhs.st.keys() {
            c.put(i, c.get(i) + rhs.get(i));
        }
        c
    }
}

impl Sub for SparseVector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut c = self.clone();
        for &i in rhs.st.keys() {
            c.put(i, c.get(i) - rhs.get(i));
        }
        c
    }
}

impl ToString for SparseVector {
    fn to_string(&self) -> String {
        let keys = self.st.keys();
        let mut v = Vec::with_capacity(keys.len());
        for &i in keys {
            v.push(format!("({}, {})", i, self.get(i)));
        }
        v.join("")
    }
}

impl Clone for SparseVector {
    fn clone(&self) -> Self {
        let mut c = Self::new(self.d);
        for &i in self.st.keys() {
            c.put(i, self.get(i));
        }
        c
    }
}

#[derive(Debug)]
pub enum Err {
    Dimension,
}
