use std::fs;

fn main() {
    let data = fs::read_to_string("../data.txt")
        .expect("Can't read file");

    let mut total = 0;

    let mut rows = Vec::new();

    for lines in data.lines() {
        rows.push(lines);
    }

    for (row_num, row) in rows.iter().enumerate() {
        for (char_num, character) in row.chars().enumerate() {

            if char_num < row.len() - 3 {
                if &row[char_num..char_num + 4] == "XMAS" || &row[char_num..char_num + 4] == "SAMX" {
                    total += 1;
                }
            }

            if row_num < rows.len() - 3 {
                let mut up_down: Vec<String>= Vec::new();
                for down in 0..4 {
                    up_down.push(rows[down][char_num..char_num + 1].to_string());
                }
                let mut q = String::new();
                for i in up_down {
                    q.push_str(&i);
                }
                println!("{q}");
                
                let mut up_down_str: String = "";
                for ud in up_down {
                    up_down_str.push(ud);
                }
                if &up_down == Vec::from(["X", "M", "A", "S"]) || up_down == ["S", "A", "M", "X"] {
                    total += 1;
                }
            }
        }
    }
    

    println!("{total}");
}
