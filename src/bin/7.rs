#[macro_use] extern crate lazy_static;
use std::io::{self, BufRead};
use regex::Regex;
use std::collections::{HashMap, HashSet};

type Color = String;

#[derive(Debug)]
struct Rule {
    contents: HashMap<Color, u32>,
}

impl Rule {
    fn parse(line: &String) -> (Color, Rule) {
        lazy_static! {
            static ref LINE_RE: Regex = Regex::new(r"^(.+?) bags contains? (.+)$").unwrap();
            static ref BAGS_RE: Regex = Regex::new(r"^(\d+) (.+?) bags?\.?$").unwrap();
        }

        let caps = LINE_RE.captures(line).unwrap();

        let container = caps.get(1).unwrap().as_str().to_string();
        let contents_part = caps.get(2).unwrap().as_str().to_string();

        let mut contents = HashMap::new();

        if contents_part != "no other bags." {
            let bag_parts: Vec<&str> = contents_part.split(", ").collect();
            for bag_part in bag_parts {
                let bag_caps = BAGS_RE.captures(bag_part).expect(bag_part);
                let count = bag_caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
                let color = bag_caps.get(2).unwrap().as_str().to_string();
                contents.insert(color, count);
            }
        }

        (container, Rule { contents })
    }
}

#[derive(Debug)]
struct Ruleset {
    rules: HashMap<Color, Rule>,
}

impl Ruleset {
    fn parse(input: &mut dyn BufRead) -> Ruleset {
        let mut rules: HashMap<Color, Rule> = HashMap::new();
        for maybe_line in input.lines() {
            let line = maybe_line.unwrap();
            let (container, rule) = Rule::parse(&line);
            rules.insert(container, rule);
        }
        Ruleset { rules }
    }
    
    fn count_eventually_containers(&self, c: &Color) -> u32 {
        let mut found: HashSet<&Color> = HashSet::new();
        let mut found_count = 0;

        loop {
            for (container, rule) in self.rules.iter() {
                for child in rule.contents.keys() {
                    if child == c || found.contains(child) {
                        found.insert(&container);
                    }
                }
            }
            if found_count == found.len() {
                break;
            }
            found_count = found.len();
        }
        found_count as u32
    }

    fn count_bags(&self, c: &Color) -> u32 {
        let rule = self.rules.get(c).unwrap();
        let mut result = 1;
        for (child_color, count) in rule.contents.iter() {
            result += count * self.count_bags(child_color);
        }
        result
    }
}

fn main() {
    let stdin = io::stdin();
    let ruleset = Ruleset::parse(&mut stdin.lock());

    let c = String::from("shiny gold");
    println!("{:?}", ruleset.count_eventually_containers(&c));
    println!("{:?}", ruleset.count_bags(&c) - 1);
}
