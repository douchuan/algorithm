use algo::ll::{LinkedList, Node};

#[test]
fn len() {
    let mut ll = LinkedList::default();
    let data = vec![1, 2, 3, 4, 5];
    for v in &data {
        let node = Node::new(*v);
        ll.push(node);
    }

    assert_eq!(ll.len(), data.len());
}
