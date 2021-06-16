use crate::tree::binary::builder::TreeBuilder;
use crate::tree::binary::node::Node;
use crate::tree::binary::tree::Tree;
use crate::tree::binary::NodeQuery;
use std::cmp::max;
use std::ptr::NonNull;

pub trait BuildTournamentTree<K> {
    fn build_tournament_tree(data: &[K]) -> Tree<K>;
    fn tournament_tree_pop(tree: &mut Tree<K>) -> Option<K>;
}

impl<K> BuildTournamentTree<K> for TreeBuilder
where
    K: Copy + std::cmp::Ord + Minimal,
{
    fn build_tournament_tree(data: &[K]) -> Tree<K> {
        do_build(data)
    }

    fn tournament_tree_pop(tree: &mut Tree<K>) -> Option<K> {
        pop(tree)
    }
}

pub trait Minimal {
    fn minimal() -> Self;
}

impl Minimal for i32 {
    fn minimal() -> Self {
        i32::MIN
    }
}

fn pop<T>(tree: &mut Tree<T>) -> Option<T>
where
    T: Copy + std::cmp::Ord + Minimal,
{
    let nq = NodeQuery::new(tree.root);
    match nq.get_element() {
        Some(element) if element != T::minimal() => {
            // 每次取出锦标赛树的根节点后，自顶向下将其替换为min
            let leaf = replace_max_by_min(nq.get().unwrap(), element);
            // 由叶子节点向上回溯，设置新的最大值
            setup_new_max(leaf);
            Some(element)
        }
        _ => None,
    }
}

// 返回叶子节点的序号
fn replace_max_by_min<T>(node: NonNull<Node<T>>, root_element: T) -> NonNull<Node<T>>
where
    T: Copy + std::cmp::Ord + Minimal,
{
    let mut nq = NodeQuery::new(Some(node));
    nq.set_element(T::minimal());

    while nq.is_branch() {
        let child = if nq.left_element() == Some(root_element) {
            nq.left()
        } else {
            nq.right()
        };

        nq.set(child.get());
        nq.set_element(T::minimal());
    }

    nq.get().unwrap()
}

fn setup_new_max<T>(node: NonNull<Node<T>>)
where
    T: Copy + std::cmp::Ord,
{
    let mut nq = NodeQuery::new_parent(Some(node));
    while nq.is_some() {
        let mut new_max = nq.get_element().unwrap();
        if let Some(v) = nq.left_element() {
            new_max = new_max.max(v);
        }
        if let Some(v) = nq.right_element() {
            new_max = new_max.max(v);
        }
        nq.set_element(new_max);
        nq = nq.parent();
    }
}

/// 构建锦标赛树, from bottom to top
/// a中不能包含T::minimal()这个特殊值，pop需要用到T::minimal()做临界值
fn do_build<T>(data: &[T]) -> Tree<T>
where
    T: Copy + std::cmp::Ord,
{
    let mut tree = Tree::default();

    //build leaf
    let mut nodes: Vec<NonNull<Node<T>>> = data.iter().map(|v| Node::from_element(*v)).collect();

    while nodes.len() > 1 {
        nodes = nodes
            .chunks(2)
            .map(|chunk| match *chunk {
                [t1, t2] => unsafe { branch(t1, t2) },
                [t] => t,
                _ => unreachable!(),
            })
            .collect();
    }

    tree.root = nodes.first().cloned();
    tree
}

/// 创建分支节点，取t1, t2较大者的value构造parent
unsafe fn branch<T>(mut n1: NonNull<Node<T>>, mut n2: NonNull<Node<T>>) -> NonNull<Node<T>>
where
    T: Copy + std::cmp::Ord,
{
    let v = max(n1.as_ref().element, n2.as_ref().element);
    let node = Node::new(v, Some(n1), Some(n2), None);
    n1.as_mut().parent = Some(node);
    n2.as_mut().parent = Some(node);
    node
}

#[test]
fn t_build_tree() {
    /*
                                            16
                        /                                       \
                     16                                           14
                 /         \                                /           \
            16                   13                    10                    14
         /     \               /     \               /    \                /     \
       7         16         8          13        10          9         12         14
     /  \       /  \       /  \       /  \      /   \      /  \       /  \       /  \
    7    6    15   16     8    4    13    3    5    10    9    1     12   2    11   14

    */
    let a = &[7, 6, 15, 16, 8, 4, 13, 3, 5, 10, 9, 1, 12, 2, 11, 14];
    let tree = do_build(a);
    let r = unsafe { crate::tree::binary::traverse::PreOrderVisitor::recursive(&tree) };
    assert_eq!(
        r,
        vec![
            16, 16, 16, 7, 7, 6, 16, 15, 16, 13, 8, 8, 4, 13, 13, 3, 14, 10, 10, 5, 10, 9, 9, 1,
            14, 12, 12, 2, 14, 11, 14
        ]
    );
}

#[test]
fn t_pop() {
    let mut a = vec![7, 6, 15, 16, 8, 4, 13, 3, 5, 10, 9, 1, 12, 2, 11, 14];
    let mut tree = do_build(&a);

    //make a desc sorted
    a.sort();
    a.reverse();

    for v in a {
        assert_eq!(pop(&mut tree), Some(v));
    }

    assert_eq!(pop(&mut tree), None);
}
