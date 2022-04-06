use std::fs::File;
use std::io;
use std::io::BufRead;

#[derive(Copy, Clone)]
struct Reindeer {
    speed_in_kms: i64,
    running_time_in_seconds: i64,
    resting_time_in_seconds: i64
}

#[derive(Copy, Clone)]
struct ReindeerState {
    distance_in_km: i64,
    running_time_left_in_seconds: i64,
    resting_time_left_in_seconds: i64,
    reindeer_information: Reindeer,
    points: i64,
}

impl ReindeerState {
    fn new(reindeer_information: Reindeer) -> ReindeerState {
        ReindeerState {
            distance_in_km: 0,
            running_time_left_in_seconds: reindeer_information.running_time_in_seconds,
            resting_time_left_in_seconds: reindeer_information.resting_time_in_seconds,
            reindeer_information,
            points: 0,
        }
    }

    fn tick(&mut self) {
        if self.running_time_left_in_seconds > 0 {
            self.distance_in_km += self.reindeer_information.speed_in_kms;
            self.running_time_left_in_seconds -= 1;
        } else if self.resting_time_left_in_seconds > 0 {
            self.resting_time_left_in_seconds -= 1;
        } else {
            self.running_time_left_in_seconds = self.reindeer_information.running_time_in_seconds;
            self.resting_time_left_in_seconds = self.reindeer_information.resting_time_in_seconds;
            self.tick();
        }
    }

    fn score(&mut self) {
        self.points += 1;
    }
}

fn main() {
    run_test();
    run_puzzle();
}

fn run_test() {
    let puzzle_input = load_puzzle_input("test_input.txt");
    let (best_distance_in_km, best_points) = run(&puzzle_input, 1000);
    println!("Best distance for test: {}. Best points for test: {}", best_distance_in_km, best_points);
}

fn run_puzzle() {
    let puzzle_input = load_puzzle_input("input.txt");
    let (best_distance_in_km, best_points) = run(&puzzle_input, 2503);
    println!("Best distance for puzzle: {}. Best points for puzzle: {}", best_distance_in_km, best_points);
}

fn load_puzzle_input(path: &str) -> Vec<Reindeer> {
    io::BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let line_tokens: Vec<&str> = line.split(' ').collect();
            let speed_in_kms: i64 = line_tokens[3].parse().unwrap();
            let running_time_in_seconds: i64 = line_tokens[6].parse().unwrap();
            let resting_time_in_seconds: i64 = line_tokens[13].parse().unwrap();
            Reindeer { speed_in_kms, running_time_in_seconds, resting_time_in_seconds }
        }).collect()
}

fn run(puzzle_input: &Vec<Reindeer>, duration_in_seconds: i64) -> (i64, i64) {
    let mut reindeer_states: Vec<ReindeerState> = puzzle_input
        .iter()
        .map(|reindeer| ReindeerState::new(*reindeer))
        .collect();

    for _ in 0..duration_in_seconds {
        reindeer_states.iter_mut().for_each(|reindeer_state| reindeer_state.tick());

        let best_current_distance_in_km = reindeer_states
            .iter()
            .map(|reindeer_state| reindeer_state.distance_in_km)
            .max()
            .unwrap();

        reindeer_states
            .iter_mut()
            .filter(|reindeer_state| reindeer_state.distance_in_km == best_current_distance_in_km)
            .for_each(|reindeer_state| reindeer_state.score());
    }

    let best_distance_in_km = reindeer_states
        .iter()
        .map(|reindeer_state| reindeer_state.distance_in_km)
        .max()
        .unwrap();

    let best_points = reindeer_states
        .iter()
        .map(|reindeer_state| reindeer_state.points)
        .max()
        .unwrap();

    (best_distance_in_km, best_points)
}