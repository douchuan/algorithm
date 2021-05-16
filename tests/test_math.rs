use algo::math;

#[test]
fn sqrt_binary_search() {
    let epsilon = 0.01;
    let valus: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    for v in valus {
        assert!((v.sqrt() - math::mysqrt::sqrt_binary_search(v)) <= epsilon);
    }
}

#[test]
fn sqrt_newton() {
    let epsilon = 0.01;
    let valus: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    for v in valus {
        assert!(
            (v.sqrt() - math::mysqrt::sqrt_newton(v)) <= epsilon,
            "std sqrt = {}, sqrt_newton = {}",
            v.sqrt(),
            math::mysqrt::sqrt_newton(v)
        );
    }
}
