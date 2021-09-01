//!
//! ## 图的典型应用
//!
//! | 应用       |   节点       ｜    连接         |
//! |-----------|--------------|-----------------|
//! | 地图       |   十字路口     |     公路      |
//! | 网络内容    |    网页       |     超链接      |
//! | 电路       |    元器件      |    导线        |
//! | 任务调度    |     任务      |     限制条件    |
//! | 商业交易    |     客户      |     交易        |
//! | 配对       |     学生      |     申请        |
//! | 计算机网络  |     网站      |     物理连接     |
//! | 软件       |     方法      |     调用关系     |
//! | 社交网络    |     人       |     友谊关系      |

pub mod directed;
pub mod mst; // minimum spanning trees
pub mod shortest; // shortest path
pub mod undirected;
pub mod util;

pub use mst::IEWGraph;
pub use shortest::IEWDigraph;
pub use undirected::IGraph;

macro_rules! impl_to_string {
    ($G: ty) => {
        impl ToString for $G {
            fn to_string(&self) -> String {
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
    };
}

impl_to_string!(dyn IGraph);
impl_to_string!(dyn IEWGraph);
impl_to_string!(dyn IEWDigraph);
