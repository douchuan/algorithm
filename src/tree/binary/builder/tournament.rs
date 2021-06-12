use crate::tree::binary::builder::TreeBuilder;
use crate::tree::binary::node::Node;
use crate::tree::binary::tree::Tree;
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
    let element = tree.root.map(|root| unsafe { (*root.as_ptr()).element });
    match element {
        Some(element) if element != T::minimal() => {
            unsafe {
                //每次取出锦标赛树的根节点后，自顶向下将其替换为min
                let leaf = replace_max_by_min(tree.root.unwrap(), element);
                //由叶子节点向上回溯，设置新的最大值
                setup_new_max(leaf);
            }
            Some(element)
        }
        _ => None,
    }
}

// 返回叶子节点的序号
unsafe fn replace_max_by_min<T>(mut node: NonNull<Node<T>>, root_element: T) -> NonNull<Node<T>>
where
    T: Copy + std::cmp::Ord + Minimal,
{
    (*node.as_ptr()).element = T::minimal();

    while !Node::is_leaf(node) {
        node = match (*node.as_ptr()).left {
            Some(left) if (*left.as_ptr()).element == root_element => left,
            _ => (*node.as_ptr()).right.unwrap(),
        };

        (*node.as_ptr()).element = T::minimal();
    }

    node
}

unsafe fn setup_new_max<T>(mut node: NonNull<Node<T>>)
where
    T: Copy + std::cmp::Ord,
{
    loop {
        match (*node.as_ptr()).parent {
            Some(parent) => {
                let mut new_max = (*parent.as_ptr()).element;
                if let Some(l) = (*parent.as_ptr()).left {
                    new_max = new_max.max((*l.as_ptr()).element);
                }
                if let Some(r) = (*parent.as_ptr()).right {
                    new_max = new_max.max((*r.as_ptr()).element);
                }
                (*parent.as_ptr()).element = new_max;
                node = parent;
            }
            _ => break,
        }
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
            .map(|chunk| match chunk {
                &[t1, t2] => branch(t1, t2),
                &[t] => t,
                _ => unreachable!(),
            })
            .collect();
    }

    tree.root = nodes.get(0).cloned();
    tree
}

/// 创建分支节点，取t1, t2较大者的value构造parent
fn branch<T>(n1: NonNull<Node<T>>, n2: NonNull<Node<T>>) -> NonNull<Node<T>>
where
    T: Copy + std::cmp::Ord,
{
    let v = unsafe { max((*n1.as_ptr()).element, (*n2.as_ptr()).element) };
    let node = Node::new(v, Some(n1), Some(n2), None);
    unsafe {
        (*n1.as_ptr()).parent = Some(node);
        (*n2.as_ptr()).parent = Some(node);
    }
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
