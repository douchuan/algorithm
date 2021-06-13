use algo::ll::{self, LinkedList, Node};

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
fn reverse() {
    let mut data = vec![1, 2, 3, 4, 5];
    let mut ll = create_ll(&data);

    //reverse
    data.reverse();
    ll::reverse::reverse(&mut ll);
    assert_eq!(ll.first(), data.first());
    assert_eq!(ll.last(), data.last());
    let ll_data: Vec<i32> = ll.into_iter().collect();
    assert_eq!(ll_data, data);
}

#[test]
fn tail2head_print() {
    let mut data = vec![1, 2, 3, 4, 5];
    let ll = create_ll(&data);

    data.reverse();
    let rev_ll = ll::tail2head::print(&ll);
    assert_eq!(rev_ll, data);
}

fn create_ll<T>(data: &[T]) -> LinkedList<T>
where
    T: Copy,
{
    let mut ll = LinkedList::default();
    for v in data {
        let node = Node::new(*v);
        ll.push_back(node);
    }

    ll
}
