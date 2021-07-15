use crate::graph::undirected::Graph;
use crate::graph::IGraph;

/// 检测环
/// Is a given graph acylic (无环图) ?
/// 给定的图是无环图吗 ?
pub struct Cycle {
    marked: Vec<bool>,
    has_cycle: bool,
}

/// 双色问题
/// Can the vertices of a given graph be assigned one of two colors
/// in such a way that no edge connects vertices of the same color?
/// which is equivalent to this question: Is the graph bipartite ?
/// 能够用两种颜色将图的所有顶点着色，使得任意一条边的两个端点的颜色都不相同吗？
/// 这个问题也等价于：这是一幅二分图吗？
///
/// bipartite example:
/// file movies.txt, from the Internet Movie Database (IMDB),
/// Recall that this file consists of lines listing a movie name followed
/// by a list of the performers in the movie. In the context of graph processing,
/// we can view it as defining a graph with movies and performers as vertices and
/// each line defining the adjacency list of edges connect- ing each movie to its
/// performers.
/// Note that the graph is a bipartite graph—there are no edges connecting
/// performers to performers or movies to movies.
pub struct TowColor {
    marked: Vec<bool>,
    color: Vec<bool>,
    is_two_colorable: bool,
}

impl Cycle {
    pub fn new(g: &Graph) -> Self {
        let mut cc = Self {
            marked: vec![false; g.V()],
            has_cycle: false,
        };

        for s in 0..g.V() {
            if !cc.marked[s] {
                cc.dfs(g, s, s);
            }
        }

        cc
    }

    pub fn has_cycle(&self) -> bool {
        self.has_cycle
    }
}

impl Cycle {
    fn dfs(&mut self, g: &Graph, v: usize, u: usize) {
        self.marked[v] = true;
        for &w in g.adj(v) {
            if !self.marked[w] {
                self.dfs(g, w, v);
            } else if w != u {
                self.has_cycle = true;
            }
        }
    }
}

impl TowColor {
    pub fn new(g: &Graph) -> Self {
        let mut c = Self {
            marked: vec![false; g.V()],
            color: vec![false; g.V()],
            is_two_colorable: true,
        };

        for s in 0..g.V() {
            if !c.marked[s] {
                c.dfs(g, s);
            }
        }

        c
    }

    pub fn is_bipartite(&self) -> bool {
        self.is_two_colorable
    }
}

impl TowColor {
    fn dfs(&mut self, g: &Graph, v: usize) {
        self.marked[v] = true;
        for &w in g.adj(v) {
            if !self.marked[w] {
                self.color[w] = !self.color[v];
                self.dfs(g, w);
            } else if self.color[w] == self.color[v] {
                self.is_two_colorable = false;
            }
        }
    }
}
