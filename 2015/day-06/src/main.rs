use std::fs;

use regex::Regex;

use crate::Command::{Toggle, TurnOff, TurnOn};

#[derive(Debug)]
enum Command {
    TurnOff,
    TurnOn,
    Toggle,
}

#[derive(Debug)]
struct Instruction {
    command: Command,
    from: (usize, usize),
    to: (usize, usize),
}

fn main() {
    let input = parse_input();

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn parse_input() -> Vec<Instruction> {
    let re = Regex::new(r"(turn off|toggle|turn on) (\d+),(\d+) through (\d+),(\d+)").unwrap();

    fs::read_to_string("2015/day-06/input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            let capture = re.captures(l).unwrap();

            let command = match &capture[1] {
                "turn off" => TurnOff,
                "turn on" => TurnOn,
                "toggle" => Toggle,
                _ => panic!("Unknown command"),
            };

            Instruction {
                command,
                from: (
                    capture[2].parse::<usize>().unwrap(),
                    capture[3].parse::<usize>().unwrap(),
                ),
                to: (
                    capture[4].parse::<usize>().unwrap(),
                    capture[5].parse::<usize>().unwrap(),
                ),
            }
        })
        .collect::<Vec<Instruction>>()
}

fn part_one(input: &Vec<Instruction>) -> u32 {
    let mut grid = [[false; 1000]; 1000];
    let mut lights_turned_on = 0;

    for instruction in input {
        for i in instruction.from.0..=instruction.to.0 {
            for j in instruction.from.1..=instruction.to.1 {
                match instruction.command {
                    TurnOff => {
                        if grid[i][j] {
                            lights_turned_on -= 1
                        }
                        grid[i][j] = false;
                    }
                    TurnOn => {
                        if !grid[i][j] {
                            lights_turned_on += 1
                        }
                        grid[i][j] = true;
                    }
                    Toggle => {
                        grid[i][j] = !grid[i][j];
                        if grid[i][j] {
                            lights_turned_on += 1;
                        } else {
                            lights_turned_on -= 1;
                        }
                    }
                }
            }
        }
    }

    lights_turned_on
}

fn part_two(input: &Vec<Instruction>) -> u32 {
    let mut grid = [[0; 1000]; 1000];
    let mut brightness = 0;

    for instruction in input {
        for i in instruction.from.0..=instruction.to.0 {
            for j in instruction.from.1..=instruction.to.1 {
                match instruction.command {
                    TurnOff => {
                        if grid[i][j] > 0 {
                            grid[i][j] -= 1;
                            brightness -= 1;
                        }
                    }
                    TurnOn => {
                        grid[i][j] += 1;
                        brightness += 1;
                    }
                    Toggle => {
                        grid[i][j] += 2;
                        brightness += 2;
                    }
                }
            }
        }
    }

    brightness
}
