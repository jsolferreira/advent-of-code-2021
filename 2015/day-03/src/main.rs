use std::collections::HashSet;
use std::fs;

fn main() {
    let input = parse_input();

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn parse_input() -> String {
    fs::read_to_string("2015/day-03/input.txt")
        .unwrap()
}

fn part_one(input: &String) -> usize {
    let mut position = (0, 0);
    let mut positions_visited = HashSet::<(i32, i32)>::new();
    positions_visited.insert(position);

    for c in input.chars() {
        match c {
            '>' => position.0 += 1,
            '<' => position.0 -= 1,
            '^' => position.1 += 1,
            'v' => position.1 -= 1,
            _ => {}
        }

        if !positions_visited.contains(&position) {
            positions_visited.insert(position);
        }
    }

    positions_visited.len()
}

fn part_two(input: &String) -> usize {
    let mut santa_position = (0, 0);
    let mut robot_position = (0, 0);
    let mut positions_visited = HashSet::<(i32, i32)>::new();
    positions_visited.insert((0, 0));

    for (i, c) in input.char_indices() {
        let mut position = if i % 2 == 0 { &mut santa_position } else { &mut robot_position };

        match c {
            '>' => (*position).0 += 1,
            '<' => (*position).0 -= 1,
            '^' => (*position).1 += 1,
            'v' => (*position).1 -= 1,
            _ => {}
        }

        if !positions_visited.contains(&position) {
            positions_visited.insert(*position);
        }
    }

    positions_visited.len()
}
