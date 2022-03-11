use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;
use itertools::Itertools;

fn main() {
    run_test();
}

fn run_test() {
    //let input = parse_input("test.txt");

    //let test_solution = solve_part_1(&input);

    //println!("Test solution: {}", test_solution);
}

fn parse_input(path: &str) -> Map {
    let mut locations: HashSet<String> = HashSet::new();
    let mut distances: HashMap<(String, String), i64> = HashMap::new();

    let mut lines = io::BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|line| line.unwrap());

    for line in lines {
        let (location_a, location_b, distance) = parse_line(line);

        locations.insert(location_a.to_string());
        locations.insert(location_b.to_string());

        distances.insert((location_a.to_string(), location_b.to_string()), distance);
        distances.insert((location_b.to_string(), location_a.to_string()), distance);
    }

    Map {
        locations, distances
    }
}

fn parse_line(line: String) -> (String, String, i64) {
    let (connection, distance): (&str, &str) = line.split(" = ").collect_tuple().unwrap();
    let (location_a, location_b): (&str, &str) = connection.split(" to ").collect_tuple().unwrap();

    (location_a.to_string(), location_b.to_string(), distance.parse().unwrap())
}

struct Map {
    locations: HashSet<String>,
    distances: HashMap<(String, String), i64>
}