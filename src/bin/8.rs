use std::io::{self, BufRead};
use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl Instruction {
    fn parse(line: &str) -> Instruction {
        let parts: Vec<&str> = line.split(" ").collect();
        let value: i32 = parts[1].parse().unwrap();
        match parts[0] {
            "nop" => Instruction::Nop(value),
            "acc" => Instruction::Acc(value),
            "jmp" => Instruction::Jmp(value),
            _ => panic!(),
        }
    }

    fn invert(&self) -> Instruction {
        match self {
            Instruction::Nop(val) => Instruction::Jmp(*val),
            Instruction::Jmp(val) => Instruction::Nop(*val),
            Instruction::Acc(val) => Instruction::Acc(*val),
        }
    }
}

#[derive(Debug)]
struct Code {
    instructions: Vec<Instruction>,
}

enum ExecuteResult {
    InifiniteLoop(i32),
    Normal(i32),
    Abnormal(i32),
}

impl Code {
    fn parse(input: &mut dyn BufRead) -> Code {
        let mut instructions = Vec::new();
        for line in input.lines() {
            let line = line.unwrap();
            instructions.push(Instruction::parse(&line));
        }
        Code { instructions }
    }

    fn get_instruction(&self, line: usize, patch: &Option<usize>) -> Instruction {
        let result = self.instructions[line];
        match patch {
            None => result,
            Some(patch_line) => if *patch_line == line { result.invert() } else { result }
        }
    }

    fn execute(&self, patch: Option<usize>) -> ExecuteResult {
        let mut val: i32 = 0;
        let mut line: i32 = 0;

        let mut seen_lines = HashSet::new();

        loop {
            if line == self.instructions.len() as i32 {
                return ExecuteResult::Normal(val);
            }
            if line < 0 || line > self.instructions.len() as i32 {
                return ExecuteResult::Abnormal(val);
            }

            seen_lines.insert(line);

            let instruction = self.get_instruction(line as usize, &patch);

            match instruction {
                Instruction::Nop(_) => {
                    line += 1;
                },
                Instruction::Acc(delta) => {
                    line += 1;
                    val += delta;
                },
                Instruction::Jmp(delta) => {
                    line += delta;
                },
            }

            if seen_lines.contains(&line) {
                return ExecuteResult::InifiniteLoop(val);
            }
        }
    }

    fn find_fix(&self) -> i32 {
        for line in 0..self.instructions.len() {
            let result = self.execute(Some(line));
            match result {
                ExecuteResult::Normal(val) => {
                    return val;
                },
                _ => continue,
            }
        }
        panic!("No solution");
    }
}

fn main() {
    let stdin = io::stdin();

    let code = Code::parse(&mut stdin.lock());
    let outcome = code.execute(None);

    if let ExecuteResult::InifiniteLoop(val) = outcome {
        println!("Part one: {}", val);
    } else {
        panic!("Expected infinite loop");
    }

    println!("Part two: {}", code.find_fix());
}
