use std::fs::File;
use std::io;
use std::io::BufRead;
use itertools::Itertools;

fn main() {
    let test_input = load_input("test_input.txt");
    let (test_result_1, test_result_2) = run(test_input, 25);
    println!("Result for Test: {} and {}", test_result_1, test_result_2);

    let puzzle_input = load_input("input.txt");
    let (puzzle_result_1, puzzle_result_2) = run(puzzle_input, 150);
    println!("Result for Puzzle: {} and {}", puzzle_result_1, puzzle_result_2);
}

fn load_input(path: &str) -> Vec<i64> {
    io::BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.parse().unwrap())
        .collect()
}

fn run(containers: Vec<i64>, amount: i64) -> (usize, usize) {
    let valid_combinations = (1..=containers.len())
        .map(|size| containers.iter().combinations(size))
        .flatten()
        .filter(|combination| combination.iter().map(|number| **number).sum::<i64>() == amount)
        .collect_vec();

    let part_1 = valid_combinations.len();

    let smallest_combination = valid_combinations
        .iter()
        .map(|combination| combination.len())
        .min()
        .unwrap();

    let part_2 = valid_combinations
        .iter()
        .filter(|combination| combination.len() == smallest_combination)
        .count();

    (part_1, part_2)
}