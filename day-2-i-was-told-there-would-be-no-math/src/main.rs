use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() {
    let input = parse_input();

    let (part_1_solution, part_2_solution) = solve(input);

    println!("Part 1 solution: {}", part_1_solution);
    println!("Part 2 solution: {}", part_2_solution);
}

fn parse_input() -> Vec<[i64; 3]> {
    let lines = io::BufReader::new(File::open("input.txt").unwrap()).lines().map(|line| line.unwrap());

    lines.map(|line| {
        let mut split_line = line.split("x");
        [split_line.nth(0).unwrap().parse().unwrap(), split_line.nth(0).unwrap().parse().unwrap(), split_line.nth(0).unwrap().parse().unwrap()]
    }).collect()
}

fn solve(input: Vec<[i64; 3]>) -> (i64, i64) {
    let mut total_wrapping_paper = 0;
    let mut total_ribbon = 0;

    for mut present in input {
        present.sort();
        let [a, b, c] = present;

        total_wrapping_paper += 2*a*b + 2*a*c + 2*b*c;
        total_wrapping_paper += a*b;

        total_ribbon += 2*a + 2*b + a*b*c;
    }

    (total_wrapping_paper, total_ribbon)
}