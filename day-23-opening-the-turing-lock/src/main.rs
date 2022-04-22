extern crate core;

use std::fs::File;
use std::io;
use std::io::BufRead;
use itertools::Itertools;
use crate::Register::{A, B};

fn main() {
    let program = load_puzzle_input();

    let mut cpu1 = Cpu { a: 0, b: 0, ip: 0 };
    run_program(&mut cpu1, &program);
    println!("Part 1 result: {}", cpu1.b);

    let mut cpu2 = Cpu { a: 1, b: 0, ip: 0 };
    run_program(&mut cpu2, &program);
    println!("Part 2 result: {}", cpu2.b);

}

fn run_program(cpu: &mut Cpu, program: &Vec<Box<dyn Instruction>>) {
    while cpu.ip < program.len() {
        let instruction = &program[cpu.ip];
        instruction.execute(cpu);
    }
}

fn load_puzzle_input() -> Vec<Box<dyn Instruction>> {
    let lines = io::BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|line| line.unwrap());

    let mut program = Vec::new();

    for line in lines {
        let clean_line = line.replace(",", "");
        let line_parts = clean_line.split(" ").collect_vec();
        let instruction = parse_instruction(line_parts);
        program.push(instruction);
    }

    program
}

fn parse_instruction(line: Vec<&str>) -> Box<dyn Instruction> {
    match line[..] {
        ["hlf", r] => Box::new(Hlf { register: parse_register(r) }),
        ["tpl", r] => Box::new(Tpl { register: parse_register(r) }),
        ["inc", r] => Box::new(Inc { register: parse_register(r) }),
        ["jmp", offset] => Box::new(Jmp { offset: parse_offset(offset) }),
        ["jie", r, offset] => Box::new(Jie { register: parse_register(r), offset: parse_offset(offset) }),
        ["jio", r, offset] => Box::new(Jio { register: parse_register(r), offset: parse_offset(offset) }),
        _ => panic!("Unknown Instruction"),
    }
}

fn parse_register(register_name: &str) -> Register {
    if register_name == "a" {
        A
    } else if register_name == "b" {
        B
    } else {
        panic!("Unknown Register");
    }
}

fn parse_offset(value: &str) -> Offset {
    let offset_value: i64 = value.parse().unwrap();

    if offset_value < 0 {
        Offset::Negative((offset_value * -1) as usize)
    } else {
        Offset::Positive(offset_value as usize)
    }
}

struct Cpu {
    a: u64,
    b: u64,
    ip: usize,
}

enum Offset {
    Positive(usize),
    Negative(usize),
}

impl Offset {
    fn calculate(&self, base_value: usize) -> usize {
        match self {
            Offset::Positive(offset) => base_value + *offset,
            Offset::Negative(offset) => base_value - *offset,
        }
    }
}

trait Instruction {
    fn execute(&self, cpu: &mut Cpu);
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Register {
    A,
    B
}

struct Hlf {
    register: Register,
}

impl Instruction for Hlf {
    fn execute(&self, cpu: &mut Cpu) {
        if self.register == A {
            cpu.a /= 2;
        } else {
            cpu.b /= 2;
        }

        cpu.ip += 1;
    }
}

struct Tpl {
    register: Register,
}

impl Instruction for Tpl {
    fn execute(&self, cpu: &mut Cpu) {
        if self.register == A {
            cpu.a *= 3;
        } else {
            cpu.b *= 3;
        }

        cpu.ip += 1;
    }
}

struct Inc {
    register: Register,
}

impl Instruction for Inc {
    fn execute(&self, cpu: &mut Cpu) {
        if self.register == A {
            cpu.a += 1;
        } else {
            cpu.b += 1;
        }

        cpu.ip += 1;
    }
}

struct Jmp {
    offset: Offset,
}

impl Instruction for Jmp {
    fn execute(&self, cpu: &mut Cpu) {
        cpu.ip = self.offset.calculate(cpu.ip);
    }
}

struct Jie {
    register: Register,
    offset: Offset,
}

impl Instruction for Jie {
    fn execute(&self, cpu: &mut Cpu) {
        if self.register == A && cpu.a % 2 == 0 {
            cpu.ip = self.offset.calculate(cpu.ip);
        } else if self.register == B && cpu.b % 2 == 0 {
            cpu.ip = self.offset.calculate(cpu.ip);
        } else {
            cpu.ip += 1;
        }
    }
}

struct Jio {
    register: Register,
    offset: Offset,
}

impl Instruction for Jio {
    fn execute(&self, cpu: &mut Cpu) {
        if self.register == A && cpu.a == 1 {
            cpu.ip = self.offset.calculate(cpu.ip);
        } else if self.register == B && cpu.b == 1 {
            cpu.ip = self.offset.calculate(cpu.ip);
        } else {
            cpu.ip += 1;
        }
    }
}