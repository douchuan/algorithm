use std::ptr::NonNull;

pub struct Node<T> {
    pub element: T,
    pub next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(element: T) -> NonNull<Self> {
        let v = Box::new(Node {
            element,
            next: None,
        });
        Box::leak(v).into()
    }

    pub fn release(node: NonNull<Node<T>>) {
        unsafe {
            let _ = Box::from_raw(node.as_ptr());
        }
    }
}
