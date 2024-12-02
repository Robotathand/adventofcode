use std::fs;

fn main() {
    let mut column1: Vec<i32> = Vec::new();
    let mut column2: Vec<i32> = Vec::new();
    let mut total = 0;

    // Read file and split into columns an i32.
    let data = fs::read_to_string("../data.txt")
        .expect("Can't read");

    for line in data.lines() {
        let mut split_line = line.split_whitespace();
        column1.push(
            split_line.next().unwrap().parse::<i32>().unwrap(),
        );
        column2.push(
            split_line.next().unwrap().parse::<i32>().unwrap(),
        );
    }

    for item1 in column1 {
        let mut count: i32 = 0;
        for item2 in &column2 {
            if &item1 == item2 {
                count += 1;
            }
        }
        total += item1 * count;
    }

    println!("{total}")
}
