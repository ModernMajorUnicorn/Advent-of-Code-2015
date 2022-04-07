use std::fs::File;
use std::io;
use std::io::BufRead;
use itertools::{Itertools};

struct AuntSue {
    number: Option<i64>,
    children: Option<i64>,
    cats: Option<i64>,
    samoyeds: Option<i64>,
    pomeranians: Option<i64>,
    akitas: Option<i64>,
    vizslas: Option<i64>,
    goldfish: Option<i64>,
    trees: Option<i64>,
    cars: Option<i64>,
    perfumes: Option<i64>,
}

impl AuntSue {
    fn new() -> AuntSue {
        AuntSue {
            number: None,
            children: None,
            cats: None,
            samoyeds: None,
            pomeranians: None,
            akitas: None,
            vizslas: None,
            goldfish: None,
            trees: None,
            cars: None,
            perfumes: None,
        }
    }

    fn get_ticker_tape() -> AuntSue {
        AuntSue {
            number: None,
            children: Some(3),
            cats: Some(7),
            samoyeds: Some(2),
            pomeranians: Some(3),
            akitas: Some(0),
            vizslas: Some(0),
            goldfish: Some(5),
            trees: Some(3),
            cars: Some(2),
            perfumes: Some(1),
        }
    }

    fn compare_to(&self, other: &AuntSue) -> bool {
        AuntSue::compare_value(self.children, other.children) &&
            AuntSue::compare_value(self.cats, other.cats) &&
            AuntSue::compare_value(self.samoyeds, other.samoyeds) &&
            AuntSue::compare_value(self.pomeranians, other.pomeranians) &&
            AuntSue::compare_value(self.akitas, other.akitas) &&
            AuntSue::compare_value(self.vizslas, other.vizslas) &&
            AuntSue::compare_value(self.goldfish, other.goldfish) &&
            AuntSue::compare_value(self.trees, other.trees) &&
            AuntSue::compare_value(self.cars, other.cars) &&
            AuntSue::compare_value(self.perfumes, other.perfumes)
    }

    fn compare_value(x: Option<i64>, y: Option<i64>) -> bool {
        match (x, y) {
            (Some(x), Some(y)) => x == y,
            _ => true
        }
    }
}

fn main() {
    let result = run_part_1();
    println!("Part 1 result: {}", result);
}

fn run_part_1() -> i64 {
    let ticker_tape = AuntSue::get_ticker_tape();
    let puzzle_input = load_puzzle_input();

    let found_aunt = puzzle_input
        .iter()
        .filter(|aunt_sue| ticker_tape.compare_to(aunt_sue))
        .at_most_one();

    match found_aunt {
        Ok(Some(aunt_sue)) => aunt_sue.number.unwrap(),
        _ => panic!("No definitive match found!"),
    }
}

fn load_puzzle_input() -> Vec<AuntSue> {
    io::BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let line_tokens: Vec<&str> = line.split(' ').collect();
            let mut aunt_sue = AuntSue::new();
            aunt_sue.number = Some(line_tokens[1].replace(":", "").parse().unwrap());

            for i in (2..line_tokens.len()).step_by(2) {
                let count: i64 = line_tokens[i + 1].replace(",", "").parse().unwrap();

                match line_tokens[i] {
                    "children:" => aunt_sue.children = Some(count),
                    "cats:" => aunt_sue.cats = Some(count),
                    "samoyeds:" => aunt_sue.samoyeds = Some(count),
                    "pomeranians:" => aunt_sue.pomeranians = Some(count),
                    "akitas:" => aunt_sue.akitas = Some(count),
                    "vizslas:" => aunt_sue.vizslas = Some(count),
                    "goldfish:" => aunt_sue.goldfish = Some(count),
                    "trees:" => aunt_sue.trees = Some(count),
                    "cars:" => aunt_sue.cars = Some(count),
                    "perfumes:" => aunt_sue.perfumes = Some(count),
                    _ => panic!("Unknown puzzle input: {}", line_tokens[i]),
                }
            }

            aunt_sue
        }).collect()
}