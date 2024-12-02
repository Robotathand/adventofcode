use std::{collections::btree_map::Range, fs};

fn main() {
    let mut total = 0;

    let data = fs::read_to_string("../data.txt")
        .expect("Can't read");
    
    for full_line in data.lines() {
        let mut safe = true;
        let line: Vec<i32> = full_line.trim().split_whitespace().filter_map(|i| i.parse::<i32>().ok()).collect(); // https://www.reddit.com/r/learnrust/comments/rpchf3/comment/hq3n3pp/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
        let mut increasing = true;

        if line[0] > line[1] {
            increasing = false;
        }

        for level in 0..line.len() - 1 {
            if line[level] == line[level + 1] {
                safe = false;
                break;
            }
            if line[level] + 3 < line[level + 1] {
                safe = false;
                break;
            }
            if line[level] - 3 > line[level + 1] {
                safe = false;
                break;
            }
            if increasing == true {
                if line[level] > line[level + 1] {
                    safe = false;
                    break;
                }
            } else if increasing == false {
                if line[level] < line[level + 1] {
                    safe = false;
                    break;
                }
            }
        }
        if safe == true {
            total += 1;
        }
    }
    println!("{total}")
}
