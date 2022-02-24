use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() {
    let input = parse_input();

    let (part_1_solution, part_2_solution) = solve(&input);

    println!("Part 1 Solution: {}", part_1_solution);
    println!("Part 2 Solution: {}", part_2_solution);
}

#[derive(Copy, Clone)]
enum InstructionType {
    TurnOn,
    TurnOff,
    Toggle
}

#[derive(Copy, Clone)]
struct Coordinates {
    x: usize,
    y: usize
}

struct Instruction {
    instruction_type: InstructionType,
    start_coordinates: Coordinates,
    end_coordinates: Coordinates
}

impl Instruction {
    fn parse_instruction_line(line: String) -> Instruction {
        let (instruction_type, remaining_line) = Instruction::extract_instruction_type(line);
        let coordinates: Vec<Coordinates> = remaining_line.split(" through ").map(|coordinate_pair| Instruction::parse_coordinate_pair(coordinate_pair)).collect();

        Instruction {
            instruction_type,
            start_coordinates: coordinates[0],
            end_coordinates: coordinates[1]
        }
    }

    fn extract_instruction_type(line: String) -> (InstructionType, String) {
        return if line.starts_with("turn on ") {
            (InstructionType::TurnOn, line.replace("turn on ", ""))
        } else if line.starts_with("turn off ") {
            (InstructionType::TurnOff, line.replace("turn off ", ""))
        } else {
            (InstructionType::Toggle, line.replace("toggle ", ""))
        }
    }

    fn parse_coordinate_pair(coordinate_pair: &str) -> Coordinates {
        let mut coordinates = coordinate_pair.split(",").map(|coordinate| coordinate.parse::<usize>().unwrap());
        let x = coordinates.nth(0).unwrap();
        let y = coordinates.nth(0).unwrap();
        Coordinates { x, y }
    }
}

fn parse_input() -> Vec<Instruction> {
    io::BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|line| line.unwrap())
        .map(|line| Instruction::parse_instruction_line(line))
        .collect()
}

fn solve(input: &Vec<Instruction>) -> (usize, i64) {
    (solve_part_1(input), solve_part_2(input))
}

fn solve_part_1(input: &Vec<Instruction>) -> usize {
    let mut grid = vec![vec![false; 1000]; 1000];

    for instruction in input {
        modify_area_part_1(&mut grid, instruction);
    }

    grid.iter().map(|row| row.iter().filter(|lamp| **lamp).count()).sum()
}

fn modify_area_part_1(grid: &mut Vec<Vec<bool>>, instruction: &Instruction) {
    for x in instruction.start_coordinates.x..=instruction.end_coordinates.x {
        for y in instruction.start_coordinates.y..=instruction.end_coordinates.y {
            grid[x][y] = modify_lamp_part_1(grid[x][y], instruction.instruction_type);
        }
    }
}

fn modify_lamp_part_1(lamp: bool, instruction: InstructionType) -> bool {
    match instruction {
        InstructionType::TurnOn => true,
        InstructionType::TurnOff => false,
        InstructionType::Toggle => !lamp
    }
}


fn solve_part_2(input: &Vec<Instruction>) -> i64 {
    let mut grid = vec![vec![0; 1000]; 1000];

    for instruction in input {
        modify_area_part_2(&mut grid, instruction);
    }

    grid.iter().map(|row| row.iter().sum::<i64>()).sum()
}

fn modify_area_part_2(grid: &mut Vec<Vec<i64>>, instruction: &Instruction) {
    for x in instruction.start_coordinates.x..=instruction.end_coordinates.x {
        for y in instruction.start_coordinates.y..=instruction.end_coordinates.y {
            grid[x][y] = modify_lamp_part_2(grid[x][y], instruction.instruction_type);
        }
    }
}

fn modify_lamp_part_2(lamp: i64, instruction: InstructionType) -> i64 {
    match (lamp, instruction) {
        (lamp, InstructionType::TurnOn) => lamp + 1,
        (0, InstructionType::TurnOff) => 0,
        (lamp, InstructionType::TurnOff) => lamp - 1,
        (lamp, InstructionType::Toggle) => lamp + 2
    }
}