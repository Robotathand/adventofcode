use std::fs;
use regex::Regex;

fn main() {
    let data = fs::read_to_string("../data.txt")
        .expect("Can't read");

    let pattern = Regex::new(r"mul\((\d?\d?\d),(\d?\d?\d)\)").unwrap();
    let mut total = 0;

    for mul in pattern.captures_iter(&data) {
        let one: i32 = mul[1].parse()
            .expect("NAN");
        let two: i32 = mul[2].parse()
            .expect("NAN");
        total += one * two;
    }

    println!("{total}")
}
