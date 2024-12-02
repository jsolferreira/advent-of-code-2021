use std::fs;

fn main() {
    let input = parse_input();

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn parse_input() -> String {
    fs::read_to_string("2015/day-01/input.txt")
        .unwrap()
}

fn part_one(input: &String) -> i32 {
    input.chars().fold(1, |acc, c| {
        if c == '(' {
            acc + 1
        } else {
            acc - 1
        }
    })
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
