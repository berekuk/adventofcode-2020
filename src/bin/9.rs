use std::io::{self,BufRead};
use std::env;

type Val = i64;

fn is_sum(slice: &[Val], sum: Val) -> bool {
    for i in 0..slice.len() {
        for j in 0..slice.len() {
            if i == j {
                continue;
            }
            if slice[i] + slice[j] == sum {
                return true;
            }
        }
    }
    false
}

struct Data {
    numbers: Vec<Val>,
    window: usize,
}

impl Data {
    fn parse(input: &mut dyn BufRead, window: usize) -> Data {
        let mut numbers = Vec::new();
        for line in input.lines() {
            let line = line.unwrap();
            let number = line.parse::<Val>().unwrap();
            numbers.push(number);
        }

        Data { numbers, window }
    }

    fn find_first_invalid_number(&self) -> Option<Val> {
        for i in 0..(self.numbers.len() - self.window - 1) {
            let slice = &self.numbers[i..(i + self.window)];
            let next = self.numbers[i + self.window];
            if !is_sum(slice, next) {
                return Some(next);
            }
        }
        None
    }

    fn find_contiguous_set(&self, expected_sum: Val) -> &[Val] {
        for start in 0..self.numbers.len() {
            for end in (start + 1)..self.numbers.len() {
                let set = &self.numbers[start..=end];
                let set_sum: Val = set.iter().sum();
                if set_sum == expected_sum {
                    return set;
                }
                if set_sum > expected_sum {
                    break;
                }
            }
        }
        panic!("set not found");
    }

    fn part_two(&self) -> Val {
        let set = self.find_contiguous_set(self.find_first_invalid_number().unwrap());
        println!("{:?}", set);
        set.iter().min().unwrap() + set.iter().max().unwrap()
    }
}

fn main() {
    let stdin = io::stdin();

    let window = env::args().nth(1).unwrap().parse::<usize>().unwrap();
    let data = Data::parse(&mut stdin.lock(), window);
    match data.find_first_invalid_number() {
        None => panic!("Invalid number not found"),
        Some(v) => println!("{}", v),
    }

    println!("{}", data.part_two());
}
