use std::cmp::Ordering;

/// The QuickUnionUF represents a union–find data type
/// (also known as the disjoint-sets data type).
/// It supports the classic union and find operations,
/// along with a count operation that returns the total number
/// of sets.
///
/// The union–find data type models a collection of sets containing
/// n elements, with each element in exactly one set.
/// The elements are named 0 through n–1.
/// Initially, there are n sets, with each element in its
/// own set. The canonical element of a set (also known as the root, identifier,
/// leader, or set representative) is one distinguished element in the set.
///
/// Here is a summary of the operations:
///
/// find(p) returns the canonical element of the set containing p. The find operation
/// returns the same value for two elements if and only if they are in the same set.
///
/// union(p, q) merges the set containing element p with the set containing
/// element q. That is, if p and q are in different sets, replace these two sets
/// with a new set that is the union of the two.
///
/// count() returns the number of sets.
///
/// The canonical element of a set can change only when the set
/// itself changes during a call to union it cannot
/// change during a call to either find or count.
///
/// This implementation uses quick union. The constructor takes O(n) time, where
/// n is the number of sites.
///
/// The union and find operations take O(n) time in the worst case.
/// The count operation takes O(1) time.
///
/// For alternative implementations of the same API, see
/// UF, QuickFindUF, and WeightedQuickUnionUF.
pub struct QuickFindUF {
    id: Vec<usize>, // id[i] = component identifier of i
    count: usize,   // number of components
}

/// In particular, suppose that we use quick-find for the dynamic connectivity
/// problem and wind up with a single component. This requires at least N - 1
/// calls to union(), and, consequently, at least (N + 3)(N - 1) ~ N^2 array
/// accesses —- we are led immediately to the hypothesis that dynamic connectivity
/// with quick-find can be a quadratic-time process.
///
/// Suppose that the input pairs come in the order 0-1, then 0-2, then 0-3, and
/// so forth.
///
/// After N - 1 such pairs, we have N sites all in the same set, and the tree
/// that is formed by the quick-union algorithm has height N - 1, with 0 linking
/// to 1, which links to 2, which links to 3, and so forth (see the diagram on
/// the facing page). By Proposition G, the number of array accesses for the
/// union() operation for the pair 0 i is exactly 2i+2 (site 0 is at depth i
/// and site i at depth 0). Thus, the total number of array accesses for the
/// find() operations for these N pairs is 2 (1 + 2 + . . . + N ) ~ N^2
pub struct QuickUnionUF {
    parent: Vec<usize>, // parent[i] = parent of i
    count: usize,       // number of components
}

/// Rather than arbitrarily connecting the second tree to the first for union(),
/// we keep track of the size of each tree and always connect the smaller tree to
/// the larger. It leads to substantial improvements in efficiency.
/// The weighted algorithm can guarantee logarithmic performance.
pub struct WeightedQuickUnionUF {
    parent: Vec<usize>,
    size: Vec<usize>,
    count: usize,
}

/// This implementation uses weighted quick union by rank
/// with path compression by halving.
pub struct UF {
    parent: Vec<usize>, // parent[i] = parent of i
    // rank[i] = rank of subtree rooted at i (never more than 31)
    // rank == 31的话，将有 2 ^ 31 个节点, 一棵满二叉树的节点数量
    rank: Vec<usize>,
    count: usize, // number of components
}

impl QuickFindUF {
    pub fn new(n: usize) -> Self {
        let id = (0..n).collect();
        Self { id, count: n }
    }

    /// Returns the canonical element of the set containing element p
    pub fn find(&self, p: usize) -> usize {
        self.id[p]
    }

    /// Merges the set containing element p with the
    /// the set containing element q
    pub fn union(&mut self, p: usize, q: usize) {
        let p_id = self.id[p];
        let q_id = self.id[q];
        // p and q are already in the same component
        if p_id == q_id {
            return;
        }

        // Rename p’s component to q’s name
        for i in 0..self.id.len() {
            if self.id[i] == p_id {
                self.id[i] = q_id;
            }
        }

        self.count -= 1;
    }
}

impl QuickUnionUF {
    pub fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        Self { parent, count: n }
    }

    /// Returns the canonical element of the set containing element p
    pub fn find(&self, mut p: usize) -> usize {
        while p != self.parent[p] {
            p = self.parent[p];
        }
        p
    }

    /// Merges the set containing element p with the
    /// the set containing element q
    pub fn union(&mut self, p: usize, q: usize) {
        let root_p = self.find(p);
        let root_q = self.find(q);
        // p and q are already in the same component
        if root_p == root_q {
            return;
        }

        self.parent[root_p] = root_q;
        self.count -= 1;
    }
}

impl WeightedQuickUnionUF {
    pub fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        Self {
            parent,
            size: vec![1; n],
            count: n,
        }
    }

    pub fn find(&self, mut p: usize) -> usize {
        while p != self.parent[p] {
            p = self.parent[p];
        }
        p
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let root_p = self.find(p);
        let root_q = self.find(q);
        // p and q are already in the same component
        if root_p == root_q {
            return;
        }

        if self.size[root_p] < self.size[root_q] {
            self.parent[root_p] = root_q;
            self.size[root_q] += self.size[root_p];
        } else {
            self.parent[root_q] = root_p;
            self.size[root_p] += self.size[root_q];
        }

        self.count -= 1;
    }
}

impl UF {
    pub fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        Self {
            parent,
            rank: vec![0; n],
            count: n,
        }
    }

    pub fn find(&mut self, mut p: usize) -> usize {
        while p != self.parent[p] {
            self.parent[p] = self.parent[self.parent[p]]; // path compression by halving
            p = self.parent[p];
        }
        p
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let root_p = self.find(p);
        let root_q = self.find(q);
        // p and q are already in the same component
        if root_p == root_q {
            return;
        }

        match self.rank[root_p].cmp(&self.rank[root_q]) {
            Ordering::Less => self.parent[root_p] = root_q,
            Ordering::Greater => self.parent[root_q] = root_p,
            Ordering::Equal => {
                self.parent[root_q] = root_p;
                self.rank[root_p] += 1;
            }
        }

        self.count -= 1;
    }
}

impl ToString for UF {
    fn to_string(&self) -> String {
        let mut buf = Vec::new();
        buf.push(format!("{} sets", self.count));
        for i in 0..self.parent.len() {
            buf.push(format!(
                "{}: parent = {}, rank = {}",
                i, self.parent[i], self.rank[i]
            ));
        }
        buf.join("\n")
    }
}

macro_rules! uf_util {
    ($UF: ty) => {
        impl $UF {
            /// Returns the number of sets
            pub fn count(&self) -> usize {
                self.count
            }

            /// Returns true if the two elements are in the same set
            pub fn connected(&mut self, p: usize, q: usize) -> bool {
                self.find(p) == self.find(q)
            }
        }

        /// test data: the file tinyUF.txt contains the 11 connections among 10 sites
        impl std::str::FromStr for $UF {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                use crate::graph::util::parser;
                let mut lines = s.lines();
                // line0: number of sites
                let s = lines.next().ok_or(())?;
                let (_, nv) = parser::parse_num(s).ok().ok_or(())?;

                let mut uf = <$UF>::new(nv);

                // line1...: connection
                for s in lines {
                    if let Ok((_, v)) = parser::parse_list_num(s) {
                        uf.union(v[0], v[1]);
                    }
                }

                Ok(uf)
            }
        }
    };
}

uf_util!(QuickFindUF);
uf_util!(QuickUnionUF);
uf_util!(WeightedQuickUnionUF);
uf_util!(UF);
