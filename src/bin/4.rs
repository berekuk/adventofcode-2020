use std::io::{self, BufRead};
use std::collections::HashMap;
use regex::Regex;


#[derive(Debug)]
struct PassportField {
    label: String,
    value: String,
}

impl PassportField {
    fn from_string(s: &str) -> PassportField {
        let parts: Vec<&str> = s.split(":").collect();
        PassportField { label: parts[0].to_string(), value: parts[1].to_string() }
    }
}

#[derive(Debug)]
struct Passport {
    fields: HashMap<String, String>,
}

impl Passport {
    fn from_lines(lines: Vec<String>) -> Passport {
        let mut fields = HashMap::new();
        for line in lines.iter() {
            for part in line.split(" ") {
                let field = PassportField::from_string(part);
                fields.insert(field.label, field.value);
            }
        }
        Passport { fields }
    }
    fn is_valid(&self) -> bool {
        match self.fields.get("byr") {
            None => return false,
            Some(v) => {
                match v.parse::<i32>() {
                    Err(_) => return false,
                    Ok(year) => {
                        if year < 1920 || year > 2002 {
                            return false;
                        }
                    }
                }
            }
        }

        match self.fields.get("iyr") {
            None => return false,
            Some(v) => {
                match v.parse::<i32>() {
                    Err(_) => return false,
                    Ok(year) => {
                        if year < 2010 || year > 2020 {
                            return false;
                        }
                    }
                }
            }
        }

        match self.fields.get("eyr") {
            None => return false,
            Some(v) => {
                match v.parse::<i32>() {
                    Err(_) => return false,
                    Ok(year) => {
                        if year < 2020 || year > 2030 {
                            return false;
                        }
                    }
                }
            }
        }

        match self.fields.get("hgt") {
            None => return false,
            Some(v) => {
                let unit = v.get((v.len() - 2)..v.len()).unwrap();
                let n = v.get(0..v.len() - 2).unwrap().parse::<i32>();
                if let Err(_) = n {
                    return false;
                }
                let n = n.unwrap();
                match unit {
                    "cm" => {
                        if n < 150 || n > 193 {
                            return false;
                        }
                    }
                    "in" => {
                        if n < 59 || n > 76 {
                            return false;
                        }
                    }
                    _ => panic!()
                }
            }
        }

        match self.fields.get("hcl") {
            None => return false,
            Some(v) => {
                let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
                if !re.is_match(v) {
                    return false;
                }
            }
        }

        match self.fields.get("ecl") {
            None => return false,
            Some(v) => {
                let re = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
                if !re.is_match(v) {
                    return false;
                }
            }
        }

        match self.fields.get("pid") {
            None => return false,
            Some(v) => {
                let re = Regex::new(r"^[0-9]{9}$").unwrap();
                if !re.is_match(v) {
                    return false;
                }
            }
        }

        true
    }
}

fn parse_input(input: &mut dyn BufRead) -> Vec<Passport> {
    let mut entry_lines: Vec<String> = Vec::new();
    let mut result: Vec<Passport> = Vec::new();

    for maybe_line in input.lines() {
        let line = maybe_line.unwrap();
        if line.trim() == "" {
            result.push(Passport::from_lines(entry_lines));
            entry_lines = Vec::new();
            continue;
        }
        entry_lines.push(line);
    }
    if entry_lines.len() > 0 {
        result.push(Passport::from_lines(entry_lines));
    }
    result
}

fn main() {
    let stdin = io::stdin();

    let passports = parse_input(&mut stdin.lock());

    let mut total = 0;
    for passport in passports.iter() {
        if passport.is_valid() {
            total += 1;
        }
    }
    println!("{}", total);
}
