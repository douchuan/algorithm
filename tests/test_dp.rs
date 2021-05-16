use algo::dp;

#[test]
fn make_changes() {
    let coins = vec![1, 2, 5];
    //(min coins, amount)
    let solutions = vec![
        (3, 11),
        (3, 12),
        (4, 13),
        (4, 14),
        (3, 15),
        (4, 16),
        (4, 17),
        (5, 18),
        (5, 19),
        (4, 20),
    ];
    for (expect, amount) in solutions {
        assert_eq!(expect, dp::coin::make_change_classic(&coins, amount));
        assert_eq!(expect, dp::coin::make_change_cache(&coins, amount));
        assert_eq!(expect, dp::coin::make_change_iter(&coins, amount));
    }
}

#[test]
fn make_changes_fail() {
    let coins = vec![2, 5];
    let solutions = vec![(-1, 3)];
    for (expect, amount) in solutions {
        assert_eq!(expect, dp::coin::make_change_classic(&coins, amount));
        assert_eq!(expect, dp::coin::make_change_cache(&coins, amount));
        assert_eq!(expect, dp::coin::make_change_iter(&coins, amount));
    }
}

#[test]
fn fib_classic_recursive() {
    let values = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144];
    for (i, expect) in values.iter().enumerate() {
        assert_eq!(*expect, dp::fib::fib_classic_recursive(i));
    }
}

#[test]
fn fib_cache_result() {
    let values = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144];
    for (i, expect) in values.iter().enumerate() {
        assert_eq!(*expect, dp::fib::fib_cache_result(i));
    }
}

#[test]
fn fib_classic_iteration_loop() {
    let values = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144];
    for (i, expect) in values.iter().enumerate() {
        assert_eq!(*expect, dp::fib::fib_classic_iteration_loop(i));
    }
}

#[test]
fn fib_classic_iteration_for() {
    let values = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144];
    for (i, expect) in values.iter().enumerate() {
        assert_eq!(*expect, dp::fib::fib_classic_iteration_for(i));
    }
}

#[test]
fn fib_classic_recursive_c() {
    let values = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144];
    for (i, expect) in values.iter().enumerate() {
        assert_eq!(*expect, unsafe { dp::fib::fib_cache_result_c(i) });
    }
}

#[test]
fn fib_classic_iteration_for_c() {
    let values = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144];
    for (i, expect) in values.iter().enumerate() {
        assert_eq!(*expect, unsafe { dp::fib::fib_classic_iteration_for_c(i) });
    }
}
