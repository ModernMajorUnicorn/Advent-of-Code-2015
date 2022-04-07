use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::BufRead;
use itertools::{Itertools};
use compare::{Compare, natural};
use std::cmp::Ordering::{Less, Equal, Greater};

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

    fn compare_to_part_1(&self, other: &AuntSue) -> bool {
        AuntSue::compare_value_part_1(self.children, other.children) &&
            AuntSue::compare_value_part_1(self.cats, other.cats) &&
            AuntSue::compare_value_part_1(self.samoyeds, other.samoyeds) &&
            AuntSue::compare_value_part_1(self.pomeranians, other.pomeranians) &&
            AuntSue::compare_value_part_1(self.akitas, other.akitas) &&
            AuntSue::compare_value_part_1(self.vizslas, other.vizslas) &&
            AuntSue::compare_value_part_1(self.goldfish, other.goldfish) &&
            AuntSue::compare_value_part_1(self.trees, other.trees) &&
            AuntSue::compare_value_part_1(self.cars, other.cars) &&
            AuntSue::compare_value_part_1(self.perfumes, other.perfumes)
    }

    fn compare_value_part_1(x: Option<i64>, y: Option<i64>) -> bool {
        match (x, y) {
            (Some(x), Some(y)) => x == y,
            _ => true
        }
    }

    fn compare_to_part_2(&self, other: &AuntSue) -> bool {
        AuntSue::compare_value_part_2(self.children, other.children, Equal) &&
            AuntSue::compare_value_part_2(self.cats, other.cats, Less) &&
            AuntSue::compare_value_part_2(self.samoyeds, other.samoyeds, Equal) &&
            AuntSue::compare_value_part_2(self.pomeranians, other.pomeranians, Greater) &&
            AuntSue::compare_value_part_2(self.akitas, other.akitas, Equal) &&
            AuntSue::compare_value_part_2(self.vizslas, other.vizslas, Equal) &&
            AuntSue::compare_value_part_2(self.goldfish, other.goldfish, Greater) &&
            AuntSue::compare_value_part_2(self.trees, other.trees, Less) &&
            AuntSue::compare_value_part_2(self.cars, other.cars, Equal) &&
            AuntSue::compare_value_part_2(self.perfumes, other.perfumes, Equal)
    }

    fn compare_value_part_2(x: Option<i64>, y: Option<i64>, expected_ordering: Ordering) -> bool {
        let cmp = natural();

        match (x, y) {
            (Some(x), Some(y)) => cmp.compare(&x, &y) == expected_ordering,
            _ => true
        }
    }
}

fn main() {
    let ticker_tape = AuntSue::get_ticker_tape();
    let puzzle_input = load_puzzle_input();

    let found_aunt_part_1 = puzzle_input
        .iter()
        .filter(|aunt_sue| ticker_tape.compare_to_part_1(aunt_sue))
        .at_most_one();

    let found_aunt_part_2 = puzzle_input
        .iter()
        .filter(|aunt_sue| ticker_tape.compare_to_part_2(aunt_sue))
        .at_most_one();

    match found_aunt_part_1 {
        Ok(Some(aunt_sue)) => println!("Part 1: {}", aunt_sue.number.unwrap()),
        _ => println!("Part 1: No definitive match found"),
    }

    match found_aunt_part_2 {
        Ok(Some(aunt_sue)) => println!("Part 2: {}", aunt_sue.number.unwrap()),
        _ => println!("Part 2: No definitive match found"),
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