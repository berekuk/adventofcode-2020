use std::io::{self,BufRead};
use std::collections::HashMap;

type Joltage = u32;
type Diff = i32;

#[derive(Debug)]
struct Data {
    adapters: Vec<Joltage>,
}

fn compatible(input: Joltage, output: Joltage) -> bool {
    let diff = (output - input) as Diff;
    0 <= diff && diff <= 3
}

impl Data {
    fn parse(input: &mut dyn BufRead) -> Data {
        let mut adapters = Vec::new();
        for line in input.lines() {
            let line = line.unwrap();
            let joltage = line.parse::<Joltage>().unwrap();
            adapters.push(joltage);
        }
        adapters.sort();
        Data { adapters }
    }

    fn chain_all(&self) -> HashMap<Diff, u32> {
        let mut input: Joltage = 0;
        let mut chain: Vec<Joltage> = Vec::new();
        let mut diff_stats: HashMap<Diff, u32> = HashMap::new();

        for adapter in self.adapters.iter() {
            if !compatible(input, *adapter)  {
                panic!("Adapter {} doesn't support input {}", adapter, input);
            }
            chain.push(*adapter);
            let diff = (adapter - input) as Diff;
            if !diff_stats.contains_key(&diff) {
                diff_stats.insert(diff, 0);
            }
            diff_stats.insert(diff, diff_stats.get(&diff).unwrap() + 1);
            input = *adapter;
        }
        diff_stats.insert(3, diff_stats.get(&3).unwrap() + 1); // final jump to device
        println!("{:?}", diff_stats);
        diff_stats
    }

    fn chain_combinations(&self, input: Joltage, output: Joltage) -> u64 {
        if self.adapters.len() == 0 {
            return if compatible(input, output) { 1 } else { 0 };
        }
        let center: usize = (self.adapters.len() - 1) / 2;
        let left_adapters = &self.adapters[0..center];
        let right_adapters = &self.adapters[(center + 1)..self.adapters.len()];

        let left_combinations = Data { adapters: left_adapters.to_vec() }.chain_combinations(input, self.adapters[center]);
        let right_combinations = Data { adapters: right_adapters.to_vec() }.chain_combinations(self.adapters[center], output);

        let can_skip_center = {
            let left_of_center = if left_adapters.len() == 0 { input } else { left_adapters[left_adapters.len() - 1] };
            let right_of_center = if right_adapters.len() == 0 { output } else { right_adapters[0] };
            compatible(left_of_center, right_of_center)
        };
        let skip_center_combinations = if can_skip_center {
            let mut adapters = left_adapters.to_vec();
            adapters.extend(right_adapters.to_vec());
            Data { adapters }.chain_combinations(input, output)
        } else {
            0
        };
        left_combinations * right_combinations + skip_center_combinations
    }
}

fn main() {
    let stdin = io::stdin();
    let data = Data::parse(&mut stdin.lock());
    let diff_stats = data.chain_all();
    println!("Part one: {}", diff_stats.get(&1).unwrap() * diff_stats.get(&3).unwrap());

    println!("Part two: {}", data.chain_combinations(0, data.adapters[data.adapters.len() - 1] + 3));
}
