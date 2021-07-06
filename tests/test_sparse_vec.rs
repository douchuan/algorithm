#[macro_use]
extern crate approx;

use algo::tree::binary::sparse_vector::SparseVector;

#[test]
fn dot() {
    let (a, b) = create();
    assert_relative_eq!(a.dot(&b).unwrap(), 0.5 * 0.6);
}

#[test]
fn magnitude() {
    let (a, _) = create();
    assert_relative_eq!(a.magnitude(), (0.5f64.powf(2.0) + 0.75f64.powf(2.0)).sqrt());
}

#[test]
fn scale() {
    let (a, _) = create();
    let c = a.scale(2.0);
    let expect = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.5];
    for (i, &v) in expect.iter().enumerate() {
        assert_relative_eq!(v, c.get(i),);
    }
}

#[test]
fn plus() {
    let (a, b) = create();
    let c = a.plus(&b);
    let expect = vec![0.0, 0.0, 0.0, 1.1, 0.9, 0.0, 0.0, 0.0, 0.0, 0.75];
    for (i, &v) in expect.iter().enumerate() {
        assert_relative_eq!(v, c.get(i),);
    }
}

#[test]
fn to_string() {
    let (a, _) = create();
    assert_eq!(a.to_string(), "(3, 0.5)(9, 0.75)");
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
