use std::convert::TryFrom;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Entry {
    min: u32,
    max: u32,
    letter: char,
    password: String,
}

// fn check_entry(entry: &Entry) -> bool {
//     let mut count = 0;
//     for c in entry.password.chars() {
//         if c == entry.letter {
//             count += 1;
//         }
//     }
//     println!("{:?}", entry);
//     entry.min <= count && count <= entry.max
// }

fn check_entry_v2(entry: &Entry) -> bool {
    let mut count = 0;
    if entry.max > entry.password.len() as u32 {
        panic!();
    }
    for k in [entry.min, entry.max].iter() {
        if entry
            .password
            .chars()
            .nth(usize::try_from(k - 1).unwrap())
            .unwrap()
            == entry.letter
        {
            count += 1;
        }
    }
    count == 1
}

fn parse(s: &str) -> Entry {
    let parts: Vec<&str> = s.split(":").collect();
    if parts.len() != 2 {
        panic!("oops");
    }

    let left = parts[0];
    let right = parts[1];

    let range: Vec<&str> = left.split(" ").collect();
    if range.len() != 2 {
        panic!("oops");
    }

    let range_parts: Vec<&str> = range[0].split("-").collect();
    if range_parts.len() != 2 {
        panic!("oops");
    }

    Entry {
        min: range_parts[0].parse().expect("expected a number"),
        max: range_parts[1].parse().expect("expected a number"),
        letter: range[1].chars().nth(0).expect("expected a char"),
        password: String::from(right.trim()),
    }
}

fn main() {
    let mut total = 0;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let s = line.unwrap();
        let entry = parse(&s);
        println!("{:?}", entry);
        if check_entry_v2(&entry) {
            println!("ok");
            total += 1;
        }
    }
    println!("{}", total);
}
