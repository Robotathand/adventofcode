use std::fs;

struct Num {
    index: u32,
    number: u32,
}

fn main() {
    let f = fs::read_to_string("../data.txt").unwrap();
    let mut total = 0;
    for l in f.lines() {
        let mut max1 = Num {
            index: 0,
            number: 0,
        };
        let mut max2 = Num {
            index: 0,
            number: 0,
        };
        for (i, c) in l.chars().enumerate() {
            let i = i as u32;
            let current: u32 = c.to_digit(10).unwrap();
            if current > max1.number {
                (max2.index, max2.number) = (max1.index, max1.number);
                (max1.index, max1.number) = (i, current);
            }
        }
        if max1.index == l.len() as u32 - 1 {
            (max2.index, max2.number) = (max1.index, max1.number);
            (max1.index, max1.number) = (0, 0);
            for i in 0..l.len() - 1 {
                let current = l.chars().nth(i).unwrap().to_digit(10).unwrap();
                if current > max1.number {
                    (max1.index, max1.number) = (i as u32, current);
                }
            }
        }
        let add = (max1.number.to_string() + max2.number.to_string().as_str())
            .parse::<u32>()
            .unwrap();
        total += add;
    }
    println!("{}", total);
}
