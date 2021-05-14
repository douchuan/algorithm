use rand::prelude::*;

pub static DATA_LEN: usize = 1000;

// vec![(test_data, expect), ..]
pub fn plan_data() -> Vec<(Vec<i32>, Vec<i32>)> {
    vec![
        //empty
        (vec![], vec![]),
        //only 1
        (vec![1], vec![1]),
        //sorted
        (
            vec![1, 2, 4, 8, 9, 9, 13, 17, 22],
            vec![1, 2, 4, 8, 9, 9, 13, 17, 22],
        ),
        //unsorted, expect asc
        (
            vec![9, 4, 13, 2, 22, 17, 8, 9, 1],
            vec![1, 2, 4, 8, 9, 9, 13, 17, 22],
        ),
        //unsorted, expect asc
        (
            vec![0, 1099, 1089, 1079, 1069, 1059, 1049],
            vec![0, 1049, 1059, 1069, 1079, 1089, 1099],
        ),
    ]
}

pub fn random_data(len: usize) -> Vec<i32> {
    let mut datas = Vec::with_capacity(len);
    for _ in 0..len {
        let v: i32 = random();
        datas.push(v);
    }
    datas
}

pub fn sorted_data_asc(len: usize) -> Vec<i32> {
    let mut data = Vec::with_capacity(len);
    for i in 0..len {
        data.push(i as i32);
    }
    data
}

pub fn sorted_data_desc(len: usize) -> Vec<i32> {
    let mut data = sorted_data_asc(len);
    data.reverse();
    data
}
