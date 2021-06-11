use crate::tree::binary::rb::Color;

pub struct TreeNode<K> {
    pub key: K,
    pub left: Option<usize>,
    pub right: Option<usize>,
    pub parent: Option<usize>,
    pub color: Option<Color>,
}

impl<K> TreeNode<K> {
    pub fn new(
        key: K,
        left: Option<usize>,
        right: Option<usize>,
        parent: Option<usize>,
        color: Option<Color>,
    ) -> Self {
        TreeNode {
            key,
            left,
            right,
            parent,
            color,
        }
    }

    pub fn new_leaf(k: K, parent: Option<usize>, color: Option<Color>) -> Self {
        Self::new(k, None, None, parent, color)
    }

    pub fn from_key(key: K) -> Self {
        Self {
            key,
            left: None,
            right: None,
            parent: None,
            color: None,
        }
    }

    /// 一个节点的左右子树都为空，称之为 叶子节点
    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    /// 分支节点
    pub fn is_branch(&self) -> bool {
        !self.is_leaf()
    }

    /// 直接子节点个数，不包括孙子...
    pub fn children_count(&self) -> usize {
        match (self.left, self.right) {
            (Some(_), Some(_)) => 2,
            (Some(_), None) | (None, Some(_)) => 1,
            (None, None) => 0,
        }
    }
}
