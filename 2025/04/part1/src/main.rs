use std::fs;

fn main() {
    let f = fs::read_to_string("../data.txt").unwrap();
    let grid = f
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    println!("{:?}", grid);
}
