use std::fs;
use std::str::Chars;

fn main() {
    let input = parse_input();

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn parse_input() -> String {
    fs::read_to_string("2023/day-01/input.txt")
        .unwrap()
}

fn part_one(input: &String) -> u32 {
    input.lines()
        .fold(0, |acc, s| {

            let c = s.chars();

            acc + find_first_digit(s.chars()) * 10 + find_first_digit(s.chars()
                .rev())
        })
}

fn find_first_digit(mut stream: impl Iterator<Item = char>) -> u32 {

    stream
        .find(|c| c.is_digit(10))
        .map(|c| c.to_digit(10))
        .flatten()
        .unwrap()
}

fn part_two(input: &String) -> usize {
    let mut floor = 1;

    for (i, c) in input.char_indices() {
        if c == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }

        if floor == -1 {
            return i;
        }
    }

    return 0;
}
