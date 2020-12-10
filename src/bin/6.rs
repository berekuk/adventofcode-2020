use std::collections::HashSet;
use std::io::{self, BufRead};

struct Form {
    answers: Vec<bool>,
}

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

impl Form {
    fn from_line(line: &String) -> Form {
        let mut form_chars = HashSet::new();
        for c in line.chars() {
            form_chars.insert(c);
        }

        let mut answers: Vec<bool> = Vec::new();
        for c in ALPHABET.chars() {
            answers.push(form_chars.contains(&c));
        }

        Form { answers: answers }
    }
}

struct Group {
    forms: Vec<Form>,
}

impl Group {
    fn yes_count(&self) -> u32 {
        let mut count = 0;
        for pos in 0..ALPHABET.chars().count() {
            for form in self.forms.iter() {
                if form.answers[pos] {
                    count += 1;
                    break;
                }
            }
        }
        count
    }

    fn yes_count_v2(&self) -> u32 {
        let mut count = 0;
        'outer: for pos in 0..ALPHABET.chars().count() {
            for form in self.forms.iter() {
                if !form.answers[pos] {
                    continue 'outer;
                }
            }
            count += 1;
        }
        count
    }
}

fn parse_groups(input: &mut dyn BufRead) -> Vec<Group> {
    let mut forms: Vec<Form> = Vec::new();
    let mut groups: Vec<Group> = Vec::new();

    for maybe_line in input.lines() {
        let line = maybe_line.unwrap();
        if line.trim() == "" {
            groups.push(Group { forms: forms });
            forms = Vec::new();
        } else {
            forms.push(Form::from_line(&line));
        }
    }
    groups.push(Group { forms: forms });
    groups
}

fn main() {
    let stdin = io::stdin();
    let groups = parse_groups(&mut stdin.lock());

    let mut yes_total = 0;
    for group in groups.iter() {
        yes_total += group.yes_count();
    }
    println!("{}", yes_total);

    let mut yes_total_v2 = 0;
    for group in groups.iter() {
        yes_total_v2 += group.yes_count_v2();
    }
    println!("{}", yes_total_v2);
}
