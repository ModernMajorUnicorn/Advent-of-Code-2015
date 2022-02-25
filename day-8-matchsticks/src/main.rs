use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() {
    run_test();
    run_puzzle();
}

fn run_test() {
    let input = parse_input("test.txt");

    let test_solution = solve_part_1(&input);

    println!("Test solution: {}", test_solution);
}

fn run_puzzle() {
    let input = parse_input("input.txt");

    let part_1_solution = solve_part_1(&input);

    println!("Part 1 solution: {}", part_1_solution);

    let part_2_solution = solve_part_2(&input);

    println!("Part 2 solution: {}", part_2_solution);
}

fn parse_input(path: &str) -> Vec<String> {
    io::BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|line| line.unwrap())
        .collect()
}

#[derive(Copy, Clone)]
enum State {
    Normal,
    Escaped,
    EscapedHex1,
    EscapedHex2,
}

fn solve_part_1(input: &Vec<String>) -> usize {
    let mut result = 0;
    let mut state = State::Normal;

    for line in input {
        let characters: Vec<char> = line.chars().collect();
        let mut printed_length = 0;

        for character in characters.iter() {
            match (state, character) {
                (State::Normal, '\\') => state = State::Escaped,
                (State::Normal, _) => printed_length += 1,

                (State::Escaped, 'x') => state = State::EscapedHex1,
                (State::Escaped, _) => {
                    state = State::Normal;
                    printed_length += 1
                },

                (State::EscapedHex1, _) => state = State::EscapedHex2,
                (State::EscapedHex2, _) => {
                    state = State::Normal;
                    printed_length += 1;
                }
            }
        }

        printed_length -= 2;

        result += characters.len() - printed_length;
    }

    result
}

fn solve_part_2(input: &Vec<String>) -> usize {
    let mut result = 0;

    for line in input {
        let characters: Vec<char> = line.chars().collect();
        let mut encoded_length = 0;

        for character in characters.iter() {
            match character {
                '\\' | '"' => encoded_length += 2,
                _ => encoded_length += 1
            }
        }

        encoded_length += 2;

        result += encoded_length - characters.len();
    }

    result
}