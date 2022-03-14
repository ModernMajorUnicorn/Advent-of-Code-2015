use std::fs::File;
use std::io;
use std::io::Read;
use serde_json::{Value};

fn main() {
    let json_data = parse_input();

    let part_1 = solve_part_1(&json_data);

    println!("Part 1: {}", part_1);

    let part_2 = solve_part_2(&json_data);

    println!("Part 2: {}", part_2);
}

fn parse_input() -> Value {
    let mut input_buffer = String::new();

    let _ = io::BufReader::new(File::open("input.txt").unwrap()).read_to_string(&mut input_buffer);

    serde_json::from_str(&input_buffer).unwrap()
}

fn solve_part_1(json_data: &Value) -> i64 {
    match json_data {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(number) => number.as_i64().unwrap(),
        Value::String(_) => 0,
        Value::Array(values) => values.iter().map(|value| solve_part_1(value)).sum(),
        Value::Object(json_data) => json_data.values().map(|value| solve_part_1(value)).sum(),
    }
}

fn solve_part_2(json_data: &Value) -> i64 {
    match json_data {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(number) => number.as_i64().unwrap(),
        Value::String(_) => 0,
        Value::Array(values) => values.iter().map(|value| solve_part_2(value)).sum(),
        Value::Object(json_data) if json_data.values().any(|x| x == &Value::String("red".to_string())) => 0,
        Value::Object(json_data) => json_data.values().map(|value| solve_part_2(value)).sum()
    }
}