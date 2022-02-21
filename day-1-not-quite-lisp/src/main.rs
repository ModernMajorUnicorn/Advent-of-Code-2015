use std::fs::File;
use std::io;
use std::io::Read;

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

fn solve(input: Vec<char>) -> (i64, usize) {
    let mut floor = 0;
    let mut basement_entered_at: Option<usize> = None;

    for (pos, c) in input.iter().enumerate() {
        if *c == '(' {
            floor += 1;
        } else if *c == ')' {
            floor -= 1;
        }

        if basement_entered_at.is_none() && floor == -1 {
            basement_entered_at = Some(pos + 1);
        }
    }

    (floor, basement_entered_at.unwrap())
}