use crate::tree::binary::node::Node;
use crate::tree::binary::tree::Tree;
use std::collections::{HashSet, LinkedList};
use std::ptr::NonNull;

/// Binary Tree Preorder Traversal
/// Given a binary tree, return the preorder traversal of its nodes’ values.
/// For example: Given binary tree {1, #, 2, 3},
///     1
///     \
///      2
///     /
///    3
/// return [1, 2, 3].
/// Note: Recursive solution is trivial, could you do it iteratively?
pub struct PreOrderVisitor;

/// Binary Tree Inorder Traversal
/// Given a binary tree, return the inorder traversal of its nodes’ values.
/// For example: Given binary tree {1, #, 2, 3},
///     1
///     \
///      2
///     /
///    3
/// return [1, 3, 2].
/// Note: Recursive solution is trivial, could you do it iteratively?
pub struct InOrderVisitor;

/// Binary Tree Postorder Traversal
/// Given a binary tree, return the postorder traversal of its nodes’ values.
/// For example: Given binary tree {1, #, 2, 3},
///     1
///     \
///      2
///     /
///    3
/// return [3, 2, 1].
/// Note: Recursive solution is trivial, could you do it iteratively?
pub struct PostOrderVisitor;

/// Binary Tree Level Order Traversal
/// Given a binary tree, return the level order traversal of its nodes’
/// values. (ie, from left to right, level by level).
/// For example: Given binary tree {3, 9, 20, #, #, 15, 7},
///     3
///    / \
///   9  20
///     / \
///    15  7
/// return its level order traversal as:
/// [
///     [3],
///     [9, 20],
///     [15, 7]
/// ]
pub struct LevelOrderVisitor;

/// Binary Tree Level Order Traversal 2
/// Given a binary tree, return the level order traversal of its nodes’
/// values. (ie, from left to right, level by level).
/// For example: Given binary tree {3,9,20,#,#,15,7},
///     3
///    / \
///   9  20
///     / \
///    15  7
/// return its bottom-up level order traversal as:
/// [
///     [15, 7]
///     [9, 20],
///     [3],
/// ]
pub struct LevelOrderVisitor2;

/// Zigzag Level Order Traversal
/// Given a binary tree, return the level order traversal of its nodes’
/// values. (ie, from left to right, level by level).
/// For example: Given binary tree {3,9,20,#,#,15,7},
///     3
///    / \
///   9  20
///     / \
///    15  7
/// return its zigzag level order traversal as:
/// [
///     [3],
///     [20, 9],
///     [15, 7]
/// ]
pub struct ZigzagOrderVisitor;

/// Two elements of a binary search tree (BST) are swapped by mistake.
/// Recover the tree without changing its structure.
/// Note: A solution using O(n) space is pretty straight forward.
/// Could you devise a constant space solution?
///
/// 示例 1：
/// 输入：root = [1,3,null,null,2]
/// 输出：[3,1,null,null,2]
/// 解释：3 不能是 1 左孩子，因为 3 > 1 。交换 1 和 3 使二叉搜索树有效。
///
/// 示例 2：
/// 输入：root = [3,1,4,null,null,2]
/// 输出：[2,1,4,null,null,3]
/// 解释：2 不能在 3 的右子树中，因为 2 < 3 。交换 2 和 3 使二叉搜索树有效。
pub struct RecoverBinarySearchTree;

/// Given two binary trees, write a function to check if they are equal or not.
/// Two binary trees are considered equal if they are structurally identical and
/// the nodes have the same value.
pub struct SameTree;

impl PreOrderVisitor {
    /// 时间复杂度 O(n), 空间复杂度 O(n)
    pub unsafe fn iterate<T>(tree: &Tree<T>) -> Vec<T>
    where
        T: Copy,
    {
        let mut results = vec![];
        let mut stack = vec![];
        //point current node
        let mut p = tree.root;
        while let Some(node) = p {
            results.push(node.as_ref().element); //visit result
            for pp in [node.as_ref().right, node.as_ref().left].iter().flatten() {
                stack.push(*pp);
            }

            p = stack.pop();
        }

        results
    }

    /// 时间复杂度 O(n), 空间复杂度 O(1)
    ///
    /// 大概思路：当一个node有left subtree时，需要遍历left subtree
    /// 的各节点，完成left subtree的遍历，需要回溯到node，这个回
    /// 溯指针记录在left_child.right中
    ///
    /// 点评：利用tree本身的node记录回溯指针（避免用栈记录回溯），
    /// 使得空间复杂度由 O(n) => O(1)
    pub unsafe fn morris<T>(tree: &mut Tree<T>) -> Vec<T>
    where
        T: Copy,
    {
        let mut results = vec![];
        let mut cur = tree.root;

        while let Some(node) = cur {
            match node.as_ref().left {
                Some(left) => {
                    let mut record = left;

                    //traverse right subtree, find前驱node
                    loop {
                        match record.as_ref().right {
                            Some(r) if r != node => record = r,
                            _ => break,
                        }
                    }

                    match record.as_ref().right {
                        Some(_r) => {
                            //已线索化
                            cur = node.as_ref().right;
                            record.as_mut().right = None;
                        }
                        None => {
                            results.push(node.as_ref().element);

                            //未线索化
                            record.as_mut().right = cur;
                            cur = Some(left);
                        }
                    }
                }
                None => {
                    results.push(node.as_ref().element);
                    //无left subtree, 直接跨到right subtree
                    cur = node.as_ref().right;
                }
            }
        }

        results
    }

    /// 时间复杂度 O(n), 空间复杂度 O(n)
    pub unsafe fn recursive<T>(tree: &Tree<T>) -> Vec<T>
    where
        T: Copy,
    {
        let mut results = vec![];
        unsafe fn visitor<T>(p: Option<NonNull<Node<T>>>, results: &mut Vec<T>)
        where
            T: Copy,
        {
            if let Some(node) = p {
                results.push(node.as_ref().element);
                visitor(node.as_ref().left, results);
                visitor(node.as_ref().right, results);
            }
        }
        visitor(tree.root, &mut results);
        results
    }
}

impl InOrderVisitor {
    pub unsafe fn iterate<T>(tree: &Tree<T>) -> Vec<T>
    where
        T: Copy,
    {
        let mut results = vec![];
        let mut stack = vec![];
        //point current node
        let mut p = tree.root;
        loop {
            match (p, stack.is_empty()) {
                (Some(node), _) => {
                    //switch to left child
                    stack.push(node);
                    p = node.as_ref().left;
                }
                (None, false) => {
                    //visit result & switch to right child
                    p = stack.pop();
                    let node = p.unwrap();
                    results.push(node.as_ref().element);
                    p = node.as_ref().right;
                }
                (None, true) => break,
            }
        }

        results
    }

    pub unsafe fn recursive<T>(tree: &Tree<T>) -> Vec<T>
    where
        T: Copy,
    {
        let mut results = vec![];
        unsafe fn visitor<T>(p: Option<NonNull<Node<T>>>, results: &mut Vec<T>)
        where
            T: Copy,
        {
            if let Some(node) = p {
                visitor(node.as_ref().left, results);
                results.push(node.as_ref().element); //visit result
                visitor(node.as_ref().right, results);
            }
        }
        visitor(tree.root, &mut results);
        results
    }
}

impl PostOrderVisitor {
    pub unsafe fn iterate<T>(tree: &Tree<T>) -> Vec<T>
    where
        T: Copy,
    {
        let mut results = vec![];
        let mut stack = vec![];
        let mut visited = HashSet::new();
        //point current node
        let mut p = tree.root;
        while let Some(node) = p {
            //switch to left child
            match node.as_ref().left {
                Some(left) if !visited.contains(&left) => {
                    stack.push(node);
                    p = Some(left);
                    continue;
                }
                _ => (),
            }

            //switch to right child
            match node.as_ref().right {
                Some(right) if !visited.contains(&right) => {
                    stack.push(node);
                    p = Some(right);
                    continue;
                }
                _ => (),
            }

            //visit & record node
            results.push(node.as_ref().element);
            visited.insert(node);
            p = stack.pop();
        }

        results
    }

    pub unsafe fn recursive<T>(tree: &Tree<T>) -> Vec<T>
    where
        T: Copy,
    {
        let mut results = vec![];
        unsafe fn visitor<T>(p: Option<NonNull<Node<T>>>, results: &mut Vec<T>)
        where
            T: Copy,
        {
            if let Some(node) = p {
                visitor(node.as_ref().left, results);
                visitor(node.as_ref().right, results);
                results.push(node.as_ref().element);
            }
        }
        visitor(tree.root, &mut results);
        results
    }
}

impl LevelOrderVisitor {
    pub unsafe fn iterate<T>(tree: &Tree<T>) -> Vec<Vec<T>>
    where
        T: Copy,
    {
        let mut results = vec![];
        if let Some(p) = tree.root {
            let mut nodes = LinkedList::new();
            let mut next_level_nodes = vec![];

            //root node enqueue
            nodes.push_back(p);
            results.push(vec![]);

            loop {
                match nodes.pop_front() {
                    Some(node) => {
                        results
                            .last_mut()
                            .expect("empty results container")
                            .push((*node.as_ptr()).element);
                        for child in [node.as_ref().left, node.as_ref().right].iter().flatten() {
                            next_level_nodes.push(*child);
                        }
                    }
                    None => {
                        if next_level_nodes.is_empty() {
                            break;
                        } else {
                            results.push(vec![]);
                            nodes.extend(next_level_nodes.iter());
                            next_level_nodes.clear();
                        }
                    }
                }
            }
        }

        results
    }

    pub unsafe fn recursive<T>(tree: &Tree<T>) -> Vec<Vec<T>>
    where
        T: Copy,
    {
        let mut results = vec![];
        unsafe fn visitor<T: Copy>(
            level_nodes: Vec<NonNull<Node<T>>>,
            results: &mut Vec<Vec<T>>,
            pos: usize,
        ) {
            if level_nodes.is_empty() {
                return;
            }

            results.push(vec![]);

            let mut next_level_nodes = vec![];
            for node in level_nodes {
                for child in [node.as_ref().left, node.as_ref().right].iter().flatten() {
                    next_level_nodes.push(*child);
                }
                results[pos].push(node.as_ref().element);
            }

            visitor(next_level_nodes, results, pos + 1);
        }
        if let Some(p) = tree.root {
            visitor(vec![p], &mut results, 0);
        }
        results
    }
}

impl LevelOrderVisitor2 {
    pub unsafe fn iterate<T>(tree: &Tree<T>) -> Vec<Vec<T>>
    where
        T: Copy,
    {
        let mut r = LevelOrderVisitor::iterate(tree);
        r.reverse();
        r
    }

    pub unsafe fn recursive<T>(tree: &Tree<T>) -> Vec<Vec<T>>
    where
        T: Copy,
    {
        let mut r = LevelOrderVisitor::recursive(tree);
        r.reverse();
        r
    }
}

impl ZigzagOrderVisitor {
    pub unsafe fn iterate<T>(tree: &Tree<T>) -> Vec<Vec<T>>
    where
        T: Copy,
    {
        let mut results = vec![];
        if let Some(p) = tree.root {
            let mut nodes = LinkedList::new();
            let mut next_level_nodes = vec![];
            let mut left_to_right = false;

            //root node enqueue
            nodes.push_back(p);
            results.push(vec![]);

            loop {
                match nodes.pop_front() {
                    Some(node) => {
                        results
                            .last_mut()
                            .expect("empty results container")
                            .push(node.as_ref().element);

                        let children = if left_to_right {
                            vec![node.as_ref().left, node.as_ref().right]
                        } else {
                            vec![node.as_ref().right, node.as_ref().left]
                        };

                        for child in children.into_iter().flatten() {
                            next_level_nodes.push(child);
                        }
                    }
                    None => {
                        if next_level_nodes.is_empty() {
                            break;
                        } else {
                            results.push(vec![]);
                            nodes.extend(next_level_nodes.iter());
                            next_level_nodes.clear();
                            left_to_right = !left_to_right;
                        }
                    }
                }
            }
        }

        results
    }

    pub unsafe fn recursive<T>(tree: &Tree<T>) -> Vec<Vec<T>>
    where
        T: Copy,
    {
        let mut results = vec![];
        unsafe fn visitor<T>(
            level_nodes: Vec<NonNull<Node<T>>>,
            results: &mut Vec<Vec<T>>,
            pos: usize,
            left_to_right: bool,
        ) where
            T: Copy,
        {
            if level_nodes.is_empty() {
                return;
            }

            results.push(vec![]);

            let mut next_level_nodes = vec![];
            for node in level_nodes {
                let children = if left_to_right {
                    vec![node.as_ref().left, node.as_ref().right]
                } else {
                    vec![node.as_ref().right, node.as_ref().left]
                };
                for child in children.into_iter().flatten() {
                    next_level_nodes.push(child);
                }
                results[pos].push(node.as_ref().element);
            }

            visitor(next_level_nodes, results, pos + 1, !left_to_right);
        }
        if let Some(p) = tree.root {
            visitor(vec![p], &mut results, 0, false);
        }
        results
    }
}
