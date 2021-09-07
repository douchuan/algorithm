#![allow(clippy::many_single_char_names)]
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

use crate::common::binary_tree;

/// 用数组实现隐式二叉堆 (最大堆)
pub struct BinaryHeap<K> {
    keys: Vec<K>, // 索引从0开始
}

impl<K> BinaryHeap<K>
where
    K: Ord,
{
    pub fn new(mut keys: Vec<K>) -> Self {
        build_heap(&mut keys);
        Self { keys }
    }

    pub fn pop(&mut self) -> Option<K> {
        let len = self.keys.len();
        if len > 0 {
            // 从长度为 n 的数组中删除第一个元素需要线性时间 O(n)。
            // 这是因为我们需要将所有剩余的元素依次向前移动一个位置。
            // 这一操作成为了整个算法的瓶颈，使得算法的复杂度升高了。
            // 为了解决这一问题，我们可以交换数组中的第一个和最后一
            // 个元素，然后将数组的长度减一。
            self.keys.swap(0, len - 1);
            let key = self.keys.pop();
            heapify(&mut self.keys, 0);
            key
        } else {
            None
        }
    }

    pub fn set(&mut self, i: usize, key: K) {
        match self.keys.get(i) {
            Some(v) if &key >= v => {
                self.keys[i] = key;
                heap_fix(&mut self.keys, i);
            }
            _ => (),
        }
    }

    pub fn insert(&mut self, key: K) {
        let i = self.keys.len();
        self.keys.push(key);
        heap_fix(&mut self.keys, i);
    }

    //for test
    pub fn keys_slice(&self) -> &[K] {
        self.keys.as_slice()
    }
}

pub fn heapify<K>(keys: &mut [K], mut i: usize)
where
    K: Ord,
{
    let n = keys.len();
    loop {
        let l = binary_tree::left(i);
        let r = binary_tree::right(i);
        let mut m = i;

        if let Some(v) = keys.get(l) {
            if l < n && v >= &keys[m] {
                m = l;
            }
        }

        if let Some(v) = keys.get(r) {
            if r < n && v >= &keys[m] {
                m = r;
            }
        }

        if m != i {
            keys.swap(i, m);
            i = m;
        } else {
            break;
        }
    }
}

pub fn build_heap<K>(keys: &mut [K])
where
    K: Ord,
{
    // i以 n / 2作为第一个分支节点，开始构建heap。
    // 因为叶子结点，已经满足堆定义，所以从二叉树倒数第二层最后一个节点
    // 开始构建，这个分支节点的index，根据等比数列求和公式得到:
    // (2 ^ 0 + 2 ^ 1 ... 2 ^ (p - 1) = 2 ^ (p - 1) - 1)
    // p为二叉树层数等于log(n)
    // index = 2 ^ (p - 1) - 1 = 2 ^ ( log(n) - 1) - 1 <= n / 2
    let mut i = keys.len() as i32 / 2;
    while i >= 0 {
        heapify(keys, i as usize);
        i -= 1;
    }
}

// 与heapify的区别:
// heapify 是从i节点开始，调整子树 (向下调整)
// heap_fix 是从i节点开始，调整父节点（向上调整）
fn heap_fix<K>(keys: &mut [K], mut i: usize)
where
    K: Ord,
{
    while i > 0 {
        let parent = binary_tree::parent(i);
        if keys[i] >= keys[parent] {
            keys.swap(i, parent);
            i = parent;
        }
    }
}
