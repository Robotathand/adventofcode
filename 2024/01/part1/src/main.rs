use std::fs;

fn main() {
    let mut column1: Vec<i32> = Vec::new();
    let mut column2: Vec<i32> = Vec::new();
    let mut distance = 0;    

    let data = fs::read_to_string("../data.txt")
        .expect("Can't read");

    for l in data.lines() {
        let mut line = l.split_whitespace();
        column1.push(
            line.next().unwrap().parse::<i32>().unwrap(),
        );
        column2.push(
            line.next().unwrap().parse::<i32>().unwrap(),
        );
    }

    column1.sort();
    column2.sort();
    
    for i in 0..column1.len() {
        if column1[i] > column2[i] {
            distance += column1[i] - column2[i];
        } else if column1[i] < column2[i] {
            distance += column2[i] - column1 [i];
        }
    }

    println!("Distance: {distance}");
}
