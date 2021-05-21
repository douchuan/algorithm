//! 最小可用ID
//!
//! 这道题目来自 Richard Bird 书中的第一章 [1]。
//! 现代社会中，有很多服务依赖一种被称为ID的概念。
//! 例如身份证就是一种ID，银行账户也是一种ID，电
//! 话号码本质上也是一种ID。假设我们使用非负整数作
//! 为某个系统的的ID，所有用户都由一个ID唯一确定。
//! 任何时间，这个系统中有些ID处在使用中的状态，有些
//! ID则可以用于分配给新用户。
//! 现在的问题是，怎样才能找到最小的可分配ID呢?
//!
//! 例如下面的列表记录了当前正在被使用的ID:
//! [18, 4, 8, 9, 16, 1, 14, 7, 19, 3, 0, 5, 2, 11, 6]
//!
//! [1] Richard Bird. “Pearls of functional algorithm design”. Cambridge University Press; 1 edition (November 1, 2010). ISBN-10: 0521513383

//todo: improve me
/// 时间复杂度 O(n^2)
pub fn mfv1(a: &[usize]) -> usize {
    let mut v = 0;
    loop {
        if a.contains(&v) {
            v += 1;
        } else {
            return v;
        }
    }
}
