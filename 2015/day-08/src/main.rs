use std::fs;

fn main() {
    let input = parse_input();

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn parse_input() -> Vec<Vec<char>> {
    fs::read_to_string("2015/day-08/input.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn part_one(input: &Vec<Vec<char>>) -> usize {
    input
        .iter()
        .map(|l| {
            let mut tot_mem = 0;
            let mut escaping = false;
            let mut remaining_escapes = 0;

            for c in l {
                if *c == '\\' && !escaping {
                    escaping = true;
                    remaining_escapes = 1;
                } else if *c == 'x' && escaping {
                    remaining_escapes = 2;
                } else if escaping {
                    if remaining_escapes == 1 {
                        escaping = false;
                        tot_mem += 1;
                    } else {
                        remaining_escapes -= 1
                    }
                } else if *c != '"' {
                    tot_mem += 1;
                }
            }

            return l.len() - tot_mem;
        })
        .sum()
}

fn part_two(input: &Vec<Vec<char>>) -> usize {
    input
        .iter()
        .map(|l| {
            let tot_mem = l.iter().fold(0, |mut acc, c| {
                acc += if *c == '"' || *c == '\\' { 2 } else { 1 };
                return acc;
            });

            return 2 + tot_mem - l.len();
        })
        .sum()
}
