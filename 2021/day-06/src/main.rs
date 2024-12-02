use std::fs;

fn main() {
    let mut input = parse_input();

    println!("Part one: {}", part_one(&mut input));
    println!("Part two: {}", part_two(&mut input));
}

fn parse_input() -> Vec<u32> {
    return fs::read_to_string("2021/day-06/input.txt")
        .unwrap()
        .split(",")
        .map(|c| c.parse().unwrap())
        .collect::<Vec<u32>>();
}

fn part_one(input: &mut Vec<u32>) -> usize {
    return count_lanterfish(input, 80, 1);
}

fn part_two(input: &mut Vec<u32>) -> usize {
    return count_lanterfish(input, 256, 1);
}

fn count_lanterfish(input: &mut Vec<u32>, days_remaining: u32, mut min: u32) -> usize {
    if input.is_empty() {
        return 0;
    }

    let mut new_fish: Vec<u32> = Vec::new();
    //let min = *input.iter().min().unwrap();

    //let mut min = 1;

    /* for i in input.iter_mut() {
        if *i == 0 {
            *i = 6;
            new_fish.push(8);
        } else {
            *i -= 1;
        }

        if *i < min {
            min = *i;
        }
    } */
    println!("{} {}", days_remaining, input.len());
    for i in input.iter_mut() {
        if *i == min - 1 {
            *i = 6;
            new_fish.push(8);
        } else {
            *i -= min;
        }

        if *i > 0 && *i < min {
            min = *i;
        }
    }

    if days_remaining != 0 {
        input.append(&mut new_fish);
        //println!("{:?} {} {}", input, day, min);
        //println!("{} {}", input.len(), day);
        return count_lanterfish(input, days_remaining - 1, min);
        //return part_one_aux(input, day - 1, min) + part_one_aux(&mut new_fish, day - 1, min);
    } else {
        return input.len();
    }
}
