use std::collections::HashMap;
use std::fs;

fn main() {
    let input = parse_input();

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn parse_input() -> Vec<Vec<char>> {
    fs::read_to_string("2015/day-05/input.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn part_one(input: &Vec<Vec<char>>) -> usize {
    input
        .iter()
        .filter(|&line| {
            let mut n_vowels = 0;
            let mut appears_twice_in_a_row = false;

            for c in line.windows(2) {
                if c == ['a', 'b'] || c == ['c', 'd'] || c == ['p', 'q'] || c == ['x', 'y'] {
                    return false;
                }

                if is_vowel(c[0]) {
                    n_vowels += 1;
                }

                if c[0] == c[1] {
                    appears_twice_in_a_row = true;
                }
            }

            if is_vowel(*(&line[line.len() - 1..][0])) {
                n_vowels += 1;
            }

            return n_vowels >= 3 && appears_twice_in_a_row;
        })
        .count()
}

fn part_two(input: &Vec<Vec<char>>) -> usize {
    input
        .iter()
        .filter(|&line| {
            let mut pairs = HashMap::<&[char], usize>::new();
            let mut previous_pair: Option<&[char]> = None;
            let mut pair_appears_twice = false;
            let mut one_letter_between = false;

            for (i, pair) in line.windows(2).enumerate() {
                one_letter_between = previous_pair
                    .map(|p| p[0] == pair[1] && p[1] == pair[0])
                    .filter(|p| *p)
                    .unwrap_or(one_letter_between);

                previous_pair = Some(pair);

                if pairs.contains_key(pair) {
                    pair_appears_twice = pairs
                        .get(pair)
                        .map(|v| i - *v > 1)
                        .filter(|p| *p)
                        .unwrap_or(pair_appears_twice);
                } else {
                    pairs.insert(pair, i);
                }
            }

            return pair_appears_twice && one_letter_between;
        })
        .count()
}

fn is_vowel(c: char) -> bool {
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}
