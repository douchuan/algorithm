use crate::tree::binary::builder::TreeBuilder;
use crate::tree::binary::node::Node;
use crate::tree::binary::tree::Tree;
use crate::tree::binary::NodeQuery;
use std::cmp::max;
use std::ptr::NonNull;

pub trait BuildTournamentTree<K, V> {
    fn build_tournament_tree(data: &[K]) -> Tree<K, V>;
    fn tournament_tree_pop(tree: &mut Tree<K, V>) -> Option<K>;
}

impl<K, V> BuildTournamentTree<K, V> for TreeBuilder
where
    K: Copy + std::cmp::Ord + Minimal,
{
    fn build_tournament_tree(data: &[K]) -> Tree<K, V> {
        do_build(data)
    }

    fn tournament_tree_pop(tree: &mut Tree<K, V>) -> Option<K> {
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

fn pop<K, V>(tree: &mut Tree<K, V>) -> Option<K>
where
    K: Copy + std::cmp::Ord + Minimal,
{
    let nq = NodeQuery::new(tree.root);
    match nq.get_key() {
        Some(key) if *key != K::minimal() => {
            let key = *key;
            // 每次取出锦标赛树的根节点后，自顶向下将其替换为min
            let leaf = replace_max_by_min(nq.node.unwrap(), key);
            // 由叶子节点向上回溯，设置新的最大值
            setup_new_max(leaf);
            Some(key)
        }
        _ => None,
    }
}

// 返回叶子节点的序号
fn replace_max_by_min<K, V>(node: NonNull<Node<K, V>>, root_key: K) -> NonNull<Node<K, V>>
where
    K: Copy + std::cmp::Ord + Minimal,
{
    let mut nq = NodeQuery::new(Some(node));
    nq.set_entry((K::minimal(), None));

    while nq.is_branch() {
        nq = if nq.left_key() == Some(&root_key) {
            nq.left()
        } else {
            nq.right()
        };

        nq.set_entry((K::minimal(), None));
    }

    nq.node.unwrap()
}

fn setup_new_max<K, V>(node: NonNull<Node<K, V>>)
where
    K: Copy + std::cmp::Ord,
{
    let mut nq = NodeQuery::new_parent(Some(node));
    while nq.is_some() {
        let mut new_max = *nq.get_key().unwrap();
        if let Some(v) = nq.left_key() {
            new_max = new_max.max(*v);
        }
        if let Some(v) = nq.right_key() {
            new_max = new_max.max(*v);
        }
        nq.set_entry((new_max, None));
        nq = nq.parent();
    }
}

/// 构建锦标赛树, from bottom to top
/// a中不能包含T::minimal()这个特殊值，pop需要用到T::minimal()做临界值
fn do_build<K, V>(data: &[K]) -> Tree<K, V>
where
    K: Copy + std::cmp::Ord,
{
    //build leaf
    let mut nodes: Vec<NonNull<Node<K, V>>> = data.iter().map(|v| Node::new_key(*v)).collect();
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

    let mut tree = Tree::default();
    tree.root = nodes.first().cloned();
    tree
}

/// 创建分支节点，取t1, t2较大者的value构造parent
unsafe fn branch<K, V>(
    mut n1: NonNull<Node<K, V>>,
    mut n2: NonNull<Node<K, V>>,
) -> NonNull<Node<K, V>>
where
    K: Copy + std::cmp::Ord,
{
    let v = max(n1.as_ref().key, n2.as_ref().key);
    let node = Node::new(v, None, Some(n1), Some(n2), None);
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

    let tree: Tree<i32, i32> = do_build(a);
    let r = unsafe { crate::tree::binary::traverse::PreOrderVisitor::recursive(&tree) };
    assert_eq!(
        r,
        vec![
            16, 16, 16, 7, 7, 6, 16, 15, 16, 13, 8, 8, 4, 13, 13, 3, 14, 10, 10, 5, 10, 9, 9, 1,
            14, 12, 12, 2, 14, 11, 14
        ]
    );

    let r = unsafe { crate::tree::binary::traverse::InOrderVisitor::recursive(&tree) };
    assert_eq!(
        r,
        vec![
            7, 7, 6, 16, 15, 16, 16, 16, 8, 8, 4, 13, 13, 13, 3, 16, 5, 10, 10, 10, 9, 9, 1, 14,
            12, 12, 2, 14, 11, 14, 14
        ]
    );
}

#[test]
fn t_pop() {
    let mut a = vec![7, 6, 15, 16, 8, 4, 13, 3, 5, 10, 9, 1, 12, 2, 11, 14];
    let mut tree: Tree<i32, i32> = do_build(&a);

    //make a desc sorted
    a.sort();
    a.reverse();

    for v in a {
        assert_eq!(pop(&mut tree), Some(v));
    }

    assert_eq!(pop(&mut tree), None);
}
