/// The MinPQ represents a priority queue of generic keys.
/// It supports the usual insert and delete-the-minimum
/// operations, along with methods for peeking at the minimum key,
/// testing if the priority queue is empty, and iterating through
/// the keys.
///
/// This implementation uses a binary heap.
/// The insert and delete-the-minimum operations take
/// O(log(n)) amortized time, where n is the number
/// of elements in the priority queue. This is an amortized bound
/// (and not a worst-case bound) because of array resizing operations.
/// The min, size, and is-empty operations take
/// O(1) time in the worst case.
/// Construction takes time proportional to the specified capacity or the
/// number of items used to initialize the data structure.
pub struct MinPQ<T> {
    pq: Vec<T>, // pq的index是从1开始计算的, 0号元素未被使用
    n: usize,
}

/// The MaxPQ represents a priority queue of generic keys.
/// It supports the usual insert and delete-the-max
/// operations, along with methods for peeking at the max key,
/// testing if the priority queue is empty, and iterating through
/// the keys.
///
/// This implementation uses a binary heap.
/// The insert and delete-the-max operations take
/// O(log(n)) amortized time, where n is the number
/// of elements in the priority queue. This is an amortized bound
/// (and not a worst-case bound) because of array resizing operations.
/// The max, size, and is-empty operations take
/// O(1) time in the worst case.
/// Construction takes time proportional to the specified capacity or the
/// number of items used to initialize the data structure.
pub struct MaxPQ<T> {
    pq: Vec<T>, // pq的index是从1开始计算的, 0号元素未被使用
    n: usize,
}

impl<T: Default> MinPQ<T> {
    pub fn new() -> Self {
        Self::with_capacity(1)
    }

    pub fn with_capacity(cap: usize) -> Self {
        let mut pq = Self {
            pq: Vec::with_capacity(cap + 1),
            n: 0,
        };

        pq.pq.push(T::default());

        pq
    }

    /// Returns true if this priority queue is empty
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// Returns the number of keys on this priority queue
    pub fn len(&self) -> usize {
        self.n
    }

    /// Returns a smallest key on this priority queue
    pub fn min(&self) -> Option<&T> {
        self.pq.get(1)
    }
}

impl<T: PartialOrd + Default> MinPQ<T> {
    pub fn from_vec(keys: Vec<T>) -> Self {
        let n = keys.len();
        let mut pq = Self {
            pq: Vec::with_capacity(n + 1),
            n,
        };

        pq.pq.push(T::default());

        for k in keys {
            pq.pq.push(k);
        }

        let mut k = n / 2;
        while k >= 1 {
            pq.sink(k);
            k -= 1;
        }

        pq
    }

    /// Adds a new key to this priority queue
    pub fn insert(&mut self, x: T) {
        self.pq.push(x);
        self.n += 1;
        self.swim(self.n);
    }

    /// Removes and returns a smallest key on this priority queue
    pub fn del_min(&mut self) -> Option<T> {
        if self.n == 0 {
            None
        } else {
            self.pq.swap(1, self.n);
            self.n -= 1;
            self.sink(1);
            self.pq.pop()
        }
    }

    fn swim(&mut self, mut k: usize) {
        while k > 1 && self.pq[k / 2] > self.pq[k] {
            self.pq.swap(k, k / 2);
            k /= 2;
        }
    }

    fn sink(&mut self, mut k: usize) {
        let n = self.n;
        while 2 * k <= n {
            let mut j = 2 * k;
            if j < n && self.pq[j] > self.pq[j + 1] {
                j += 1;
            }
            if self.pq[k] <= self.pq[j] {
                break;
            }
            self.pq.swap(k, j);
            k = j;
        }
    }
}

impl<T: Default> MaxPQ<T> {
    pub fn new() -> Self {
        Self::with_capacity(1)
    }

    pub fn with_capacity(cap: usize) -> Self {
        let mut pq = Self {
            pq: Vec::with_capacity(cap + 1),
            n: 0,
        };

        pq.pq.push(T::default());

        pq
    }

    /// Returns true if this priority queue is empty
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// Returns the number of keys on this priority queue
    pub fn len(&self) -> usize {
        self.n
    }

    /// Returns a largest key on this priority queue
    pub fn max(&self) -> Option<&T> {
        self.pq.get(1)
    }
}

impl<T: PartialOrd + Default> MaxPQ<T> {
    pub fn from_vec(keys: Vec<T>) -> Self {
        let n = keys.len();
        let mut pq = Self {
            pq: Vec::with_capacity(n + 1),
            n,
        };

        pq.pq.push(T::default());

        for k in keys {
            pq.pq.push(k);
        }

        let mut k = n / 2;
        while k >= 1 {
            pq.sink(k);
            k -= 1;
        }

        pq
    }

    /// Adds a new key to this priority queue
    pub fn insert(&mut self, x: T) {
        self.pq.push(x);
        self.n += 1;
        self.swim(self.n);
    }

    /// Removes and returns a largest key on this priority queue
    pub fn del_max(&mut self) -> Option<T> {
        if self.n == 0 {
            None
        } else {
            self.pq.swap(1, self.n);
            self.n -= 1;
            self.sink(1);
            self.pq.pop()
        }
    }

    fn swim(&mut self, mut k: usize) {
        while k > 1 && self.pq[k / 2] < self.pq[k] {
            self.pq.swap(k, k / 2);
            k /= 2;
        }
    }

    fn sink(&mut self, mut k: usize) {
        let n = self.n;
        while 2 * k <= n {
            let mut j = 2 * k;
            if j < n && self.pq[j] < self.pq[j + 1] {
                j += 1;
            }
            if self.pq[k] >= self.pq[j] {
                break;
            }
            self.pq.swap(k, j);
            k = j;
        }
    }
}
