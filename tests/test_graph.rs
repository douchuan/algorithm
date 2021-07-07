use std::fs;

#[test]
fn read_res_file() {
    let s = include_str!("res/graph/tinyG.txt");
    println!("{}", s);
}