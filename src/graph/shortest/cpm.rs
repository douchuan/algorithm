use crate::graph::shortest::{AcyclicLP, EWDigraph};
use crate::graph::util::parser;
use crate::graph::IEWDigraph;

/// Parallel job scheduling
/// Given a set of jobs of specified duration to be completed, with
/// precedence constraints that specify that certain jobs have to be
/// completed before certain other jobs are begun, how can we schedule
/// the jobs on identical processors (as many as needed) such that they
/// are all completed in the minimum amount of time while still respecting
/// the constraints?
///
/// We assume that we have sufficient processors to perform as many jobs as
/// possible, limited only by precedence constraints.
///
/// Every sequence of jobs, each constrained to follow the job just
/// preceding it in the sequence, represents a lower bound on the
/// length of the schedule. If we define the length of such a sequence
/// to be its earliest possible completion time (total of the durations
/// of its jobs), the longest sequence is known as a critical path
/// because any delay in the start- ing time of any job delays the best
/// achievable completion time of the entire project.
///
/// Critical path method for parallel precedence-constrained job scheduling

pub struct CPM {
    n: usize, // number of jobs
    lp: AcyclicLP,
}

impl CPM {
    /// number of jobs
    pub fn len(&self) -> usize {
        self.n
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    pub fn dist_to(&self, i: usize) -> f32 {
        self.lp.dist_to(i)
    }

    pub fn finish_time(&self) -> f32 {
        let sink = 2 * self.n + 1;
        self.lp.dist_to(sink)
    }
}

impl std::convert::TryFrom<&str> for CPM {
    type Error = ();

    /// Definition.
    /// The critical path method for parallel scheduling is to proceed
    /// as follows: Create an edge-weighted DAG with a source s, a sink
    /// t, and two vertices for each job (a start vertex and an end vertex).
    /// For each job, add an edge from its start vertex to its end vertex
    /// with weight equal to its duration. For each precedence constraint
    /// v->w, add a zero-weight edge from the end vertex corresponding tovs
    /// to the beginning vertex corresponding to w. Also add zero-weight
    /// edges from the source to each job’s start vertex and from each
    /// job’s end vertex to the sink. Now, schedule each job at the time
    /// given by the length of its longest path from the source.
    fn try_from(i: &str) -> Result<Self, Self::Error> {
        let mut lines = i.lines();

        // number of jobs
        let s = lines.next().ok_or(())?;
        let (_, n) = parser::parse_num::<usize>(s).ok().ok_or(())?;

        // source and sink
        // a source s, a sink t, and two vertices for each
        // job (a start vertex and an end vertex)
        let source = 2 * n;
        let sink = 2 * n + 1;

        // build network
        let mut g = EWDigraph::new(2 * n + 2);
        for i in 0..n {
            let s = lines.next().ok_or(())?;
            if let Ok((_, v)) = parser::parse_list_float(s) {
                let duration = v[0];
                // add zero-weight edges from the source to each job’s
                // start vertex and from each job’s end vertex to the sink
                g.add_edge(source, i, 0.0);
                g.add_edge(i + n, sink, 0.0);
                // For each job, add an edge from its start vertex to
                // its end vertex with weight equal to its duration
                g.add_edge(i, i + n, duration);

                let m = v[1] as usize;
                if m > 0 {
                    for v in &v[2..] {
                        let successor = *v as usize;
                        // For each precedence constraint v->w, add a zero-weight edge from the
                        // end vertex corresponding tovs to the begin- ning vertex corresponding to w.
                        g.add_edge(i + n, successor, 0.0);
                    }
                }
            }
        }

        let lp = AcyclicLP::new(&g, source).ok().ok_or(())?;
        Ok(CPM { n, lp })
    }
}
