use itertools::{Itertools};
use std::time::Instant;

const PUZZLE_INPUT: [i64; 28] = [1, 3, 5, 11, 13, 17, 19, 23, 29, 31, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113];

fn main() {
    let result = run();
    println!("Part 1 result: {}", result);
}

fn get_puzzle_input() -> Vec<i64> {
    let mut result: Vec<i64> = Vec::new();

    for x in PUZZLE_INPUT.iter() {
        result.push(*x);
    }

    result
}

fn run() -> i64 {
    let packages = get_puzzle_input();
    let target_weight = packages.iter().sum::<i64>() / 3;
    let mut minimum_quantum_entanglement: Option<i64> = None;
    let mut previous_group_size = usize::MAX;

    for k in 1..=packages.len() {
        for group_reference in packages.iter().combinations(k) {
            let group = group_reference.iter().map(|x| **x).collect_vec();
            //println!("{} - {:?}", 3, group);

            if group.iter().sum::<i64>() == target_weight {

                if minimum_quantum_entanglement.is_some() && previous_group_size < group.len() {
                    return minimum_quantum_entanglement.unwrap();
                }

                let quantum_entanglement = mul(&group);

                match minimum_quantum_entanglement {
                    Some(x) =>
                        if quantum_entanglement < x && run_rec(sub(&packages, &group), target_weight, 2) {
                            minimum_quantum_entanglement = Some(quantum_entanglement);
                            //println!("{}", quantum_entanglement);
                        }
                    None => if run_rec(sub(&packages, &group), target_weight, 2) {
                        minimum_quantum_entanglement = Some(quantum_entanglement);
                        //println!("{}", quantum_entanglement);
                    }
                }

                previous_group_size = group.len();
            }
        }
    }

    panic!("No result found")
}

fn run_rec(packages: Vec<i64>, target_weight: i64, required_groups: i64) -> bool {
    if required_groups == 0 {
        return packages.len() == 0;
    }

    for k in 1..=packages.len() {
        for group_reference in packages.iter().combinations(k) {
            let group = group_reference.iter().map(|x| **x).collect_vec();

            //println!("{} - {:?}", required_groups, group);
            if group.iter().sum::<i64>() == target_weight {
                let sub_packages = sub(&packages, &group);

                if run_rec(sub_packages, target_weight, required_groups - 1) {
                    //println!("Found");
                    return true;
                }
            }
        }
    }


    //println!("Rejected");
    return false;
}

fn sub(packages: &Vec<i64>, subtracted: &Vec<i64>) -> Vec<i64> {
    packages.iter().filter(|x| !subtracted.contains(x)).map(|x| *x).collect_vec()
}

fn mul(packages: &Vec<i64>) -> i64 {
    let mut product = 1;

    for package in packages {
        product *= package;
    }

    product
}