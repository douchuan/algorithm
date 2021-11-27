use algo::math;

#[test]
fn sqrt() {
    // let epsilon = f64::EPSILON;
    let epsilon = 0.01;
    let valus: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    for v in valus {
        assert!((v.sqrt() - math::mysqrt::sqrt_binary_search(v)) <= epsilon);
        assert!((v.sqrt() - math::mysqrt::sqrt_newton(v)) <= epsilon);
        assert!((v.sqrt() - math::mysqrt::sqrt3(v)) <= epsilon);
    }
}
