use algo::common::{MaxPQ, MinPQ, Stack};

const TOBE: &'static str = include_str!("res/common/tobe.txt");

#[test]
fn stack() {
    let v: Vec<&str> = TOBE.split(' ').collect();
    let mut stack = Stack::new();
    let mut r = Vec::new();
    for s in v {
        match s {
            "-" => {
                if let Some(s) = stack.pop() {
                    r.push(s);
                }
            }
            _ => stack.push(s),
        }
    }

    assert_eq!("to be not that or be", r.join(" "));
    assert_eq!(2, stack.len());
}

#[test]
fn min_pq() {
    //case-1, test insert & del_min
    let t = vec![4, 1, 3, 2, 16, 9, 10, 14, 8, 7];
    let mut heap = MinPQ::with_capacity(t.len());
    for v in &t {
        heap.insert(*v);
    }
    assert!(!heap.is_empty());
    assert_eq!(t.len(), heap.len());
    assert_eq!(Some(1), heap.min().cloned());
    for v in vec![1, 2, 3, 4, 7, 8, 9, 10, 14, 16] {
        assert_eq!(heap.del_min(), Some(v));
    }
    assert!(heap.is_empty());
    assert_eq!(0, heap.len());
    assert_eq!(None, heap.min());
    assert_eq!(heap.del_min(), None);

    //case-2, test MinPQ::from_vec
    let t = vec![4, 1, 3, 2, 16, 9, 10, 14, 8, 7];
    let mut heap = MinPQ::from_vec(t);
    for v in vec![1, 2, 3, 4, 7, 8, 9, 10, 14, 16] {
        assert_eq!(heap.del_min(), Some(v));
    }
    assert_eq!(heap.del_min(), None);
}

#[test]
fn max_pq() {
    //case-1, test insert & del_max
    let t = vec![4, 1, 3, 2, 16, 9, 10, 14, 8, 7];
    let mut heap = MaxPQ::with_capacity(t.len());
    for v in &t {
        heap.insert(*v);
    }
    assert!(!heap.is_empty());
    assert_eq!(t.len(), heap.len());
    assert_eq!(Some(16), heap.max().cloned());
    for v in vec![16, 14, 10, 9, 8, 7, 4, 3, 2, 1] {
        assert_eq!(heap.del_max(), Some(v));
    }
    assert!(heap.is_empty());
    assert_eq!(0, heap.len());
    assert_eq!(None, heap.max());
    assert_eq!(heap.del_max(), None);

    //case-2, test MaxPQ::from_vec
    let t = vec![4, 1, 3, 2, 16, 9, 10, 14, 8, 7];
    let mut heap = MaxPQ::from_vec(t);
    for v in vec![16, 14, 10, 9, 8, 7, 4, 3, 2, 1] {
        assert_eq!(heap.del_max(), Some(v));
    }
    assert_eq!(heap.del_max(), None);
}
