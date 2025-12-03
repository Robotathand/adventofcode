use std::fs;

fn main() {
    let data = fs::read_to_string("../data.txt").unwrap();
    let nums: Vec<&str> = data.split(['-', ',']).collect();
    let mut total = 0;
    for i in 0..nums.len() / 2 {
        // println!("{} {}", nums[2 * i], nums[2 * i + 1]);
        /* if nums[2 * i].len() % 2 != 0 && nums[2 * i].len() == nums[2 * i + 1].len() {
            break;
        } */
        let begin: i64 = nums[2 * i].parse().unwrap();
        let end: i64 = nums[2 * i + 1].parse().unwrap();
        for num in begin..end + 1 {
            let num_string = num.to_string();
            let length = num_string.len();
            if length % 2 == 0
            /* && &num_string[..1] != "0" */
            {
                if num_string[..length / 2] == num_string[length / 2..] {
                    total += num;
                }
            }
        }
    }
    println!("{total}");
}
