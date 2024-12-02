use std::cmp::min;
use std::fs;

fn main() {
    let input = parse_input();

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn parse_input() -> String {
    fs::read_to_string("2015/day-02/input.txt")
        .unwrap()
}

fn part_one(input: &String) -> i32 {
    input
        .split_whitespace()
        .map(|d| {
            let dimensions: Vec<i32> = d.split("x")
                .map(|d| d.parse::<i32>().unwrap())
                .collect();

            let l = dimensions[0];
            let w = dimensions[1];
            let h = dimensions[2];

            let s1 = l * w;
            let s2 = w * h;
            let s3 = h * l;

            let min = min(s1, min(s2, s3));

            2 * s1 + 2 * s2 + 2 * s3 + min
        })
        .sum()
}

fn part_two(input: &String) -> i32 {
    input
        .split_whitespace()
        .map(|d| {
            let dimensions: Vec<i32> = d.split("x")
                .map(|d| d.parse::<i32>().unwrap())
                .collect();

            let l = dimensions[0];
            let w = dimensions[1];
            let h = dimensions[2];

            let p1 = 2 * l + 2 * w;
            let p2 = 2 * w + 2 * h;
            let p3 = 2 * h + 2 * l;

            let min = min(p1, min(p2, p3));

            min + l * w * h
        })
        .sum()
}
