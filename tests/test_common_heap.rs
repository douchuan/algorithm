use algo::common::heap;

#[test]
fn heapify() {
    //verify empty ok
    let mut t: Vec<i32> = vec![];
    let expect: Vec<i32> = vec![];
    heap::heapify(&mut t, 1);
    assert_eq!(t, expect, "t = {:?}, expect = {:?}", t, expect);

    //normal
    let t = vec![16, 4, 10, 14, 7, 9, 3, 2, 8, 1];
    let expect = vec![16, 14, 10, 8, 7, 9, 3, 2, 4, 1];
    let mut tt = t.clone();
    heap::heapify(&mut tt, 1);
    assert_eq!(tt, expect, "t = {:?}, expect = {:?}", t, expect);
}

#[test]
fn build_heap() {
    //verify empty ok
    let mut t: Vec<i32> = vec![];
    let expect: Vec<i32> = vec![];
    heap::build_heap(&mut t);
    assert_eq!(t, expect, "t = {:?}, expect = {:?}", t, expect);

    //normal
    let t = vec![4, 1, 3, 2, 16, 9, 10, 14, 8, 7];
    let expect = vec![16, 14, 10, 8, 7, 9, 3, 2, 4, 1];
    let mut tt = t.clone();
    heap::build_heap(&mut tt);
    assert_eq!(tt, expect, "t = {:?}, expect = {:?}", t, expect)
}

#[test]
fn pop() {
    let t = vec![4, 1, 3, 2, 16, 9, 10, 14, 8, 7];
    let mut heap = heap::BinaryHeap::new(t);
    for v in vec![16, 14, 10, 9, 8, 7, 4, 3, 2, 1] {
        assert_eq!(heap.pop(), Some(v));
    }
    assert_eq!(heap.pop(), None);
}

#[test]
fn set() {
    //only 1
    let t = vec![10];
    let mut heap = heap::BinaryHeap::new(t);
    // data layout:
    //   vec![10];
    heap.set(0, 100);
    let data = heap.keys_slice();
    assert_eq!(data, vec![100]);

    //set fail
    let t = vec![10];
    let mut heap = heap::BinaryHeap::new(t);
    // data layout:
    //   vec![10];
    heap.set(0, 9);
    let data = heap.keys_slice();
    assert_eq!(data, vec![10]);

    //normal
    let t = vec![4, 1, 3, 2, 16, 9, 10, 14, 8, 7];
    let mut heap = heap::BinaryHeap::new(t);
    // data layout:
    //   vec![16, 14, 10, 8, 7, 9, 3, 2, 4, 1];
    heap.set(3, 100);
    let data = heap.keys_slice();
    assert_eq!(data, vec![100, 16, 10, 14, 7, 9, 3, 2, 4, 1])
}

#[test]
fn insert() {
    //normal
    let t = vec![4, 1, 3, 2, 16, 9, 10, 14, 8, 7];
    let mut heap = heap::BinaryHeap::new(t);
    // data layout:
    //   vec![16, 14, 10, 8, 7, 9, 3, 2, 4, 1];
    heap.insert(100);
    let data = heap.keys_slice();
    assert_eq!(data, vec![100, 16, 10, 8, 14, 9, 3, 2, 4, 1, 7])
}
