use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() {
    let (test_replacements, test_start) = load_puzzle_input("test_input.txt");
    let test_result = run_part_1(&test_replacements, &test_start);
    println!("Result for Test: {}", test_result);

    let (puzzle_replacements, medicine_molecule) = load_puzzle_input("input.txt");

    let part_1_result = run_part_1(&puzzle_replacements, &medicine_molecule);
    println!("Result for Puzzle Part 1: {}", part_1_result);

    let part_2_result = run_part_2(&puzzle_replacements, &medicine_molecule);
    println!("Result for Puzzle Part 2: {}", part_2_result);
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

fn run_part_1(replacements: &Vec<(String, String)>, start: &String) -> usize {
    let mut variations = HashSet::new();

    for (source, destination) in replacements {
        let replacement_variations = run_one_replacement(source, destination, &start);

        for replacement_variation in replacement_variations {
            variations.insert(replacement_variation);
        }
    }

    variations.len()
}

fn run_one_replacement(source: &String, destination: &String, start: &String) -> Vec<String> {
    let parts: Vec<String> = start.split(source).map(|part| part.to_string()).collect();
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

fn run_part_2(replacements: &Vec<(String, String)>, molecule: &String) -> i64 {
    let mut flipped_replacements: Vec<(String, String)> = replacements.iter().map(|(source, destination)| (destination.to_string(), source.to_string())).collect();
    flipped_replacements.sort_by(|(a1, b1), (a2, b2)| (a2.len() - b2.len()).cmp(&(a1.len() - b1.len())));

    let mut current_variant = molecule.to_string();
    let mut distance = 0;

    while current_variant != "e" {
        for (source, destination) in flipped_replacements.iter() {
            if current_variant.contains(source) {
                current_variant = current_variant.replacen(source, destination, 1);
                distance += 1;
            }
        }
    }

    distance
}