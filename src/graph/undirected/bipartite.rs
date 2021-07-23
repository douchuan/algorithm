use crate::common::Stack;
use crate::graph::IGraph;

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
pub struct Bipartite {
    marked: Vec<bool>,
    color: Vec<bool>,
    edge_to: Vec<usize>,
    cycle: Option<Stack<usize>>,
    is_bipartite: bool,
}

impl Bipartite {
    pub fn new(g: &Box<dyn IGraph>) -> Self {
        let mut tc = Self {
            marked: vec![false; g.V()],
            color: vec![false; g.V()],
            edge_to: vec![0; g.V()],
            cycle: None,
            is_bipartite: true,
        };

        for s in 0..g.V() {
            if !tc.marked[s] {
                tc.dfs(g, s);
            }
        }

        tc
    }

    pub fn is_bipartite(&self) -> bool {
        self.is_bipartite
    }

    /// Returns the side of the bipartite that vertex v is on.
    pub fn color(&self, v: usize) -> bool {
        if !self.is_bipartite {
            panic!("graph is not bipartite");
        }
        self.color[v]
    }
}

impl Bipartite {
    fn dfs(&mut self, g: &Box<dyn IGraph>, v: usize) {
        self.marked[v] = true;
        for &w in g.adj(v) {
            // short circuit if odd-length cycle found
            if self.cycle.is_some() {
                return;
            }

            if !self.marked[w] {
                self.edge_to[w] = v;
                self.color[w] = !self.color[v];
                self.dfs(g, w);
            } else if self.color[w] == self.color[v] {
                self.is_bipartite = false;
                let mut cycle = Stack::new();
                cycle.push(w);
                let mut x = v;
                while x != w {
                    cycle.push(x);
                    x = self.edge_to[x];
                }
                cycle.push(w);
                self.cycle = Some(cycle);
            }
        }
    }
}
