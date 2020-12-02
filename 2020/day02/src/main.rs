#[macro_use]
extern crate lazy_static;

use regex::Regex;

fn main() {
    let input_pwds = include_str!("input.txt");
    let passwords = input_pwds.split('\n')
        .filter(|&x| x.len() > 0)
        .map(|x| parse_line(x))
        .collect::<Vec<_>>();

    let mut valid1 = passwords.clone();
    valid1.retain(|x| x.valid_1());

    let mut valid2 = passwords.clone();
    valid2.retain(|x| x.valid_2());
    println!("Valid1: {}", valid1.len());
    println!("Valid2: {}", valid2.len());
}

#[derive(Debug, Clone)]
struct Password {
    a: usize,
    b: usize,
    required: char,
    actual: String
}

impl Password {
    fn valid_1(&self) -> bool {
        let mut tmp = self.actual.clone();
        tmp.retain(|c| c == self.required);
        tmp.len() >= self.a && tmp.len() <= self.b
    }

    fn valid_2(&self) -> bool {
        let cs: Vec<_> = self.actual.chars().collect();
        let c1 = cs[self.a - 1];
        let c2 = cs[self.b - 1];
        c1 == self.required && c2 != self.required
            || c1 != self.required && c2 == self.required
    }
}

fn parse_line(line: &str) -> Password {
    let parts: Vec<_> = line.splitn(2, ": ").collect();
    lazy_static! {
        static ref RE: Regex = Regex::new("^([0-9]+)-([0-9]+) (.)$").unwrap();
    }

    let caps = RE.captures(parts[0]).unwrap();
    let a: usize = caps.get(1).and_then(|m| m.as_str().parse::<>().ok()).unwrap();
    let b: usize = caps.get(2).and_then(|m| m.as_str().parse::<>().ok()).unwrap();
    let text = caps.get(3).and_then(|m| m.as_str().chars().next()).unwrap();
    Password {
        a,
        b,
        required: text,
        actual: parts[1].to_string()
    }
}