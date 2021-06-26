use crate::tree::binary::node::Node;
use std::ptr::NonNull;

pub struct Tree<T> {
    pub root: Option<NonNull<Node<T>>>,
}

impl<T> Tree<T> {
    pub fn height(&self) -> usize {
        height(self.root)
    }
}

impl<T> Default for Tree<T> {
    fn default() -> Self {
        Tree { root: None }
    }
}

impl<T> Drop for Tree<T> {
    fn drop(&mut self) {
        fn visitor<T>(p: Option<NonNull<Node<T>>>) {
            if let Some(p) = p {
                let p = unsafe { Box::from_raw(p.as_ptr()) };
                visitor(p.left);
                visitor(p.right);
            }
        }
        visitor(self.root);
    }
}

fn height<T>(node: Option<NonNull<Node<T>>>) -> usize {
    node.map_or(0, |node| unsafe {
        let lh = height(node.as_ref().left);
        let rh = height(node.as_ref().right);
        1 + std::cmp::max(lh, rh)
    })
}
