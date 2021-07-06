#[macro_use]
extern crate approx;

use algo::tree::binary::sparse_vector::SparseVector;

#[test]
fn dot() {
    let (a, b) = create();
    let _ = abs_diff_eq!(a.dot(&b).unwrap(), 0.5 * 0.6);
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
