#[macro_export]
macro_rules! graph_util {
    ($Graph: ty) => {
        use std::str::FromStr;

        impl ToString for $Graph {
            fn to_string(&self) -> String {
                self.stringify()
            }
        }

        impl FromStr for $Graph {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                use crate::graph::util::parser;
                let mut lines = s.lines();
                // line0: V
                let s = lines.next().ok_or(())?;
                let (_, nv) = parser::parse_num(s).ok().ok_or(())?;
                // line1: E
                let s = lines.next().ok_or(())?;
                let (_, ne) = parser::parse_num::<usize>(s).ok().ok_or(())?;

                let mut graph = <$Graph>::new(nv);
                // line2...: links
                for s in lines {
                    if let Ok((_, v)) = parser::parse_list_num(s) {
                        graph.add_edge(v[0], v[1]);
                    }
                }

                debug_assert_eq!(ne, graph.ne);

                Ok(graph)
            }
        }
    };
}
