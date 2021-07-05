use crate::tree::binary::rb2::RedBlackTreeV2;
use crate::tree::binary::Tree;

pub enum Err {
    IllegalDimension,
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
        self.st.find(&i).cloned().unwrap_or(0.0)
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
            return Err(Err::IllegalDimension);
        }

        let mut sum = 0.0;
        if self.st.size() <= that.st.size() {}

        unimplemented!()
    }
}
