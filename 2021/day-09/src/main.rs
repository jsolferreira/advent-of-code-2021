use std::{collections::HashSet, fs};

fn main() {
    let input = parse_input();

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn parse_input() -> Vec<Vec<u32>> {
    fs::read_to_string("2021/day-09/input.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn part_one(input: &Vec<Vec<u32>>) -> u32 {
    input
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(j, col_value)| match is_low_point(i, j, &input) {
                    true => Some(1 + col_value),
                    false => None,
                })
                .sum::<u32>()
        })
        .sum::<u32>()
}

fn is_low_point(i: usize, j: usize, input: &Vec<Vec<u32>>) -> bool {
    if j > 0 && input[i][j - 1] <= input[i][j] {
        return false;
    }
    if j < input[i].len() - 1 && input[i][j + 1] <= input[i][j] {
        return false;
    }

    if i > 0 && input[i - 1][j] <= input[i][j] {
        return false;
    }
    if i < input.len() - 1 && input[i + 1][j] <= input[i][j] {
        return false;
    }

    true
}

fn part_two(input: &Vec<Vec<u32>>) -> u32 {
    let mut visited_cells: HashSet<(usize, usize)> = HashSet::new();
    let mut basins = input
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(j, _)| match is_low_point(i, j, &input) {
                    true => Some(find_basin(i, j, input, &mut visited_cells)),
                    false => None,
                })
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<u32>>();

    basins.sort_by(|a, b| b.cmp(a));

    // first 3 basins
    (0..3).fold(1, |acc, basin| acc * basin)
}

fn find_basin(
    i: usize,
    j: usize,
    input: &Vec<Vec<u32>>,
    visited_cells: &mut HashSet<(usize, usize)>,
) -> u32 {
    let col_value = input[i][j];

    if col_value == 9 || visited_cells.contains(&(i, j)) {
        return 0;
    }

    visited_cells.insert((i, j));

    let mut value = 1;

    if j > 0 && input[i][j - 1] > col_value {
        value += find_basin(i, j - 1, input, visited_cells);
    }
    if j < input[i].len() - 1 && input[i][j + 1] > col_value {
        value += find_basin(i, j + 1, input, visited_cells);
    }
    if i > 0 && input[i - 1][j] > col_value {
        value += find_basin(i - 1, j, input, visited_cells);
    }
    if i < input.len() - 1 && input[i + 1][j] > col_value {
        value += find_basin(i + 1, j, input, visited_cells);
    }

    value
}
