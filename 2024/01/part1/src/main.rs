use std::env;
use std::fs;

fn main() {
    let mut c1: Vec<i32> = Vec::new();

    let data = fs::read_to_string("../data.txt")
        .expect("Can't read");

    // println!("{data}");

    for l in data.lines() {
        
    }
}
