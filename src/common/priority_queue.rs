use std::cmp::Ordering;

/// Often, we collect a set of items, then process the one with the
/// largest key, then perhaps collect more items, then process the
/// one with the current largest key, and so forth. For example, you
/// are likely to have a computer (or a cellphone) that is capable of
/// running several applications at the same time. This effect is typically
/// achieved by assigning a priority to events associated with applications,
/// then always choosing to process next the highest-priority event. For
/// example, most cellphones are likely to process an incoming call with
/// higher priority than a game application.

type Comparator<T> = Box<dyn Fn(&T, &T) -> bool>;
// type Comparator2<T> = Box<dyn Fn(&Option<T>, &Option<T>) -> bool>;

pub struct PQ<T> {
    pq: Vec<T>, // 为了计算方便, pq的index是从1开始计算的, 0号元素未被使用
    n: usize,
    comparator: Comparator<T>,
}

pub struct IndexPQ<T> {
    // max_n: usize, // maximum number of elements on PQ
    n: usize,     // number of elements on PQ
    pq: Vec<i32>, // binary heap using 1-based indexing
    qp: Vec<i32>, // inverse of pq, qp[pq[i]] = pq[qp[i]] = i
    keys: Vec<Option<T>>,
    comparator: Comparator<Option<T>>,
}

impl<T: PartialOrd + Default> PQ<T> {
    /// The MinPQ represents a priority queue of generic keys.
    /// It supports the usual insert and delete-the-minimum
    /// operations, along with methods for peeking at the minimum key,
    /// testing if the priority queue is empty, and iterating through
    /// the keys.
    pub fn new_min_pq(cap: usize) -> Self {
        let comparator = Box::new(|a: &T, b: &T| a.gt(b));
        Self::new(cap, comparator)
    }

    /// The MaxPQ represents a priority queue of generic keys.
    /// It supports the usual insert and delete-the-max
    /// operations, along with methods for peeking at the max key,
    /// testing if the priority queue is empty, and iterating through
    /// the keys.
    pub fn new_max_pq(cap: usize) -> Self {
        let comparator = Box::new(|a: &T, b: &T| a.lt(b));
        Self::new(cap, comparator)
    }

    fn new(cap: usize, comparator: Comparator<T>) -> Self {
        let mut pq = Self {
            pq: Vec::with_capacity(cap + 1),
            n: 0,
            comparator,
        };

        // 虽然此处初始化了0号元素, 但0号元素并不参与算法过程，
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

    /// MinPQ: Returns a smallest key on this priority queue
    /// MaxPQ: Returns a largest key on this priority queue
    pub fn peek(&self) -> Option<&T> {
        self.pq.get(1)
    }

    /// Adds a new key to this priority queue
    pub fn enqueue(&mut self, x: T) {
        self.pq.push(x);
        self.n += 1;
        self.swim(self.n);
    }

    /// MinPQ: Removes and returns a smallest key on this priority queue
    /// MaxPQ: Removes and returns a largest key on this priority queue
    pub fn dequeue(&mut self) -> Option<T> {
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
        while k > 1 && self.compare(k / 2, k) {
            self.pq.swap(k, k / 2);
            k /= 2;
        }
    }

    fn sink(&mut self, mut k: usize) {
        let n = self.n;
        while 2 * k <= n {
            let mut j = 2 * k;
            if j < n && self.compare(j, j + 1) {
                j += 1;
            }
            if !self.compare(k, j) {
                break;
            }
            self.pq.swap(k, j);
            k = j;
        }
    }

    fn compare(&self, i: usize, j: usize) -> bool {
        (self.comparator)(&self.pq[i], &self.pq[j])
    }
}

impl<T: PartialOrd + Copy + Clone> IndexPQ<T> {
    pub fn new_min_pq(max_n: usize) -> Self {
        let comparator = Box::new(|a: &Option<T>, b: &Option<T>| a.gt(b));
        Self::new(max_n, comparator)
    }

    pub fn new_max_pq(max_n: usize) -> Self {
        let comparator = Box::new(|a: &Option<T>, b: &Option<T>| a.lt(b));
        Self::new(max_n, comparator)
    }

    fn new(max_n: usize, comparator: Comparator<Option<T>>) -> Self {
        Self {
            // max_n,
            n: 0,
            pq: vec![-1; max_n + 1],
            qp: vec![-1; max_n + 1],
            keys: vec![None; max_n + 1],
            comparator,
        }
    }

    /// Returns true if this priority queue is empty
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// Returns the number of keys on this priority queue
    pub fn len(&self) -> usize {
        self.n
    }

    /// Is i an index on this priority queue?
    pub fn contains(&self, i: usize) -> bool {
        self.qp[i] != -1
    }

    /// MinPQ: Returns an index associated with a minimum key
    /// MaxPQ: Returns an index associated with a maximum key
    pub fn peek_index(&self) -> Option<&i32> {
        self.pq.get(1)
    }

    /// MinPQ: Returns a minimum key
    /// MaxPQ: Returns a maximum key
    pub fn peek_key(&self) -> Option<&T> {
        self.pq.get(1).and_then(|&i| self.keys[i as usize].as_ref())
    }

    /// Associates key with index i
    pub fn enqueue(&mut self, i: usize, key: T) -> Result<(), &'static str> {
        if self.contains(i) {
            Err("index is already in the priority queue")
        } else {
            self.n += 1;
            self.qp[i] = self.n as i32;
            self.pq[self.n] = i as i32;
            self.keys[i] = Some(key);
            self.swim(self.n);
            Ok(())
        }
    }

    /// MinPQ: Removes a minimum key and returns its associated index
    /// MaxPQ: Removes a maximum key and returns its associated index
    pub fn dequeue(&mut self) -> Option<usize> {
        if self.n == 0 {
            None
        } else {
            let min = self.pq[1] as usize;
            self.exch(1, self.n);
            self.n -= 1;
            self.sink(1);
            debug_assert_eq!(self.pq[self.n + 1], min as i32);
            self.qp[min] = -1; // delete
            let _ = self.keys[min].take();
            self.pq[self.n + 1] = -1;
            Some(min)
        }
    }

    pub fn decrease_key(&mut self, i: usize, key: T) -> Result<(), &'static str> {
        if !self.contains(i) {
            Err("index is not in the priority queue")
        } else {
            match self.keys[i].partial_cmp(&Some(key)) {
                None => Err("Calling decreaseKey() with a key that comparison is impossible"),
                Some(Ordering::Equal) => Err("Calling decreaseKey() with a key equal to the key in the priority queue"),
                Some(Ordering::Less) => Err("Calling decreaseKey() with a key strictly greater than the key in the priority queue"),
                Some(Ordering::Greater) => {
                    self.keys[i] = Some(key);
                    self.swim(self.qp[i] as usize);
                    Ok(())
                }
            }
        }
    }

    fn swim(&mut self, mut k: usize) {
        while k > 1 && self.compare(k / 2, k) {
            self.exch(k, k / 2);
            k /= 2;
        }
    }

    fn sink(&mut self, mut k: usize) {
        while 2 * k <= self.n {
            let mut j = 2 * k;
            if j < self.n && self.compare(j, j + 1) {
                j += 1;
            }
            if !self.compare(k, j) {
                break;
            }
            self.exch(k, j);
            k = j;
        }
    }

    fn compare(&self, i: usize, j: usize) -> bool {
        (self.comparator)(
            &self.keys[self.pq[i] as usize],
            &self.keys[self.pq[j] as usize],
        )
    }

    fn exch(&mut self, i: usize, j: usize) {
        self.pq.swap(i, j);
        self.qp.swap(self.pq[i] as usize, self.pq[j] as usize);
    }
}
