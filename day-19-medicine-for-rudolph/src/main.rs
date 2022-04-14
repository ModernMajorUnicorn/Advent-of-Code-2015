use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() {
    let (test_replacements, test_start) = load_puzzle_input("test_input.txt");
    let test_result = run(test_replacements, test_start);
    println!("Result for Test: {}", test_result);

    let (puzzle_replacements, puzzle_start) = load_puzzle_input("input.txt");
    let puzzle_result = run(puzzle_replacements, puzzle_start);
    println!("Result for Puzzle: {}", puzzle_result);
}

fn load_puzzle_input(path: &str) -> (Vec<(String, String)>, String) {
    let lines: Vec<String> = io::BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let mut replacements: Vec<(String, String)> = Vec::new();

    for i in 0..(lines.len() - 2) {
        let mut line_parts = lines[i].split(" => ");
        let source = line_parts.next().unwrap().to_string();
        let destination = line_parts.next().unwrap().to_string();
        replacements.push((source, destination));
    }

    (replacements, lines.last().unwrap().to_string())
}

fn run(replacements: Vec<(String, String)>, start: String) -> usize {
    let mut variations = HashSet::new();

    for (source, destination) in replacements {
        let replacement_variations = run_one_replacement(source, destination, &start);

        for replacement_variation in replacement_variations {
            variations.insert(replacement_variation);
        }
    }

    variations.len()
}

fn run_one_replacement(source: String, destination: String, start: &String) -> Vec<String> {
    let parts: Vec<String> = start.split(&source).map(|part| part.to_string()).collect();
    let mut results: Vec<String> = Vec::new();

    for i in 1..parts.len() {
        let mut replaced_string = parts[0].to_string();

        for j in 1..parts.len() {
            if i == j {
                replaced_string.push_str(&destination);
            } else {
                replaced_string.push_str(&source);
            }

            replaced_string.push_str(&parts[j]);
        }

        results.push(replaced_string);
    }

    results
}