use std::{env::consts, fs};

fn main() {
    let data = fs::read_to_string("../data.txt")
        .expect("Can't read file");

        let mut total = 0;

    let mut grid: [[char; 100]; 100];

    for (r, row) in data.lines().enumerate() {
        for (col, column) in row.chars().enumerate() {
            grid[col][r] = column;
        }
    }

    let x = grid[1][1];
    println!("{}", x);
}
