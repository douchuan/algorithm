use std::cmp::Ordering;

#[derive(Copy, Clone)]
pub struct DirectedEdge {
    v: usize,
    w: usize,
    weight: f32,
}

impl DirectedEdge {
    pub fn new(v: usize, w: usize, weight: f32) -> Self {
        if weight.is_nan() {
            panic!("Weight is NaN");
        }
        Self { v, w, weight }
    }

    pub fn weight(&self) -> f32 {
        self.weight
    }

    pub fn from(&self) -> usize {
        self.v
    }

    pub fn to(&self) -> usize {
        self.w
    }
}

impl PartialOrd for DirectedEdge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl PartialEq for DirectedEdge {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl Ord for DirectedEdge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.partial_cmp(&other.weight).unwrap()
    }
}

impl Eq for DirectedEdge {}

impl ToString for DirectedEdge {
    fn to_string(&self) -> String {
        format!("{}->{} {:5.2}", self.v, self.w, self.weight)
    }
}

/*
impl Default for Edge {
    fn default() -> Self {
        Self {
            v: 0,
            w: 0,
            weight: 0.0,
        }
    }
}
*/
