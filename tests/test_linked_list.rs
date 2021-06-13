use algo::ll::{self, LinkedList, Node};

#[test]
fn normal() {
    let mut ll = LinkedList::default();
    let data = vec![1, 2, 3, 4, 5];
    for v in &data {
        let node = Node::new(*v);
        ll.push_back(node);
    }

    //len
    assert_eq!(ll.len(), data.len());

    //verify data
    let ll_data: Vec<i32> = ll.into_iter().collect();
    assert_eq!(ll_data, data);
}

#[test]
fn reverse() {
    let mut ll = LinkedList::default();
    let data = vec![1, 2, 3, 4, 5];
    for v in &data {
        let node = Node::new(*v);
        ll.push_back(node);
    }

    //reverse
    ll.head = ll::reverse::reverse(ll.head);
    let ll_data: Vec<i32> = ll.into_iter().collect();
    assert_eq!(ll_data, vec![5, 4, 3, 2, 1]);
}
