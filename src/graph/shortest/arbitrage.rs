//! Proposition Z.
//! The arbitrage problem is a negative-cycle-detection problem
//! in edge-weighted digraphs.
//!
//! Proof:
//! Replace each weight by its logarithm, negated. With this change,
//! computing path weights by multiplying edge weights in the original
//! problem corresponds to adding them in the transformed problem.
//! Specifically, any product w1w2 . . . wk corresponds to a sum
//! - ln(w1) - ln(w2) - . . . - ln(wk). The transformed edge weights
//! might be negative or positive, a path from v to w gives a way of
//! converting from currency v to currency w, and any negative cycle
//! is an arbitrage opportunity.

use crate::graph::shortest::{BellmanFordSP, DirectedEdge, EWDigraph};
use crate::graph::IEWDigraph;

/// The Arbitrage provides a client that finds an arbitrage
/// opportunity in a currency exchange table by constructing a
/// complete-digraph representation of the exchange table and then finding
/// a negative cycle in the digraph.
/// This implementation uses the Bellman-Ford algorithm to find a
/// negative cycle in the complete digraph.
pub struct Arbitrage<'a> {
    names: Vec<&'a str>,
    cycle: Option<Vec<DirectedEdge>>, // negative cycle (or null if no such cycle)
}

impl<'a> Arbitrage<'a> {
    pub fn has_opportunity(&self) -> bool {
        self.cycle.is_some()
    }

    pub fn opportunity_cycle(&self) -> Option<Vec<(&'a str, &'a str, f32)>> {
        self.cycle.as_ref().map(|cycle| {
            cycle
                .iter()
                .map(|v| {
                    let from = self.names[v.from()];
                    let to = self.names[v.to()];
                    (from, to, (-v.weight()).exp())
                })
                .collect()
        })
    }

    pub fn calc(&self, stake: f32) -> Option<f32> {
        self.cycle
            .as_ref()
            .map(|cycle| cycle.iter().fold(stake, |acc, e| acc * (-e.weight()).exp()))
    }
}

impl<'a> std::convert::TryFrom<&'a str> for Arbitrage<'a> {
    type Error = ();

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        use crate::graph::util::parser;

        let mut lines = s.lines();

        // line0: V
        let s = lines.next().ok_or(())?;
        let (_, nv) = parser::parse_num::<usize>(s).ok().ok_or(())?;

        let mut g: Box<dyn IEWDigraph> = Box::new(EWDigraph::new(nv));
        let mut names = Vec::with_capacity(nv);
        // line1...: rates
        for v in 0..nv {
            let s = lines.next().ok_or(())?;
            let (_, (name, rates)) = parser::parse_list_rates(s).ok().ok_or(())?;
            debug_assert_eq!(nv, rates.len());
            names.push(name);
            for (w, rate) in rates.iter().enumerate() {
                // ln() is natural logarithm (base e)
                g.add_edge(v, w, -rate.ln());
            }
        }

        let spt = BellmanFordSP::new(g.as_ref(), 0);
        let cycle = spt.negative_cycle().map(|c| c.cloned().collect());
        Ok(Self { names, cycle })
    }
}
