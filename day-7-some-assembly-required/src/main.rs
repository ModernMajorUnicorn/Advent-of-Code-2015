mod logical;
mod parse;
mod signal_source;

use std::collections::HashMap;
use crate::logical::LogicalInstruction;

fn main() {
    run_test();
    run_puzzle();
}

fn run_test() {
    let test_input = parse::parse_input("test.txt");
    let test_solution = solve_part_1(&test_input);
    println!("{:?}", test_solution);
}

fn run_puzzle() {
    let input = parse::parse_input("input.txt");

    let solution_part_1 = *solve_part_1(&input).get("a").unwrap();
    println!("a is {}", solution_part_1);

    let solution_part_2 = *solve_part_2(&input, solution_part_1).get("a").unwrap();
    println!("a is now {}", solution_part_2);
}

fn solve_part_1(input: &Vec<Box<dyn LogicalInstruction>>) -> HashMap<String, u16> {
    let mut wires: HashMap<String, u16> = HashMap::new();
    let mut circuit_complete = false;

    while !circuit_complete {
        circuit_complete = true;

        for instruction in input.iter() {
            circuit_complete &= instruction.perform(&mut wires);
        }
    }

    wires
}

fn solve_part_2(input: &Vec<Box<dyn LogicalInstruction>>, b_override: u16) -> HashMap<String, u16> {
    let mut wires: HashMap<String, u16> = HashMap::new();
    wires.insert("b".to_string(), b_override);
    let mut circuit_complete = false;

    while !circuit_complete {
        circuit_complete = true;

        for instruction in input.iter() {
            if instruction.get_destination() != "b" {
                circuit_complete &= instruction.perform(&mut wires);
            }
        }
    }

    wires
}