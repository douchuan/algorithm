use algo::sort;

#[test]
fn bubble() {
    let test = |x: i32, y: i32| x > y;
    let data = sort::util::plan_data();
    for (t, expect) in data {
        let mut tt = t.clone();
        sort::bubble::sort(&mut tt, test);
        assert_eq!(tt, expect, "t = {:?}, expect = {:?}", t, expect);
    }
}

#[test]
fn insert() {
    let test = |x: i32, y: i32| x < y;
    let data = sort::util::plan_data();
    for (t, expect) in data {
        let mut tt = t.clone();
        sort::insert::sort(&mut tt, test);
        assert_eq!(tt, expect, "t = {:?}, expect = {:?}", t, expect);
    }
}

#[test]
fn merge_v1() {
    let test = |x: i32, y: i32| x < y;
    let data = sort::util::plan_data();
    for (t, expect) in data {
        let tt = sort::merge::v1::sort(&t, test);
        assert_eq!(tt, expect, "t = {:?}, expect = {:?}", t, expect);
    }
}

#[test]
fn merge_v2() {
    let test = |x: i32, y: i32| x < y;
    let data = sort::util::plan_data();
    for (t, expect) in data {
        let mut tt = t.clone();
        sort::merge::v2::sort(&mut tt, test);
        assert_eq!(tt, expect, "t = {:?}, expect = {:?}", t, expect);
    }
}

#[test]
fn merge_v3() {
    let test = |x: i32, y: i32| x < y;
    let data = sort::util::plan_data();
    for (t, expect) in data {
        let mut tt = t.clone();
        sort::merge::v3::sort(&mut tt, test);
        assert_eq!(tt, expect, "t = {:?}, expect = {:?}", t, expect);
    }
}

#[test]
fn quick() {
    let test = |x: i32, y: i32| x < y;
    let data = sort::util::plan_data();
    for (mut t, expect) in data {
        sort::quick::sort(&mut t, test);
        assert_eq!(t, expect);
    }
}

#[test]
fn shell() {
    let test = |x: i32, y: i32| x < y;
    let data = sort::util::plan_data();
    for (t, expect) in data {
        let mut tt = t.clone();
        sort::shell::sort(&mut tt, test);
        assert_eq!(tt, expect, "t = {:?}, expect = {:?}", t, expect);
    }
}
