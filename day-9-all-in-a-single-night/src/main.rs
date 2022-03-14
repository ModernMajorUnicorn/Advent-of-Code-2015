use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;
use itertools::Itertools;

struct Map {
    locations: HashSet<String>,
    distances: HashMap<(String, String), i64>
}

fn main() {
    run("test.txt", "Test solution");
    run("input.txt", "Puzzle solution");
}

fn run(input_file: &str, output_prefix: &str) {
    let input = parse_input(input_file);
    let solution = solve(input);
    println!("{}: {:?}", output_prefix, solution);
}

fn parse_input(path: &str) -> Map {
    let mut locations: HashSet<String> = HashSet::new();
    let mut distances: HashMap<(String, String), i64> = HashMap::new();

    let lines = io::BufReader::new(File::open(path).unwrap())
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

fn solve(input: Map) -> (i64, i64) {
    let distances = input.locations
        .iter()
        .permutations(input.locations.len())
        .map(|path| calculate_distance(path, &input.distances))
        .collect_vec();

    (*distances.iter().min().unwrap(), *distances.iter().max().unwrap())
}

fn calculate_distance(path: Vec<&String>, distances: &HashMap<(String, String), i64>) -> i64 {
    path
        .iter()
        .tuple_windows()
        .map(|(source, destination)| distances.get(&(source.to_string(), destination.to_string())).unwrap())
        .sum()
}