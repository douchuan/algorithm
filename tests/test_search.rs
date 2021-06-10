use algo::search;

#[test]
fn binary_search() {
    //empty array
    assert_eq!(search::binary::search(&[], 0), None);

    let xs = vec![1, 2, 3, 4, 5, 6, 7, 8];
    for (v, p) in vec![
        (1, Some(0)),
        (4, Some(3)),
        (8, Some(7)),
        (100, None),
        (-1, None),
    ] {
        assert_eq!(search::binary::search(&xs, v), p);
    }
}
