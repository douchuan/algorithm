use algo::ll::{self, LinkedList};

#[test]
fn normal() {
    let data = vec![1, 2, 3, 4, 5];
    let ll = create_ll(&data);

    //len
    assert_eq!(ll.len(), data.len());

    //verify data
    let ll_data: Vec<i32> = ll.into_iter().collect();
    assert_eq!(ll_data, data);
}

#[test]
fn push_front() {
    let mut data = vec![1, 2, 3, 4, 5];
    let mut ll = LinkedList::default();
    for &v in &data {
        ll.push_front(v);
    }

    //len
    assert_eq!(ll.len(), data.len());

    //verify data
    let ll_data: Vec<i32> = ll.into_iter().collect();
    data.reverse();
    assert_eq!(ll_data, data);
}

#[test]
fn reverse() {
    let mut data = vec![1, 2, 3, 4, 5];
    let mut ll = create_ll(&data);

    //reverse
    data.reverse();
    ll.reverse();
    assert_eq!(ll.first(), data.first());
    assert_eq!(ll.last(), data.last());
    let ll_data: Vec<i32> = ll.into_iter().collect();
    assert_eq!(ll_data, data);
}

#[test]
fn find_kth2tail() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let ll = create_ll(&data);

    let p = ll::kth2tail::find(&ll, 2);
    assert_eq!(unsafe { p.unwrap().as_ref().element }, 8);

    let p = ll::kth2tail::find(&ll, 5);
    assert_eq!(unsafe { p.unwrap().as_ref().element }, 5);

    let p = ll::kth2tail::find(&ll, 10);
    assert_eq!(p, None);
}

#[test]
fn drop_clear() {
    static mut DROPS: i32 = 0;
    struct Elem;
    impl Drop for Elem {
        fn drop(&mut self) {
            unsafe {
                DROPS += 1;
            }
        }
    }

    let mut ll = LinkedList::default();
    ll.push_back(Elem);
    ll.push_front(Elem);
    ll.push_back(Elem);
    ll.push_front(Elem);
    drop(ll);

    assert_eq!(unsafe { DROPS }, 4);
}

#[test]
fn drop_clear_with_reverse() {
    static mut DROPS: i32 = 0;
    struct Elem;
    impl Drop for Elem {
        fn drop(&mut self) {
            unsafe {
                DROPS += 1;
            }
        }
    }

    let mut ll = LinkedList::default();
    ll.push_back(Elem);
    ll.push_front(Elem);
    ll.push_back(Elem);
    ll.push_front(Elem);
    ll.reverse(); // do reverse
    drop(ll);

    assert_eq!(unsafe { DROPS }, 4);
}

#[test]
fn drop_with_pop() {
    static mut DROPS: i32 = 0;
    struct Elem;
    impl Drop for Elem {
        fn drop(&mut self) {
            unsafe {
                DROPS += 1;
            }
        }
    }

    let mut ll = LinkedList::default();
    ll.push_back(Elem);
    ll.push_front(Elem);
    ll.push_back(Elem);
    ll.push_front(Elem);

    // do pop, and drop Elem
    drop(ll.pop_front());
    drop(ll.pop_front());
    assert_eq!(unsafe { DROPS }, 2);

    drop(ll);
    assert_eq!(unsafe { DROPS }, 4);
}

fn create_ll<T>(data: &[T]) -> LinkedList<T>
where
    T: Copy,
{
    let mut ll = LinkedList::default();
    for v in data {
        ll.push_back(*v);
    }
    ll
}
