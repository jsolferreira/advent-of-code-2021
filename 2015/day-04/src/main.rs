use std::fs;

fn main() {
    let input = parse_input();

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn parse_input() -> String {
    fs::read_to_string("2015/day-04/input.txt").unwrap()
}

fn part_one(input: &String) -> u32 {
    let mut i = 0;
    let trimmed_input = input.trim();

    loop {
        let st = format!("{}{}", trimmed_input, i);
        let hash = format!("{:x}", md5::compute(&st));

        if hash.starts_with("00000") {
            return i;
        }

        i += 1;
    }
}

fn part_two(input: &String) -> u32 {
    let mut i = 0;
    let trimmed_input = input.trim();

    loop {
        let st = format!("{}{}", trimmed_input, i);
        let hash = format!("{:x}", md5::compute(&st));

        if hash.starts_with("000000") {
            return i;
        }

        i += 1;
    }
}
