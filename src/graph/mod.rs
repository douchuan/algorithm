//!
//! ## 图的典型应用
//!
//! | 应用       |   节点       ｜    连接
//! |-----------|--------------|--------------
//! | 地图       |   十字路口     |     公路
//! | 网络内容    |    网页       |     超链接
//! | 电路       |    元器件      |    导线
//! | 任务调度    |     任务      |     限制条件
//! | 商业交易    |     客户      |     交易
//! | 配对       |     学生      |     申请
//! | 计算机网络  |     网站      |     物理连接
//! | 软件       |     方法      |     调用关系
//! | 社交网络    |     人       |     友谊关系

#[macro_use]
pub mod util;
use crate::graph::mst::Edge;
use crate::ll::linked_list::Iter;

pub mod directed;
pub mod mst; // minimum spanning trees
pub mod undirected;

pub trait IGraph {
    /// number of vertices
    #[allow(non_snake_case)]
    fn V(&self) -> usize;

    /// number of edges
    #[allow(non_snake_case)]
    fn E(&self) -> usize;

    /// add edge v-w to this graph
    fn add_edge(&mut self, v: usize, w: usize);

    /// vertices adjacent to v
    fn adj(&self, v: usize) -> Iter<'_, usize>;

    /// directed graph op
    fn reverse(&self) -> Box<dyn IGraph> {
        panic!("No Support");
    }

    fn stringify(&self) -> String {
        let mut buf = Vec::new();
        buf.push(format!("{} vertices, {} edges", self.V(), self.E()));
        for v in 0..self.V() {
            let adj = self
                .adj(v)
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(" ");
            buf.push(format!("{}: {}", v, adj));
        }
        buf.join("\n")
    }
}

/// Edge weighted graph
pub trait IEWGraph {
    /// number of vertices
    #[allow(non_snake_case)]
    fn V(&self) -> usize;

    /// number of edges
    #[allow(non_snake_case)]
    fn E(&self) -> usize;

    /// Adds the undirected edge e to this edge-weighted graph
    fn add_edge(&mut self, e: Edge);

    /// Returns the edges incident on vertex v
    fn adj(&self, v: usize) -> Iter<'_, Edge>;

    /// Returns all edges in this edge-weighted graph
    fn edges(&self) -> Vec<Edge>;

    /// Returns the degree of vertex v
    fn degree(&self, v: usize) -> usize;

    fn stringify(&self) -> String {
        let mut buf = Vec::new();
        buf.push(format!("{} {}", self.V(), self.E()));
        for v in 0..self.V() {
            let adj = self
                .adj(v)
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join("  ");
            buf.push(format!("{}: {}", v, adj));
        }
        buf.join("\n")
    }
}
