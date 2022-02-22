use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::Read;
use crate::CurrentActor::{Robot, Santa};

fn main() {
    let input = parse_input();

    let (part_1_solution, part_2_solution) = solve(input);

    println!("Part 1 solution: {}", part_1_solution);
    println!("Part 2 solution: {}", part_2_solution);
}

fn parse_input() -> Vec<char> {
    let mut buf: String = String::new();
    let _ = io::BufReader::new(File::open("input.txt").unwrap()).read_to_string(&mut buf);
    buf.chars().collect::<Vec<_>>()
}

fn solve(input: Vec<char>) -> (usize, usize) {
    (solve_santa(&input), solve_santa_and_robot(&input))
}

fn solve_santa(input: &Vec<char>) -> usize {
    let (mut x, mut y) = (0, 0);
    let mut visited_houses: HashSet<(i64, i64)> = HashSet::from([(0, 0)]);

    for direction in input {
        match direction {
            '<' => x -= 1,
            '>' => x += 1,
            '^' => y -= 1,
            'v' => y += 1,
            _ => {}
        }

        visited_houses.insert((x, y));
    }

    visited_houses.len()
}

#[derive(Copy, Clone)]
enum CurrentActor {
    Santa,
    Robot
}

fn solve_santa_and_robot(input: &Vec<char>) -> usize {
    let (mut x_santa, mut y_santa, mut x_robot, mut y_robot) = (0, 0, 0, 0);
    let mut visited_houses: HashSet<(i64, i64)> = HashSet::from([(0, 0)]);
    let mut current_actor = Santa;

    for direction in input {
        let (x, y) = match current_actor {
            Santa =>
                {
                    current_actor = Robot;
                    (&mut x_santa, &mut y_santa)
                }
            Robot =>
                {
                    current_actor = Santa;
                    (&mut x_robot, &mut y_robot)
                }
        };

        match direction {
            '<' => *x -= 1,
            '>' => *x += 1,
            '^' => *y -= 1,
            'v' => *y += 1,
            _ => {}
        }

        visited_houses.insert((*x, *y));
    }

    visited_houses.len()
}