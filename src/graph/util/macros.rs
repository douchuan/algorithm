#[macro_export]
macro_rules! graph_util {
    ($Graph: ty) => {
        impl ToString for $Graph {
            fn to_string(&self) -> String {
                self.stringify()
            }
        }

        impl std::str::FromStr for $Graph {
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

#[macro_export]
macro_rules! weighted_graph_util {
    ($Graph: ty) => {
        impl ToString for $Graph {
            fn to_string(&self) -> String {
                self.stringify()
            }
        }

        impl std::str::FromStr for $Graph {
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

                // line2...: weighted links
                for s in lines {
                    if let Ok((_, v)) = parser::parse_list_float(s) {
                        debug_assert_eq!(3, v.len());
                        let e = Edge::new(v[0] as usize, v[1] as usize, v[2]);
                        graph.add_edge(e);
                    }
                }

                debug_assert_eq!(ne, graph.ne);

                Ok(graph)
            }
        }
    };
}
