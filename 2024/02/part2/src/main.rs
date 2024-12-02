use std::fs;

fn main() {
    let mut total = 0;

    let data = fs::read_to_string("../data.txt")
        .expect("Can't read");

    for line_str in data.lines() {
        let line: Vec<i32> = line_str.trim().split_whitespace().filter_map(|i| i.parse::<i32>().ok()).collect();
        let mut safe = false;

        if check(line.clone()) == true {
            safe = true;
        } else {
            for removed in 0..line.len() {
                let mut line_removed: Vec<i32> = Vec::new();

                for value in 0..line.len() {
                    if value != removed {
                        line_removed.push(line[value]);
                    }
                
                }

                if check(line_removed) == true {
                    safe = true;
                    break;
                }
            }
        }
        if safe == true {
            total += 1;
        }
    }
    println!("{total}");
}

fn check(line: Vec<i32>) -> bool {
    let mut safe_line = true;
    let mut increasing = true;

    if line[0] > line[1] {
        increasing = false;
    }
    for level in 0..line.len() - 1 {
        if line[level] == line[level + 1] {
            safe_line = false;
            break;
        }
        if line[level] + 3 < line[level + 1] {
            safe_line = false;
            break;
        }
        if line[level] - 3 > line[level + 1] {
            safe_line = false;
            break;
        }
        if increasing == true {
            if line[level] > line[level + 1] {
                safe_line = false;
                break;
            }
        } else if increasing == false {
            if line[level] < line[level + 1] {
                safe_line = false;
                break;
            }
        }
    }
    safe_line
}