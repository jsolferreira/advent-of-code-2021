use std::{collections::HashMap, fs};

fn main() {
    let input = parse_input();

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn parse_input() -> Vec<String> {
    fs::read_to_string("2021/day-10/input.txt")
        .unwrap()
        .lines()
        .map(|line| String::from(line))
        .collect()
}

fn part_one(input: &Vec<String>) -> u32 {
    let pair_map: HashMap<char, char> =
        HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let illegal_points: HashMap<char, u32> =
        HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);

    let mut total_illegal_points = 0;

    for line in input {
        let mut stack: Vec<char> = Vec::new();
        for c in line.chars() {
            if pair_map.contains_key(&c) {
                stack.push(*pair_map.get(&c).unwrap())
            } else {
                let f = match stack.pop() {
                    Some(pair) => pair == c,
                    None => false,
                };

                if !f {
                    total_illegal_points += *illegal_points.get(&c).unwrap();
                }
            }
        }
    }

    total_illegal_points
}

fn part_two(input: &Vec<String>) -> u64 {
    let pair_map: HashMap<char, char> =
        HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let illegal_points: HashMap<char, u64> =
        HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);

    let mut tot_scores: Vec<u64> = Vec::new();

    'outer: for line in input {
        let mut total_illegal_points = 0;
        let mut stack: Vec<char> = Vec::new();
        for c in line.chars() {
            if pair_map.contains_key(&c) {
                stack.push(*pair_map.get(&c).unwrap())
            } else {
                let f = match stack.pop() {
                    Some(pair) => c == pair,
                    None => false,
                };

                if !f {
                    continue 'outer;
                }
            }
        }

        for c in stack.iter().rev() {
            total_illegal_points = total_illegal_points * 5 + *illegal_points.get(&c).unwrap()
        }

        if total_illegal_points > 0 {
            tot_scores.push(total_illegal_points);
        }
    }

    tot_scores.sort();

    tot_scores[tot_scores.len() / 2]
}
