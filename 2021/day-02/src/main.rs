use std::fs;

enum Direction {
    FORWARD,
    DOWN,
    UP,
}

struct Command {
    direction: Direction,
    units: u32,
}

fn main() {
    let input = parse_input();

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn parse_input() -> Vec<Command> {
    fs::read_to_string("2021/day-02/input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let mut split = line.split(" ");

            let direction_str = split.next().unwrap();
            let units = split.next().unwrap();

            let direction = match direction_str {
                "forward" => Direction::FORWARD,
                "down" => Direction::DOWN,
                "up" => Direction::UP,
                _ => panic!("Invalid direction"),
            };

            return Command {
                direction,
                units: units.parse().unwrap(),
            };
        })
        .collect()
}

fn part_one(input: &Vec<Command>) -> u32 {
    let mut position = 0;
    let mut depth = 0;

    for command in input {
        match command.direction {
            Direction::FORWARD => position += command.units,
            Direction::DOWN => depth += command.units,
            Direction::UP => depth -= command.units,
        };
    }

    position * depth
}

fn part_two(input: &Vec<Command>) -> u32 {
    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for command in input {
        match command.direction {
            Direction::FORWARD => {
                position += command.units;
                depth += aim * command.units
            }
            Direction::DOWN => aim += command.units,
            Direction::UP => aim -= command.units,
        };
    }

    position * depth
}
