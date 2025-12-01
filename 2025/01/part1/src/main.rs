use std::{fs};

fn main() {
    let data = fs::read_to_string("../data.txt").expect("Can't read");

    let mut digit = 50; // missed starting value
    let mut total = 0;

    for line in data.lines() {
        let length = line[1..].parse::<i32>().unwrap();
        if &line[0..1] == "L" {
            digit -= length;
        } else if &line[0..1] == "R" {
            digit += length;
        }
        while digit < 0 {
            digit += 100;
        }
        while digit > 99 {
            digit -= 100;
        }

        if digit == 0 {
            total += 1;
        }
        
        // println!("{} {} {} {}", line, length, digit, total)
    }
    println!("{}", total)
}
