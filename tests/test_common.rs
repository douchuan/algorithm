use algo::common::{
    IndexPQ, Queue, QuickFindUF, QuickUnionUF, Stack, TopM, WeightedQuickUnionUF, PQ, UF,
};
use std::str::FromStr;

const TOBE: &'static str = include_str!("../res/common/tobe.txt");
const TINY_UF: &'static str = include_str!("../res/common/tinyUF.txt");
// const M_UF: &'static str = include_str!("../res/common/mediumUF.txt");

#[test]
fn queue() {
    let v: Vec<&str> = TOBE.split(' ').collect();
    let mut queue = Queue::default();
    let mut r = Vec::new();
    for s in v {
        match s {
            "-" => {
                if let Some(s) = queue.dequeue() {
                    r.push(s);
                }
            }
            _ => queue.enqueue(s),
        }
    }

    assert_eq!("to be or not to be", r.join(" "));
    assert_eq!(2, queue.len());
}

#[test]
fn stack() {
    let v: Vec<&str> = TOBE.split(' ').collect();
    let mut stack = Stack::default();
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
    let mut min_pq = PQ::new_min_pq(t.len());
    for v in &t {
        min_pq.enqueue(*v);
    }
    assert!(!min_pq.is_empty());
    assert_eq!(t.len(), min_pq.len());
    assert_eq!(Some(1), min_pq.peek().cloned());
    for v in vec![1, 2, 3, 4, 7, 8, 9, 10, 14, 16] {
        assert_eq!(min_pq.dequeue(), Some(v));
    }
    assert!(min_pq.is_empty());
    assert_eq!(0, min_pq.len());
    assert_eq!(None, min_pq.peek());
    assert_eq!(min_pq.dequeue(), None);
}

#[test]
fn max_pq() {
    //case-1, test insert & del_max
    let t = vec![4, 1, 3, 2, 16, 9, 10, 14, 8, 7];
    let mut max_pq = PQ::new_max_pq(t.len());
    for v in &t {
        max_pq.enqueue(*v);
    }
    assert!(!max_pq.is_empty());
    assert_eq!(t.len(), max_pq.len());
    assert_eq!(Some(16), max_pq.peek().cloned());
    for v in vec![16, 14, 10, 9, 8, 7, 4, 3, 2, 1] {
        assert_eq!(max_pq.dequeue(), Some(v));
    }
    assert!(max_pq.is_empty());
    assert_eq!(0, max_pq.len());
    assert_eq!(None, max_pq.peek());
    assert_eq!(max_pq.dequeue(), None);
}

#[test]
fn index_min_pq() {
    fn queue(i: &str) -> Queue<&str> {
        let mut queue = Queue::default();
        for s in i.split_whitespace() {
            queue.enqueue(s);
        }
        queue
    }
    let mut inputs = vec![
        queue("A B C F G I I Z"),
        queue("B D H P Q Q"),
        queue("A B E F J N"),
    ];

    // init
    let mut pq = IndexPQ::new_min_pq(inputs.len());
    for i in 0..inputs.len() {
        if let Some(v) = inputs[i].dequeue() {
            let _ = pq.enqueue(i, v);
        }
    }

    let mut vec = Vec::new();
    while !pq.is_empty() {
        vec.push(pq.peek_key().unwrap().to_string());
        let i = pq.dequeue().unwrap();
        if let Some(v) = inputs[i as usize].dequeue() {
            let _ = pq.enqueue(i as usize, v);
        }
    }
    assert_eq!("A A B B B C D E F F G H I I J N P Q Q Z", vec.join(" "));
}

#[test]
fn index_max_pq() {
    fn queue(i: &str) -> Queue<&str> {
        let mut queue = Queue::default();
        for s in i.split_whitespace() {
            queue.enqueue(s);
        }
        queue
    }
    let mut inputs = vec![
        queue("A B C F G I I Z"),
        queue("B D H P Q Q"),
        queue("A B E F J N"),
    ];

    // init
    let mut pq = IndexPQ::new_max_pq(inputs.len());
    for i in 0..inputs.len() {
        if let Some(v) = inputs[i].dequeue() {
            let _ = pq.enqueue(i, v);
        }
    }

    let mut vec = Vec::new();
    while !pq.is_empty() {
        vec.push(pq.peek_key().unwrap().to_string());
        let i = pq.dequeue().unwrap();
        if let Some(v) = inputs[i as usize].dequeue() {
            let _ = pq.enqueue(i as usize, v);
        }
    }
    assert_eq!("B D H P Q Q A B E F J N A B C F G I I Z", vec.join(" "));
}

#[test]
fn quick_find_uf() {
    let i = TINY_UF;

    let uf = QuickFindUF::from_str(i).unwrap();
    assert_eq!(2, uf.count());

    let uf = QuickUnionUF::from_str(i).unwrap();
    assert_eq!(2, uf.count());

    let uf = WeightedQuickUnionUF::from_str(i).unwrap();
    assert_eq!(2, uf.count());

    let uf = UF::from_str(i).unwrap();
    assert_eq!(2, uf.count());
}

#[test]
fn top_m() {
    let mut top = TopM::new(5);
    for v in 0..100 {
        top.insert(v);
    }
    assert_eq!(vec![99, 98, 97, 96, 95], top.into_vec());
}
