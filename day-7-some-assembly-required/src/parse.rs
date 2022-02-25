use std::fs::File;
use std::io;
use std::io::BufRead;
use crate::logical::{AndInstruction, LogicalInstruction, LShiftInstruction, MovInstruction, NotInstruction, OrInstruction, RShiftInstruction};
use crate::signal_source::SignalSource;

pub fn parse_input(path: &str) -> Vec<Box<dyn LogicalInstruction>> {
    io::BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|line| line.unwrap())
        .map(|line| parse_line(line))
        .collect()
}

fn parse_line(line: String) -> Box<dyn LogicalInstruction> {
    let (source, destination) = separate_source_and_destination(line);
    let mut instruction = parse_operation(source);
    instruction.set_destination(destination);
    instruction
}

fn separate_source_and_destination(line: String) -> (String, String) {
    let parts: Vec<&str> = line.split(" -> ").collect();
    (parts[0].to_string(), parts[1].to_string())
}

fn parse_operation(operation: String) -> Box<dyn LogicalInstruction> {
    if operation.contains("AND") {
        Box::new(parse_and(operation))
    } else if operation.contains("OR") {
        Box::new(parse_or(operation))
    } else if operation.contains("NOT") {
        Box::new(parse_not(operation))
    } else if operation.contains("LSHIFT") {
        Box::new(parse_lshift(operation))
    } else if operation.contains("RSHIFT") {
        Box::new(parse_rshift(operation))
    } else {
        Box::new(parse_mov(operation))
    }
}

fn parse_signal_source(signal_source: String) -> SignalSource {
    let parse_result = signal_source.parse::<u16>();

    if parse_result.is_ok() {
        SignalSource {
            is_static: true,
            value: parse_result.unwrap(),
            wire_name: String::new()
        }
    } else {
        SignalSource {
            is_static: false,
            value: 0,
            wire_name: signal_source
        }
    }
}

fn parse_and(operation: String) -> AndInstruction {
    let parts: Vec<&str> = operation.split(" AND ").collect();

    AndInstruction {
        source_1: parse_signal_source(parts[0].to_string()),
        source_2: parse_signal_source(parts[1].to_string()),
        destination: String::new()
    }
}

fn parse_or(operation: String) -> OrInstruction {
    let parts: Vec<&str> = operation.split(" OR ").collect();

    OrInstruction {
        source_1: parse_signal_source(parts[0].to_string()),
        source_2: parse_signal_source(parts[1].to_string()),
        destination: String::new()
    }
}

fn parse_not(operation: String) -> NotInstruction {
    NotInstruction {
        source: parse_signal_source(operation.replace("NOT ", "")),
        destination: String::new()
    }
}

fn parse_lshift(operation: String) -> LShiftInstruction {
    let parts: Vec<&str> = operation.split(" LSHIFT ").collect();

    LShiftInstruction {
        source: parse_signal_source(parts[0].to_string()),
        shift_count: parts[1].parse().unwrap(),
        destination: String::new()
    }
}

fn parse_rshift(operation: String) -> RShiftInstruction {
    let parts: Vec<&str> = operation.split(" RSHIFT ").collect();

    RShiftInstruction {
        source: parse_signal_source(parts[0].to_string()),
        shift_count: parts[1].parse().unwrap(),
        destination: String::new()
    }
}

fn parse_mov(operation: String) -> MovInstruction {
    MovInstruction {
        source: parse_signal_source(operation.trim().to_string()),
        destination: String::new()
    }
}