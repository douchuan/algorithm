//! 堆
//!
//! 堆可以用于解决很多实际问题，包括排序、带有优先级的调度，实现图算法等等
//!
//! 堆是一种满足如下性质的数据结构:
//!   - 顶部(top)总是保存着最小(或最大)的元素
//!   - 弹出(pop)操作将顶部元素移除，同时保持堆的性质，新的顶部元素仍然是剩余元素中的最小(或 最大)值
//!   - 将新元素插入到堆中仍然保持堆的性质，顶部元素还是所有元素中的最小(或最大)值
//!   - 其他操作(例如将两个堆合并)，都会保持堆的性质
//!
//! 顶部保存最小元素的堆为最小堆，顶部保存最大元素的堆为最大堆

// 用数组实现隐式二叉堆
pub struct BinaryHeap<T, F> {
    data: Vec<T>,
    test: F,
}

impl<T, F> BinaryHeap<T, F>
where
    T: Copy,
    F: Fn(T, T) -> bool + Copy,
{
    pub fn new(mut data: Vec<T>, test: F) -> Self {
        build_heap(&mut data, test);
        Self { data, test }
    }

    pub fn from_slice(a: &[T], test: F) -> Self {
        let mut data = Vec::new();
        data.extend_from_slice(a);
        Self::new(data, test)
    }

    pub fn pop(&mut self) -> Option<T> {
        let len = self.data.len();
        if len > 0 {
            //从长度为 n 的数组中删除第一个元素需要线性时间 O(n)。
            // 这是因为我们需要将所有剩余的元素依次 向前移动一位。
            // 这一操作成为了整个算法的瓶颈，使得算法的复杂度升高了。
            // 为了解决这一问题，我们可以交换数组中的第一个和最后一
            // 个元素，然后将数组的长度减一。
            self.data.swap(0, len - 1);
            let v = self.data.pop();
            let test = self.test;
            heapify(&mut self.data, 0, test);
            v
        } else {
            None
        }
    }

    pub fn set(&mut self, i: usize, vv: T) {
        if self.data.get(i).is_none() {
            return;
        }

        let test = self.test;
        let v = self.data[i];
        if test(vv, v) {
            self.data[i] = vv;
            heap_fix(&mut self.data, i, test);
        }
    }

    pub fn insert(&mut self, v: T) {
        let test = self.test;
        let i = self.data.len();
        self.data.push(v);
        heap_fix(&mut self.data, i, test);
    }

    //for test
    pub fn get_data(&self) -> &[T] {
        self.data.as_slice()
    }
}

// 索引从 0 开始
fn left(i: usize) -> usize {
    (i << 1) + 1
}

fn right(i: usize) -> usize {
    (i + 1) << 1
}

fn parent(i: usize) -> usize {
    ((i + 1) >> 1) - 1
}

pub fn heapify<T, F>(a: &mut [T], mut i: usize, test: F)
where
    T: Copy,
    F: Fn(T, T) -> bool + Copy,
{
    let n = a.len();
    loop {
        let l = left(i);
        let r = right(i);
        let mut m = i;

        if let Some(v) = a.get(l) {
            if l < n && test(*v, a[i]) {
                m = l;
            }
        }

        if let Some(v) = a.get(r) {
            if r < n && test(*v, a[m]) {
                m = r;
            }
        }

        if m != i {
            a.swap(i, m);
            i = m;
        } else {
            break;
        }
    }
}

pub fn build_heap<T, F>(a: &mut [T], test: F)
where
    T: Copy,
    F: Fn(T, T) -> bool + Copy,
{
    let len = a.len();
    // i以 n / 2作为第一个分支节点，开始构建heap。
    // 因为叶子结点，已经满足堆定义，所以从二叉树倒数第二层最后一个节点
    // 开始构建，这个分支节点的index，根据等比数列求和公式得到:
    // (2 ^ 0 + 2 ^ 1 ... 2 ^ (p - 1) = 2 ^ (p - 1) - 1)
    // p为二叉树层数等于log(n)
    // index = 2 ^ (p - 1) - 1 = 2 ^ ( log(n) - 1) - 1 <= n / 2
    let mut i = len / 2;
    loop {
        heapify(a, i, test);
        if i == 0 {
            break;
        } else {
            i -= 1;
        }
    }
}

// 与heapify的区别:
// heapify 是从i节点开始，调整子树 (向下调整)
// heap_fix 是从i节点开始，调整父节点（向上调整）
fn heap_fix<T: Copy, F>(a: &mut [T], mut i: usize, test: F)
where
    F: Fn(T, T) -> bool + Copy,
{
    while i > 0 {
        let parent = parent(i);
        if test(a[i], a[parent]) {
            a.swap(i, parent);
            i = parent;
        }
    }
}
