use std::fs;

fn main() {
    let input = parse_input();

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn parse_input() -> Vec<Vec<u32>> {
    fs::read_to_string("2021/day-03/input.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn part_one(input: &Vec<Vec<u32>>) -> usize {
    let mut total = 0;

    for i in (0..5).rev() {
        let mut total_0 = 0;
        let mut total_1 = 0;

        for bits in input {
            if bits[i] == 0 {
                total_0 += 1;
            } else if bits[i] == 1 {
                total_1 += 1;
            }
        }

        if total_1 > total_0 {
            total += 2 ^ (4 - i);
        }
    }

    total
}

fn part_two(input: &Vec<Vec<u32>>) -> u32 {
    let oxygen_generator_rating = get_oxygen_generator_rating(&input, 0);
    let co2_scrubber_rating = get_co2_scrubber_rating(&input, 0);

    oxygen_generator_rating * co2_scrubber_rating
}

fn get_oxygen_generator_rating(input: &Vec<Vec<u32>>, bit_pos: usize) -> u32 {
    let mut arr_of_zeroes = Vec::new();
    let mut arr_of_ones = Vec::new();

    if input.len() == 1 {
        return binary_to_decimal(&input[0]);
    }

    for bits in input {
        if bits[bit_pos] == 0 {
            arr_of_zeroes.push(bits.clone());
        } else if bits[bit_pos] == 1 {
            arr_of_ones.push(bits.clone());
        }
    }

    if arr_of_zeroes.len() > arr_of_ones.len() {
        return get_oxygen_generator_rating(&arr_of_zeroes, bit_pos + 1);
    } else {
        return get_oxygen_generator_rating(&arr_of_ones, bit_pos + 1);
    }
}

fn get_co2_scrubber_rating(input: &Vec<Vec<u32>>, bit_pos: usize) -> u32 {
    let mut arr_of_zeroes = Vec::new();
    let mut arr_of_ones = Vec::new();

    if input.len() == 1 {
        return binary_to_decimal(&input[0]);
    }

    for bits in input {
        if bits[bit_pos] == 0 {
            arr_of_zeroes.push(bits.clone());
        } else if bits[bit_pos] == 1 {
            arr_of_ones.push(bits.clone());
        }
    }

    if arr_of_zeroes.len() <= arr_of_ones.len() {
        return get_co2_scrubber_rating(&arr_of_zeroes, bit_pos + 1);
    } else {
        return get_co2_scrubber_rating(&arr_of_ones, bit_pos + 1);
    }
}

fn binary_to_decimal(binary: &[u32]) -> u32 {
    let mut decimal = 0;
    for bit in 0..binary.len() {
        decimal += 2_u32.pow((binary.len() - 1 - bit) as u32) * binary[bit];
    }

    decimal
}
