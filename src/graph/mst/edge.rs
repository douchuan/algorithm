use std::cmp::Ordering;

// Default, 用于MinPQ中的0号元素
#[derive(Default, Copy, Clone)]
pub struct Edge {
    v: usize,
    w: usize,
    weight: f32,
}

impl Edge {
    pub fn new(v: usize, w: usize, weight: f32) -> Self {
        if weight.is_nan() {
            panic!("Weight is NaN");
        }
        Self { v, w, weight }
    }

    pub fn weight(&self) -> f32 {
        self.weight
    }

    pub fn either(&self) -> usize {
        self.v
    }

    pub fn other(&self, v: usize) -> usize {
        if self.v == v {
            self.w
        } else if self.w == v {
            self.v
        } else {
            panic!("illegal vertex")
        }
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

// todo: eliminate, used by KruskalMST
impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.partial_cmp(&other.weight).unwrap()
    }
}

impl Eq for Edge {}

impl ToString for Edge {
    fn to_string(&self) -> String {
        format!("{}-{} {:.5}", self.v, self.w, self.weight)
    }
}
