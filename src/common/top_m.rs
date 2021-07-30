use crate::common::PQ;

/// Find the largest M integers.
/// This implementation uses a min PQ of size
/// at most m + 1 to identify the M largest elements
pub struct TopM<T> {
    pq: PQ<T>,
    m: usize,
}

impl<T: PartialOrd + Default> TopM<T> {
    pub fn new(m: usize) -> Self {
        Self {
            pq: PQ::new_min_pq(m),
            m,
        }
    }

    pub fn insert(&mut self, v: T) {
        self.pq.enqueue(v);
        if self.pq.len() > self.m {
            let _ = self.pq.dequeue();
        }
    }

    pub fn into_vec(mut self) -> Vec<T> {
        let mut vec = Vec::with_capacity(self.m);
        while let Some(v) = self.pq.dequeue() {
            vec.push(v);
        }
        vec.reverse();
        vec
    }
}
