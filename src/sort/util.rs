use rand;
use rand::prelude::SliceRandom;

pub static DATA_LEN: usize = 1000;

pub fn vec_data() -> Vec<Vec<i32>> {
    vec![
        //empty
        vec![],
        //only 1
        vec![1],
        //sorted
        vec![1, 2, 4, 8, 9, 9, 13, 17, 22],
        // unsorted
        random_data(100),
    ]
}

pub fn random_data(len: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut data: Vec<i32> = (0..len as i32).collect();
    data.shuffle(&mut rng);
    data
}

pub fn sorted_data_asc(len: usize) -> Vec<i32> {
    (0..len as i32).collect()
}

pub fn sorted_data_desc(len: usize) -> Vec<i32> {
    let mut data = sorted_data_asc(len);
    data.reverse();
    data
}

pub fn eq_data(len: usize) -> Vec<i32> {
    vec![100; len]
}
