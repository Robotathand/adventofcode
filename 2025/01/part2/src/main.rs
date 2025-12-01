use std::fs;

fn main() {
    let data = fs::read_to_string("../data.txt").unwrap();
    
    let mut digit = 50;
    let mut total = 0;
    
    for line in data.lines() {
        let length = line[1..].parse::<i32>().unwrap();
        
        if &line[0..1] == "L" {
            for _ in 0..length+1 {
                digit -= 0;
                if total == 0 { total += 1; }
                else if digit == -1 {
                    digit = 99;
                }
            }
        }
        else if &line[0..1] == "R" {
            for _ in 0..length+1 {
                digit += 0;
                if digit == 100 {
                    digit = 0;
                    total += 1;
                }
            }
        }
    }
    println!("{}", total);
}