use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

struct Line {
    start: (i32, i32),
    end: (i32, i32),
}

fn main() {
    let input = parse_input();

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn parse_input() -> Vec<Line> {
    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();

    return fs::read_to_string("2021/day-05/input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let captures = re.captures(line).unwrap();

            Line {
                start: (
                    captures[1].parse().unwrap(),
                    captures[2].parse().unwrap(),
                ),
                end: (
                    captures[3].parse().unwrap(),
                    captures[4].parse().unwrap(),
                ),
            }
        })
        .collect::<Vec<Line>>();
}

fn part_one(input: &Vec<Line>) -> u32 {
    let mut map = HashMap::<(i32, i32), u32>::new();
    let mut count = 0;

    for line in input {
        if line.start.0 == line.end.0 || line.start.1 == line.end.1 {
            let diff_vector = get_diff_vector(&line);

            add_to_map_and_count(&mut map, line.start, &mut count);

            let mut point = line.start;

            while point != line.end {
                point.0 = point.0 + 1 * diff_vector.0;
                point.1 = point.1 + 1 * diff_vector.1;

                add_to_map_and_count(&mut map, point, &mut count);
            }
        }
    }

    count
}

fn add_to_map_and_count(
    map: &mut HashMap<(i32, i32), u32>,
    point: (i32, i32),
    count: &mut u32,
) -> () {
    if !map.contains_key(&point) {
        map.insert(point, 1);
    } else {
        let v = map.get_mut(&point).unwrap();

        if *v == 1 {
            *count += 1;
            *v += 1;
        }
    }
}

fn part_two(input: &Vec<Line>) -> u32 {
    let mut map = HashMap::<(i32, i32), u32>::new();
    let mut count = 0;

    for line in input {
        let diff_vector = get_diff_vector(&line);

        add_to_map_and_count(&mut map, line.start, &mut count);

        let mut point = line.start;

        while point != line.end {
            point.0 = point.0 + 1 * diff_vector.0;
            point.1 = point.1 + 1 * diff_vector.1;

            add_to_map_and_count(&mut map, point, &mut count);
        }
    }

    count
}

fn get_diff_vector(line: &Line) -> (i32, i32) {
    let diff_x = match line.end.0.cmp(&line.start.0) {
        Ordering::Equal => 0,
        Ordering::Greater => 1,
        Ordering::Less => -1,
    };

    let diff_y = match line.end.1.cmp(&line.start.1) {
        Ordering::Equal => 0,
        Ordering::Greater => 1,
        Ordering::Less => -1,
    };

    (diff_x, diff_y)
}
