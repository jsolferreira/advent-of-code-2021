use std::fs;

#[derive(Debug)]
struct Segment {
    patterns: Vec<String>,
    output: Vec<String>,
}

fn main() {
    let input = parse_input();

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn parse_input() -> Vec<Segment> {
    fs::read_to_string("2021/day-08/input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.split(" | ").collect();

            let patterns: Vec<String> = split[0].split(" ").map(String::from).collect();
            let output: Vec<String> = split[1].split(" ").map(String::from).collect();

            Segment { patterns, output }
        })
        .collect()
}

fn part_one(segments: &Vec<Segment>) -> i32 {
    let mut count = 0;

    for segment in segments {
        for pattern in &segment.patterns {
            if pattern.len() == 2 || pattern.len() == 4 || pattern.len() == 3 || pattern.len() == 7
            {
                for o in &segment.output {
                    if pattern.len() == o.len() && pattern.chars().all(|c| o.contains(c)) {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn part_two(segments: &Vec<Segment>) -> i32 {
    todo!()
}
