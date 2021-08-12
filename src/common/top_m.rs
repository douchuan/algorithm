use crate::common::PQ;

/// Find the largest M integers.
/// This implementation uses a MinPQ of size
/// at most m + 1 to identify the M largest elements.
pub struct TopM<T> {
    pq: PQ<T>,
    m: usize,
}

impl<T: PartialOrd + Default> TopM<T> {
    pub fn new(m: usize) -> Self {
        Self {
            pq: PQ::new_min_pq(m + 1),
            m,
        }
    }

    pub fn insert(&mut self, v: T) {
        self.pq.enqueue(v);
        // remove minimum if m+1 entries on the PQ
        if self.pq.len() > self.m {
            let _ = self.pq.dequeue();
        }
        // top m entries are on the PQ
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
