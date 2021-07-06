#[macro_use]
extern crate approx;

use algo::tree::binary::sparse_vector::SparseVector;

#[test]
fn dot() {
    let (a, b) = create();
    relative_eq!(a.dot(&b).unwrap(), 0.5 * 0.6);
}

#[test]
fn plus() {
    let (a, b) = create();
    let c = a.plus(&b);
    let expect = vec![0.0, 0.0, 0.0, 1.1, 0.9, 0.0, 0.0, 0.0, 0.0, 0.75];
    for (i, &v) in expect.iter().enumerate() {
        relative_eq!(v, c.get(i));
    }
}

fn create() -> (SparseVector, SparseVector) {
    let mut a = SparseVector::new(10);
    let mut b = SparseVector::new(10);
    a.put(3, 0.5);
    a.put(9, 0.75);
    a.put(6, 0.11);
    a.put(6, 0.0);
    b.put(3, 0.6);
    b.put(4, 0.9);

    (a, b)
}
