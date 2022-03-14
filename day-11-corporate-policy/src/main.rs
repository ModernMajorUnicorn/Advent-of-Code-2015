use std::char::from_u32;
use itertools::Itertools;

fn main() {
    let input = String::from("vzbxkghb");

    let result_1 = solve(input);
    println!("Part 1: {:?}", result_1);

    let result_2 = solve(result_1);
    println!("Part 2: {:?}", result_2);
}

fn solve(input: String) -> String {
    let mut working_copy = input.chars().collect_vec();
    increment_password(&mut working_copy);

    while !meets_policy(&working_copy) {
        increment_password(&mut working_copy);
    }

    String::from_iter(working_copy.iter())
}

fn increment_password(password: &mut Vec<char>) {
    let mut carry_flag = true;
    let mut i = password.len() - 1;

    while carry_flag {
        if i == 0 && password[0] == 'z' {
            password[0] = 'a';
            password.insert(0, 'a');
            carry_flag = false;
        } else if password[i] == 'z' {
            password[i] = 'a';
            i -= 1;
        } else {
            password[i] = from_u32(password[i] as u32 + 1).unwrap();
            carry_flag = false;
        }
    }
}

fn meets_policy(password: &Vec<char>) -> bool {
    meets_rule_1(password) && meets_rule_2(password) && meets_rule_3(password)
}

fn meets_rule_1(password: &Vec<char>) -> bool {
    password.iter().tuple_windows().any(|(a, b, c)| *c as u32 == *b as u32 + 1 && *b as u32 == *a as u32 + 1)
}

fn meets_rule_2(password: &Vec<char>) -> bool {
    password.iter().all(|character| *character != 'i' && *character != 'o' && *character != 'l')
}

fn meets_rule_3(password: &Vec<char>) -> bool {
    let pairs = password.iter().tuple_windows().map(|(a, b): (&char, &char)| (*a, *b)).collect_vec();

    let first_pair = match pairs.iter().find(|(a, b)| a == b) {
        None => return false,
        Some(x) => *x
    };

    pairs.iter().any(|(a, b)| a == b && (*a, *b) != first_pair)
}