use std::fs;
use regex::Regex;

fn main() {
    let data = fs::read_to_string("../data.txt")
        .expect("Can't read");

    let pattern = Regex::new(r"mul\((\d?\d?\d),(\d?\d?\d)\)|do\(\)|don't\(\)").unwrap();
    let mut total = 0;
    let mut enable = true;

    for find in pattern.captures_iter(&data) {
        let find_string: String = find[0].parse()
            .expect("Can't make find into string");
        if enable == true && &find_string[..3] == "mul" {
            let one: i32 = find[1].parse()
                .expect("NAN");
            let two: i32 = find[2].parse()
                .expect("NAN");
            total += one * two;
        } else if find_string == "do()" {
            enable = true;
        } else if find_string == "don't()" {
            enable = false;
        }
    }

    println!("{total}");
}
