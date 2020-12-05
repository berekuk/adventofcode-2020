use std::io::{self, BufRead};

fn load_seats(input: &mut dyn BufRead) -> Vec<u32> {
    let mut seats = Vec::new();
    for line in input.lines() {
        let mut seat: u32 = 0;
        for c in line.unwrap().chars() {
            if c == 'B' || c == 'R' {
                seat += 1;
            }
            seat <<= 1;
        }
        seat >>= 1;
        seats.push(seat);
    }
    seats
}

fn max_seat(seats: &Vec<u32>) -> u32 {
    *seats.iter().max().unwrap()
}

fn main() {
    let stdin = io::stdin();
    let mut seats = load_seats(&mut stdin.lock());
    
    println!("{}", max_seat(&seats));

    seats.sort();

    for (i, v) in seats.iter().enumerate() {
        if i == 0 || i == seats.len() - 1 {
            continue;
        }
        let prev = seats.get(i - 1).unwrap();
        if v - prev > 1 {
            println!("{}", v - 1);
        }
    }
}
