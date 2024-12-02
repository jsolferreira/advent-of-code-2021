use std::fs;

fn main() {
    let input = parse_input();

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn parse_input() -> Vec<i32> {
    fs::read_to_string("2021/day-01/input.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}

fn part_one(input: &Vec<i32>) -> u32 {
    input[..].windows(2).fold(0, |acc, window| {
        if window[1] > window[0] {
            return acc + 1;
        } else {
            return acc;
        }
    })
}

fn part_two(input: &Vec<i32>) -> u32 {
    let mut last_sum = 0;

    input[..].windows(3).fold(0, |acc, window| {
        let sum = window[0] + window[1] + window[2];

        if sum > last_sum && last_sum != 0 {
            last_sum = sum;
            return acc + 1;
        } else {
            last_sum = sum;
            return acc;
        }
    })
}
