use std::fs;

type Bingo = (Vec<u32>, Vec<Board>);

type BoardCell = (u32, bool);
type BoardRow = Vec<BoardCell>;
type Board = Vec<BoardRow>;

fn main() {
    let mut input = parse_input();

    println!("Part one: {}", part_one(&mut input));
    println!("Part two: {}", part_two(&mut input));
}

fn parse_input() -> Bingo {
    let input: Vec<String> = fs::read_to_string("2021/day-04/input.txt")
        .unwrap()
        .split("\r\n\r\n")
        .map(|s| s.to_string())
        .collect();

    let numbers_drawn: Vec<u32> = input
        .first()
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();

    let boards: Vec<Board> = input
        .iter()
        .skip(1)
        .map(|board_str| {
            board_str
                .split("\r\n")
                .map(|row_str| {
                    row_str
                        .split_whitespace()
                        .map(|n| (n.parse().unwrap(), false))
                        .collect::<BoardRow>()
                })
                .collect::<Vec<BoardRow>>()
        })
        .collect();

    (numbers_drawn, boards)
}

fn part_one((numbers_drawn, boards): &mut Bingo) -> u32 {
    for number in numbers_drawn {
        for board in boards.iter_mut() {
            let is_winner = draw_number(board, *number);

            if is_winner {
                return board_score(board, *number);
            }
        }
    }

    panic!("No winner")
}

fn draw_number(board: &mut Board, number: u32) -> bool {
    for board_row in board.iter_mut() {
        for (i, board_cell) in board_row.into_iter().enumerate() {
            if board_cell.0 == number {
                board_cell.1 = true;
                return check_complete_row(board_row) || check_complete_column(board, i);
            }
        }
    }

    false
}

fn check_complete_row(board_row: &BoardRow) -> bool {
    board_row.iter().all(|cell| cell.1 == true)
}

fn check_complete_column(board: &Board, i: usize) -> bool {
    board.iter().all(|row| row[i].1 == true)
}

fn board_score(board: &Board, last_number_drawn: u32) -> u32 {
    let sum: u32 = board
        .iter()
        .map(|board_row| {
            board_row
                .into_iter()
                .filter_map(|cell| match cell.1 {
                    false => Some(cell.0),
                    true => None,
                })
                .sum::<u32>()
        })
        .sum();

    sum * last_number_drawn
}

fn part_two((numbers_drawn, boards): &mut Bingo) -> u32 {
    let mut remaining_boards = boards.len();
    let mut winners: Vec<usize> = Vec::new();
    for number in numbers_drawn {
        for (index, board) in boards.iter_mut().enumerate() {
            if !winners.contains(&index) {
                let is_winner = draw_number(board, *number);

                if is_winner {
                    if remaining_boards == 1 {
                        return board_score(board, *number);
                    } else {
                        winners.push(index);
                        remaining_boards -= 1;
                    }
                }
            }
        }
    }

    panic!("No winner")
}
