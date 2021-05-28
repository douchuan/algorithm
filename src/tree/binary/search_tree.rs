use crate::tree::binary::{Tree, TreeIndex};

pub trait SearchTree<K>
where
    K: std::cmp::PartialOrd,
{
    fn insert(&mut self, k: K) -> bool;
    fn lookup(&self, x: K) -> Option<TreeIndex>;
    fn min(&self) -> Option<TreeIndex>;
    fn max(&self) -> Option<TreeIndex>;
    /// 查找后继元素
    fn succ(&self, x: K) -> Option<TreeIndex>;
    /// 寻找前驱元素
    fn pred(&self, x: K) -> Option<TreeIndex>;
}

impl<K> SearchTree<K> for Tree<K>
where
    K: std::cmp::PartialOrd,
{
    fn insert(&mut self, k: K) -> bool {
        todo!()
    }

    fn lookup(&self, x: K) -> Option<usize> {
        todo!()
    }

    fn min(&self) -> Option<usize> {
        todo!()
    }

    fn max(&self) -> Option<usize> {
        todo!()
    }

    fn succ(&self, x: K) -> Option<usize> {
        todo!()
    }

    fn pred(&self, x: K) -> Option<usize> {
        todo!()
    }
}
