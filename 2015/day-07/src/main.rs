use regex::Regex;
use std::{collections::HashMap, fs};

#[derive(Debug, PartialEq)]
enum Operation {
    And,
    Or,
    RShift,
    LShift,
    Not,
    Assign,
}

#[derive(Debug)]
struct Instruction {
    operation: Operation,
    val1: String,
    val2: String,
    assignment: String,
}

impl Instruction {
    fn change_val1(&mut self, x: String) {
        self.val1 = x;
    }
    fn change_val2(&mut self, x: String) {
        self.val2 = x;
    }
    fn resolve(&mut self) {
        match self.operation {
            Operation::And => {
                if let (Ok(x), Ok(y)) = (self.val1.parse::<i32>(), self.val2.parse::<i32>()) {
                    self.operation = Operation::Assign;
                    self.val1 = (x & y).to_string();
                }
            }
            Operation::Or => {
                if let (Ok(x), Ok(y)) = (self.val1.parse::<i32>(), self.val2.parse::<i32>()) {
                    self.operation = Operation::Assign;
                    self.val1 = (x | y).to_string();
                }
            }
            Operation::RShift => {
                if let (Ok(x), Ok(y)) = (self.val1.parse::<i32>(), self.val2.parse::<i32>()) {
                    self.operation = Operation::Assign;
                    self.val1 = (x >> y).to_string();
                }
            }
            Operation::LShift => {
                if let (Ok(x), Ok(y)) = (self.val1.parse::<i32>(), self.val2.parse::<i32>()) {
                    self.operation = Operation::Assign;
                    self.val1 = (x << y).to_string();
                }
            }
            Operation::Not => {
                if let Ok(x) = self.val1.parse::<u16>() {
                    self.operation = Operation::Assign;
                    self.val1 = (!x).to_string();
                }
            }
            _ => {}
        }
    }
}

fn main() {
    let mut input_part1 = parse_input("2015/day-07/input_part1.txt");
    let mut input_part2 = parse_input("2015/day-07/input_part2.txt");

    println!("Part one: {}", resolve(&mut input_part1));
    println!("Part two: {}", resolve(&mut input_part2));
}

fn parse_input(path: &str) -> Vec<Instruction> {
    let and_regex = Regex::new(r"(\w+) AND (\w+) -> (\w+)").unwrap();
    let or_regex = Regex::new(r"(\w+) OR (\w+) -> (\w+)").unwrap();
    let rshift_regex = Regex::new(r"(\w+) RSHIFT (\w+) -> (\w+)").unwrap();
    let lshift_regex = Regex::new(r"(\w+) LSHIFT (\w+) -> (\w+)").unwrap();
    let not_regex = Regex::new(r"NOT (\w+) -> (\w+)").unwrap();
    let assign_regex = Regex::new(r"(\w+) -> (\w+)").unwrap();

    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            let is_and = and_regex.is_match(line);
            let is_or = or_regex.is_match(line);
            let is_rshift = rshift_regex.is_match(line);
            let is_lshift = lshift_regex.is_match(line);
            let is_not = not_regex.is_match(line);

            if is_and {
                let capture = and_regex.captures(line).unwrap();

                Instruction {
                    operation: Operation::And,
                    val1: capture[1].to_string(),
                    val2: capture[2].to_string(),
                    assignment: capture[3].to_string(),
                }
            } else if is_or {
                let capture = or_regex.captures(line).unwrap();

                Instruction {
                    operation: Operation::Or,
                    val1: capture[1].to_string(),
                    val2: capture[2].to_string(),
                    assignment: capture[3].to_string(),
                }
            } else if is_rshift {
                let capture = rshift_regex.captures(line).unwrap();

                Instruction {
                    operation: Operation::RShift,
                    val1: capture[1].to_string(),
                    val2: capture[2].to_string(),
                    assignment: capture[3].to_string(),
                }
            } else if is_lshift {
                let capture = lshift_regex.captures(line).unwrap();

                Instruction {
                    operation: Operation::LShift,
                    val1: capture[1].to_string(),
                    val2: capture[2].to_string(),
                    assignment: capture[3].to_string(),
                }
            } else if is_not {
                let capture = not_regex.captures(line).unwrap();

                Instruction {
                    operation: Operation::Not,
                    val1: capture[1].to_string(),
                    val2: String::from(""),
                    assignment: capture[2].to_string(),
                }
            } else {
                let capture = assign_regex.captures(line).unwrap();

                Instruction {
                    operation: Operation::Assign,
                    val1: capture[1].to_string(),
                    val2: String::from(""),
                    assignment: capture[2].to_string(),
                }
            }
        })
        .collect::<Vec<Instruction>>()
}

fn resolve(input: &mut Vec<Instruction>) -> u32 {
    let mut value_table = HashMap::<String, String>::new();

    while let Some(z) = input.iter().position(|instruction| {
        instruction.operation == Operation::Assign
            && match instruction.val1.trim().parse::<i32>() {
                Ok(_) => true,
                Err(_) => false,
            }
    }) {
        let a = input.swap_remove(z);

        value_table.insert(a.assignment.to_string(), a.val1.to_string());

        for i in input.iter_mut() {
            if i.val1 == a.assignment {
                i.change_val1(a.val1.to_string());
            }
            if i.val2 == a.assignment {
                i.change_val2(a.val1.to_string());
            }
            i.resolve();
        }
    }

    value_table.get("a").unwrap().parse::<u32>().unwrap()
}
