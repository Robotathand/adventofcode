use std::fs;

fn main() {
    let f = fs::read_to_string("../data.txt").unwrap();
    let grid = f
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| if c == '@' { 1 } else { 0 })
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    let mut access = vec![vec![0; grid.len()]; grid[0].len()];
}
