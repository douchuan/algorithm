pub fn sqrt_binary_search(x: f64) -> f64 {
    assert!(x >= 0.0);
    let mut low = 0.0;
    let mut up = x;
    let mut last = 0.0;
    loop {
        let mid = (low + up) / 2.0;
        if (mid - last).abs() <= f64::EPSILON {
            return mid;
        }

        if mid * mid > x {
            up = mid;
        } else {
            low = mid;
        }

        last = mid;
    }
}

pub fn sqrt_newton(x: f64) -> f64 {
    assert!(x >= 0.0);
    if x == 0.0 {
        return x;
    }
    if (x - 1.0).abs() <= f64::EPSILON {
        return 1.0;
    }
    let mut last = 0.0;
    let mut iter_v = x;
    loop {
        // iter_v = iter_v - (iter_v * iter_v - x) / (2.0 * iter_v);
        iter_v = (iter_v + x / iter_v) / 2.0; //由上式化简得来
        if (iter_v - last).abs() <= f64::EPSILON {
            return iter_v;
        }
        last = iter_v;
    }
}

// exp(log(x) * 0.5)
pub fn sqrt3(x: f64) -> f64 {
    assert!(x >= 0.0);
    (x.ln() * 0.5f64).exp()
}
