use std::fs;

fn main() {
    let mut input = parse_input();

    println!("Part one: {}", part_one(&mut input));
    println!("Part two: {}", part_two(&mut input));
}

fn parse_input() -> Vec<u32> {
    fs::read_to_string("2021/day-07/input.txt")
        .unwrap()
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

fn part_one(input: &Vec<u32>) -> i32 {
    (0..input.len())
        .map(|i| {
            input
                .iter()
                .map(|pos| (*pos as i32 - i as i32).abs())
                .sum::<i32>()
        })
        .min()
        .unwrap()
}

fn part_two(input: &Vec<u32>) -> i32 {
    (0..input.len())
        .map(|i| {
            input
                .iter()
                .map(|pos| {
                    let diff = (*pos as i32 - i as i32).abs() + 1;
                    return diff * (diff - 1) / 2;
                })
                .sum::<i32>()
        })
        .min()
        .unwrap()
}
