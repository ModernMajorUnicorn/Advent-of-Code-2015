use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::ops::Not;

fn main() {
    println!("Hello, world!");
}

trait LogicalInstruction {
    fn perform(self, wires: &mut HashMap<String, u16>);
    fn set_destination(&mut self, destination: String);
}

struct MovInstruction {
    value: u16,
    destination: String
}

impl LogicalInstruction for MovInstruction {
    fn perform(self, wires: &mut HashMap<String, u16>) {
        wires.insert(self.destination, self.value);
    }

    fn set_destination(&mut self, destination: String) {
        self.destination = destination;
    }
}

struct AndInstruction {
    source_1: String,
    source_2: String,
    destination: String
}

impl LogicalInstruction for AndInstruction {
    fn perform(self, wires: &mut HashMap<String, u16>) {
        let operand_1 = *wires.get(&self.source_1).unwrap();
        let operand_2 = *wires.get(&self.source_2).unwrap();
        wires.insert(self.destination, operand_1 & operand_2);
    }

    fn set_destination(&mut self, destination: String) {
        self.destination = destination;
    }
}

struct OrInstruction {
    source_1: String,
    source_2: String,
    destination: String
}

impl LogicalInstruction for OrInstruction {
    fn perform(self, wires: &mut HashMap<String, u16>) {
        let operand_1 = *wires.get(&self.source_1).unwrap();
        let operand_2 = *wires.get(&self.source_2).unwrap();
        wires.insert(self.destination, operand_1 | operand_2);
    }

    fn set_destination(&mut self, destination: String) {
        self.destination = destination;
    }
}

struct NotInstruction {
    source: String,
    destination: String
}

impl LogicalInstruction for NotInstruction {
    fn perform(self, wires: &mut HashMap<String, u16>) {
        let operand = *wires.get(&self.source).unwrap();
        wires.insert(self.destination, !operand);
    }

    fn set_destination(&mut self, destination: String) {
        self.destination = destination;
    }
}

struct LShiftInstruction {
    source: String,
    shift_count: i64,
    destination: String
}

impl LogicalInstruction for LShiftInstruction {
    fn perform(self, wires: &mut HashMap<String, u16>) {
        let operand = *wires.get(&self.source).unwrap();
        wires.insert(self.destination, operand << self.shift_count);
    }

    fn set_destination(&mut self, destination: String) {
        self.destination = destination;
    }
}

struct RShiftInstruction {
    source: String,
    shift_count: i64,
    destination: String
}

impl LogicalInstruction for RShiftInstruction {
    fn perform(self, wires: &mut HashMap<String, u16>) {
        let operand = *wires.get(&self.source).unwrap();
        wires.insert(self.destination, operand >> self.shift_count);
    }

    fn set_destination(&mut self, destination: String) {
        self.destination = destination;
    }
}

fn parse_input(path: &str) -> Vec<dyn LogicalInstruction> {
    io::BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|line| line.unwrap())
        .map(|line| parse_line(line))
        .collect()
}

fn parse_line(line: String) -> Box<dyn LogicalInstruction> {
    let (source, destination) = separate_source_and_destination(line);
    let mut instruction = parse_source(source);
    instruction.set_destination(destination);
    instruction
}

fn separate_source_and_destination(line: String) -> (String, String) {
    let parts: Vec<String> = line.split(" -> ").collect();
    (parts[0].to_string(), parts[1].to_string())
}

fn parse_source(source: String) -> Box<dyn LogicalInstruction> {
    if source.contains("AND") {
        Box::new(parse_and(source))
    } else if source.contains("OR") {
        Box::new(parse_or(source))
    } else if source.contains("NOT") {
        Box::new(parse_not(source))
    } else if source.contains("LSHIFT") {
        Box::new(parse_lshift(source))
    } else if source.contains("RSHIFT") {
        Box::new(parse_rshift(source))
    } else {
        Box::new(parse_mov(source))
    }
}

fn parse_and(source: String) -> AndInstruction {
    let parts: Vec<String> = source.split(" AND ").collect();

    AndInstruction {
        source_1: parts[0].to_string(),
        source_2: parts[1].to_string(),
        destination: String::new()
    }
}

fn parse_or(source: String) -> OrInstruction {
    let parts: Vec<String> = source.split(" OR ").collect();

    OrInstruction {
        source_1: parts[0].to_string(),
        source_2: parts[1].to_string(),
        destination: String::new()
    }
}

fn parse_not(source: String) -> NotInstruction {
    NotInstruction {
        source: source.replace("NOT ", ""),
        destination: String::new()
    }
}

fn parse_lshift(source: String) -> LShiftInstruction {
    let parts: Vec<String> = source.split(" LSHIFT ").collect();

    LShiftInstruction {
        source: parts[0].to_string(),
        shift_count: parts[1].parse().unwrap(),
        destination: String::new()
    }
}

fn parse_rshift(source: String) -> RShiftInstruction {
    let parts: Vec<String> = source.split(" RSHIFT ").collect();

    RShiftInstruction {
        source: parts[0].to_string(),
        shift_count: parts[1].parse().unwrap(),
        destination: String::new()
    }
}

fn parse_mov(source: String) -> MovInstruction {
    MovInstruction {
        value: source.trim().parse().unwrap(),
        destination: String::new()
    }
}