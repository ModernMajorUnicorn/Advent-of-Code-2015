use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;
use itertools::Itertools;

struct HappinessMap {
    guests: HashSet<String>,
    map_entries: HashMap<(String, String), i64>
}

impl HappinessMap {
    fn new() -> HappinessMap {
        HappinessMap {
            guests: HashSet::new(),
            map_entries: HashMap::new()
        }
    }

    fn add(&mut self, person_1: String, person_2: String, person_1_happiness: i64, modifier: &str) {
        self.guests.insert(person_1.to_string());
        self.guests.insert(person_2.to_string());

        if modifier == "gain" {
            self.map_entries.insert((person_1, person_2), person_1_happiness);
        } else {
            self.map_entries.insert((person_1, person_2), person_1_happiness * -1);
        }
    }

    fn with_self(&self) -> HappinessMap {
        let mut result = HappinessMap {
            guests: self.guests.clone(),
            map_entries: self.map_entries.clone()
        };

        for guest in self.guests.iter() {
            result.add(guest.to_string(), "You".to_string(), 0, "gain");
            result.add("You".to_string(), guest.to_string(), 0, "gain");
        }

        result
    }
}

fn main() {
    run("test.txt", "Test");
    run("input.txt", "Puzzle");
}

fn run(path: &str, name: &str) {
    let happiness_map = load_happiness_map(path);
    let max_happiness = calculate_max_happiness(&happiness_map);

    let happiness_map_self = happiness_map.with_self();
    let max_happiness_self = calculate_max_happiness(&happiness_map_self);

    println!("Part 1 result for {}: {}", name, max_happiness);
    println!("Part 2 result for {}: {}", name, max_happiness_self);
}

fn load_happiness_map(path: &str) -> HappinessMap {
    const PERSON_1_INDEX: usize = 0;
    const MODIFIER_INDEX: usize = 2;
    const HAPPINESS_INDEX: usize = 3;
    const PERSON_2_INDEX: usize = 10;

    let lines =
        io::BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|line| line.unwrap());

    let mut result = HappinessMap::new();

    for line in lines {
        let words = line.split(" ").collect_vec();
        let person_1 = words[PERSON_1_INDEX].to_string();
        let modifier = words[MODIFIER_INDEX];
        let happiness = words[HAPPINESS_INDEX];
        let person_2 = words[PERSON_2_INDEX].replace(".", "");

        result.add(person_1, person_2, happiness.parse().unwrap(), modifier)
    }

    result
}

fn calculate_max_happiness(happiness_map: &HappinessMap) -> i64 {
    happiness_map.guests.iter().permutations(happiness_map.guests.len())
        .map(|variation| {
            let mut total_happiness = 0;

            for i in 0..variation.len() - 1 {
                let person_1 = variation[i];
                let person_2 = variation[i + 1];

                total_happiness += happiness_map.map_entries.get(&(person_1.to_string(), person_2.to_string())).unwrap();
                total_happiness += happiness_map.map_entries.get(&(person_2.to_string(), person_1.to_string())).unwrap();
            }

            let person_1 = variation.last().unwrap();
            let person_2 = variation.first().unwrap();

            total_happiness += happiness_map.map_entries.get(&(person_1.to_string(), person_2.to_string())).unwrap();
            total_happiness += happiness_map.map_entries.get(&(person_2.to_string(), person_1.to_string())).unwrap();

            total_happiness
        }).max().unwrap()
}