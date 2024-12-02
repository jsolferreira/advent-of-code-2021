use std::{collections::HashMap, fs};

struct ParseResult {
    packet_version_sum: u64,
    value: u64,
    bits_read: u64,
}

struct ParseLiteralPacketResult {
    value: u64,
    bits_read: u64,
}

fn main() {
    let input = parse_input();

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn parse_input() -> String {
    fs::read_to_string("2021/day-16/input.txt").unwrap()
}

fn part_one(input: &String) -> u64 {
    let binary = hexa_to_binary(&input);
    parse_packet(&binary).packet_version_sum
}

fn part_two(input: &String) -> u64 {
    let binary = hexa_to_binary(&input);
    parse_packet(&binary).value
}

fn hexa_to_binary(input: &String) -> Vec<u64> {
    let map: HashMap<char, [u64; 4]> = HashMap::from([
        ('0', [0, 0, 0, 0]),
        ('1', [0, 0, 0, 1]),
        ('2', [0, 0, 1, 0]),
        ('3', [0, 0, 1, 1]),
        ('4', [0, 1, 0, 0]),
        ('5', [0, 1, 0, 1]),
        ('6', [0, 1, 1, 0]),
        ('7', [0, 1, 1, 1]),
        ('8', [1, 0, 0, 0]),
        ('9', [1, 0, 0, 1]),
        ('A', [1, 0, 1, 0]),
        ('B', [1, 0, 1, 1]),
        ('C', [1, 1, 0, 0]),
        ('D', [1, 1, 0, 1]),
        ('E', [1, 1, 1, 0]),
        ('F', [1, 1, 1, 1]),
    ]);

    input
        .chars()
        .flat_map(|c| *map.get(&c).unwrap())
        .collect::<Vec<u64>>()
}

fn parse_packet(input: &[u64]) -> ParseResult {
    let packet_version = get_packet_version(&input);
    let packet_type_id = get_packet_type_id(&input);

    if packet_type_id == 4 {
        let parse_result = parse_literal_packet(input);
        ParseResult {
            packet_version_sum: packet_version,
            value: parse_result.value,
            bits_read: parse_result.bits_read,
        }
    } else {
        let parse_result = parse_operator_packet(input);

        ParseResult {
            packet_version_sum: parse_result.packet_version_sum + packet_version,
            value: parse_result.value,
            bits_read: parse_result.bits_read,
        }
    }
}

fn get_packet_version(input: &[u64]) -> u64 {
    binary_to_decimal(&input[0..3])
}

fn get_packet_type_id(input: &[u64]) -> u64 {
    binary_to_decimal(&input[3..6])
}

fn get_length_type_id(input: &[u64]) -> u64 {
    *(&input[6])
}

fn get_subpackets_length(input: &[u64]) -> u64 {
    binary_to_decimal(&input[7..22])
}

fn get_number_of_subpackets(input: &[u64]) -> u64 {
    binary_to_decimal(&input[7..18])
}

fn binary_to_decimal(binary: &[u64]) -> u64 {
    let mut decimal = 0;
    for bit in 0..binary.len() {
        decimal += 2_u64.pow((binary.len() - 1 - bit) as u32) * binary[bit];
    }

    decimal
}

fn parse_literal_packet(input: &[u64]) -> ParseLiteralPacketResult {
    let packet_value = &input[6..];
    let mut binary_value: Vec<u64> = Vec::new();
    let mut bits_read = 6;

    for i in packet_value.chunks(5) {
        binary_value.extend_from_slice(&i[1..5]);
        bits_read += 5;

        if i[0] == 0 {
            break;
        }
    }

    ParseLiteralPacketResult {
        value: binary_to_decimal(&binary_value),
        bits_read,
    }
}

fn parse_operator_packet(input: &[u64]) -> ParseResult {
    let length_type_id = get_length_type_id(&input);
    let packet_type_id = get_packet_type_id(&input);
    let mut packet_version_sum = 0;
    let mut value;
    let mut bits_read;

    if length_type_id == 0 {
        let subpackets_length = get_subpackets_length(&input);
        let mut start_pos = 22;
        let end_pos = 22 + subpackets_length;
        bits_read = 22;

        let parse_result = parse_packet(&input[start_pos as usize..end_pos as usize]);
        start_pos += parse_result.bits_read;
        packet_version_sum += parse_result.packet_version_sum;
        bits_read += parse_result.bits_read;

        value = parse_result.value;

        while start_pos < end_pos {
            let parse_result = parse_packet(&input[start_pos as usize..end_pos as usize]);
            start_pos += parse_result.bits_read;
            packet_version_sum += parse_result.packet_version_sum;
            bits_read += parse_result.bits_read;

            value = apply_operation(packet_type_id, value, parse_result.value);
        }

        return ParseResult {
            packet_version_sum,
            value,
            bits_read,
        };
    } else {
        let mut number_of_subpackets = get_number_of_subpackets(&input);
        let mut start_pos = 18;
        bits_read = 18;

        let parse_result = parse_packet(&input[start_pos as usize..]);
        start_pos += parse_result.bits_read;
        packet_version_sum += parse_result.packet_version_sum;
        bits_read += parse_result.bits_read;
        number_of_subpackets -= 1;

        value = parse_result.value;

        while number_of_subpackets > 0 {
            let parse_result = parse_packet(&input[start_pos as usize..]);
            start_pos += parse_result.bits_read;
            packet_version_sum += parse_result.packet_version_sum;
            bits_read += parse_result.bits_read;
            number_of_subpackets -= 1;

            value = apply_operation(packet_type_id, value, parse_result.value);
        }
    }

    ParseResult {
        packet_version_sum,
        value,
        bits_read,
    }
}

fn apply_operation(packet_type_id: u64, a: u64, b: u64) -> u64 {
    if packet_type_id == 0 {
        return a + b;
    } else if packet_type_id == 1 {
        return a * b;
    } else if packet_type_id == 2 {
        return a.min(b);
    } else if packet_type_id == 3 {
        return a.max(b);
    } else if packet_type_id == 5 {
        return if a > b { 1 } else { 0 };
    } else if packet_type_id == 6 {
        return if a < b { 1 } else { 0 };
    } else if packet_type_id == 7 {
        return if a == b { 1 } else { 0 };
    }

    0
}
