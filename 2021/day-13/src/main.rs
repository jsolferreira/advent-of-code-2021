use regex::Regex;
use std::{collections::HashSet, fs};

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    UP,
    LEFT,
}

#[derive(Copy, Clone)]
struct Fold {
    direction: Direction,
    line: u32,
}

fn main() {
    let (input, mut folds) = parse_input();

    println!("Part one: {}", part_one(&input, &mut folds));
    println!("Part two:");
    part_two(&input, &mut folds);
}

fn parse_input() -> (Vec<(u32, u32)>, Vec<Fold>) {
    let split = fs::read_to_string("2021/day-13/input.txt")
        .unwrap()
        .split("\n\r")
        .map(|str| String::from(str))
        .collect::<Vec<String>>();

    let positions = split[0]
        .lines()
        .map(|line| {
            let mut split = line.split(",");

            (
                split.next().unwrap().parse::<u32>().unwrap(),
                split.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect::<Vec<(u32, u32)>>();

    let re = Regex::new(r".*(x|y)=(\d+)").unwrap();

    let folds = split[1]
        .trim()
        .lines()
        .map(|line| {
            let captures = re.captures(line).unwrap();

            let direction = match &captures[1] {
                "x" => Direction::LEFT,
                "y" => Direction::UP,
                _ => panic!("Invalid fold direction {}", &captures[1]),
            };

            let line = captures[2].parse().unwrap();

            Fold { direction, line }
        })
        .collect::<Vec<Fold>>();

    (positions, folds)
}

fn part_one(input: &Vec<(u32, u32)>, folds: &mut Vec<Fold>) -> usize {
    fold(&input, &mut vec![folds[0]], 0).len()
}

fn part_two(input: &Vec<(u32, u32)>, folds: &mut Vec<Fold>) -> () {
    let code = fold(&input, folds, 0);

    draw(&code);
}

fn fold(input: &Vec<(u32, u32)>, folds: &mut Vec<Fold>, fold_index: usize) -> Vec<(u32, u32)> {
    if fold_index < folds.len() {
        let mut st: HashSet<(u32, u32)> = HashSet::new();
        let f = folds[fold_index];

        for a in input {
            if f.direction == Direction::LEFT && a.0 > f.line {
                st.insert((f.line - (a.0 - f.line), a.1));
            } else if f.direction == Direction::UP && a.1 > f.line {
                st.insert((a.0, f.line - (a.1 - f.line)));
            } else {
                st.insert(a.clone());
            }
        }

        return fold(
            &st.into_iter().collect::<Vec<(u32, u32)>>(),
            folds,
            fold_index + 1,
        );
    }

    input.to_vec()
}

fn draw(input: &Vec<(u32, u32)>) -> () {
    for y in 0..7 {
        for x in 0..50 {
            if input.contains(&(x, y)) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!("")
    }
}
