use std::io::{self, BufRead};

fn main() {
    let mut entries: Vec<i32> = Vec::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let s = line.unwrap();
        let entry = s.trim().parse().expect("Expected a number");
        entries.push(entry);
    }

    entries.sort();

    // O(N^3), but I don't care
    for a in &entries {
        for b in &entries {
            for c in &entries {
                if a + b + c == 2020 {
                    println!("{}", a * b * c);
                }
            }
        }
    }
}
