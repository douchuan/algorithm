#[test]
fn vec_random_access() {
    // Allocate vector big enough for 4 elements.
    let size = 4;
    let mut x: Vec<usize> = Vec::with_capacity(size);
    let x_ptr = x.as_mut_ptr();

    unsafe {
        for v in 0..4 {
            *x_ptr.add(v) = v;
        }
        x.set_len(size);
    }
    assert_eq!(&*x, &[0, 1, 2, 3]);

    // 扩充
    x.reserve(4);
    let x_ptr = x.as_mut_ptr();
    unsafe {
        for v in 4..8 {
            *x_ptr.add(v) = v;
        }
        x.set_len(8)
    }
    assert_eq!(&*x, &[0, 1, 2, 3, 4, 5, 6, 7]);
}
