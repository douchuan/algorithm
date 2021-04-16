#![allow(unused)]
use crate::tree::{Tree, TreeNode};

pub struct PreorderIter<'a> {
    stack: Vec<&'a TreeNode>,
}

impl<'a> PreorderIter<'a> {
    pub fn new(root: Option<&'a TreeNode>) -> Self {
        if let Some(node) = root {
            PreorderIter { stack: vec![node] }
        } else {
            PreorderIter { stack: vec![] }
        }
    }
}

impl<'a> Iterator for PreorderIter<'a> {
    type Item = &'a TreeNode;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            if let Some(right) = &node.right {
                self.stack.push(&right)
            }

            if let Some(left) = &node.left {
                self.stack.push(&left)
            }

            return Some(node);
        }

        return None;
    }
}

/*
cannot borrow `*node` as mutable more than once at a time

pub struct PreorderIterMut<'a> {
    stack: Vec<&'a mut TreeNode>,
}

impl<'a> PreorderIterMut<'a> {
    pub fn new(root: Option<&'a mut TreeNode>) -> Self {
        if let Some(node) = root {
            PreorderIterMut { stack: vec![node] }
        } else {
            PreorderIterMut { stack: vec![] }
        }
    }
}

impl<'a> Iterator for PreorderIterMut<'a> {
    type Item = &'a mut TreeNode;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            if let Some(right) = &mut node.right {
                self.stack.push(right)
            }

            if let Some(left) = &mut node.left {
                self.stack.push(left)
            }

            return Some(node);
        }

        return None;
    }
}
*/

#[test]
fn basic() {
    let n3 = TreeNode::new(3, None, None);
    let n2 = TreeNode::new(2, Some(Box::new(n3)), None);
    let n1 = TreeNode::new(1, None, Some(Box::new(n2)));
    let tree = Tree::new(Some(n1));
    let iter = PreorderIter::new(tree.root.as_ref());
    let mut r = vec![];
    for it in iter {
        r.push(it.value);
    }
    assert_eq!(vec![1, 2, 3], r);
}
