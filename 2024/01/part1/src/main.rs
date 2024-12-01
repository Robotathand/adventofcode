use std::fs;

fn main() {
    let mut c1: Vec<i32> = Vec::new();
    let mut c2: Vec<i32> = Vec::new();
    // let mut t1 = 0;
    // let mut t2 = 0;
    let mut d = 0;

    let data = fs::read_to_string("../data.txt")
        .expect("Can't read");

    // println!("{data}");

    for l in data.lines() {
        let v1 = l[..1].parse().unwrap();
        let v2 = l[4..].parse().unwrap();
        c1.push(v1);
        c2.push(v2);
        println!("{v1}, {v2}");
    }

    /* for i in c1 {
        t1 += i;
    }
    for i in c2 {
        t2 += i;
    }

    println!("Total: {t1}, {t2}");

    let d = t2 - t1; */

    c1.sort();
    c2.sort();

    for i in &c1 {
        println!("C1: {i}");
    }

    for i in &c2 {
        println!("C2: {i}");
    }

    for i in 0..c1.len() {
        let q = c1[i];
        let w = c2[i];
        println!("L: {}, {}", q, w);
        if c1[i] > c2[i] {
            d += c1[i] - c2[1];
            let a = c1[i] - c2[1];
            println!("A: {a}");
        } else if c1[i] < c2[i] {
            d += c2[i] - c1 [i];
            let a = c2[i] - c1[1];
            println!("B: {a}");
        }
        println!("D: {d}\n");
    }

    println!("Distance: {d}");
}
