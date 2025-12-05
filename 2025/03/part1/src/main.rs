use std::fs;

struct Num {
    index: u32,
    number: u32,
}

fn main() {
    let file = fs::read_to_string("../data.txt").unwrap();
    let mut total = 0;
    for line in file.lines() {
        let mut max1 = Num {
            index: 0,
            number: 0,
        };
        let mut max2 = Num {
            index: 0,
            number: 0,
        };
        for (i, char) in line.chars().enumerate() {
            let index = i as u32;
            let current: u32 = char.to_digit(10).unwrap();
            if current > max1.number {
                (max2.index, max2.number) = (max1.index, max1.number);
                (max1.index, max1.number) = (index, current);
            }
        }
        if max1.index == line.len() as u32 - 1 {
            (max1.index, max1.number, max2.index, max2.number) =
                (max2.index, max2.number, max1.index, max1.number);
        } else {
            (max2.index, max2.number) = (0, 0);
            for i in max1.index + 1..line.len() as u32 {
                let current = line.chars().nth(i as usize).unwrap().to_digit(10).unwrap();
                if current > max2.number {
                    (max2.index, max2.number) = (i, current);
                }
            }
        }
        let add = (max1.number.to_string() + max2.number.to_string().as_str())
            .parse::<u32>()
            .unwrap();
        // println!("{add}");
        total += add;
    }
    println!("{}", total);
}
