use std::fs::File;
use std::io;
use std::io::BufRead;

struct LightGrid {
    size: usize,
    grid: Vec<Vec<bool>>
}

impl LightGrid {
    fn new(size: usize) -> LightGrid {
        let new_grid = (0..size).map(|_| (0..size).map(|_| false).collect()).collect();

        LightGrid {
            size,
            grid: new_grid
        }
    }

    fn copy(&self) -> LightGrid {
        LightGrid {
            size: self.size,
            grid: self.grid.iter().map(|row| row.iter().map(|light| *light).collect()).collect()
        }
    }

    fn coordinates_in_grid(&self, x: usize, y: usize) -> bool {
        x < self.size && y < self.size
    }

    fn set_lamp(&mut self, x: usize, y: usize, value: bool) {
        if self.coordinates_in_grid(x, y) {
            self.grid[y][x] = value;
        }
    }

    fn get_lamp(&self, x: usize, y: usize) -> bool {
        if self.coordinates_in_grid(x, y) {
            self.grid[y][x]
        } else {
            false
        }
    }

    fn get_surroundings(&self, x: usize, y: usize) -> [bool; 8] {
        [
            self.get_lamp(x - 1, y - 1),
            self.get_lamp(x - 1, y),
            self.get_lamp(x - 1, y + 1),

            self.get_lamp(x, y - 1),
            self.get_lamp(x, y + 1),

            self.get_lamp(x + 1, y - 1),
            self.get_lamp(x + 1, y),
            self.get_lamp(x + 1, y + 1),
        ]
    }

    fn calculate_next_grid(&self) -> LightGrid {
        let mut next_grid = LightGrid::new(self.size);

        for y in 0..self.size {
            for x in 0..self.size {
                let surrounding_count =
                    self.get_surroundings(x, y)
                        .iter()
                        .filter(|light| **light)
                        .count();

                let new_value = match (self.grid[y][x], surrounding_count) {
                    (true, 2) => true,
                    (true, 3) => true,
                    (true, _) => false,
                    (false, 3) => true,
                    (false, _) => false,
                };

                next_grid.set_lamp(x, y, new_value);
            }
        }

        next_grid
    }
}

fn main() {
    let test_input = load_puzzle_input("test_input.txt", 6);
    let puzzle_input = load_puzzle_input("input.txt", 100);

    let test_result_part_1 = run_part_1(&test_input, 4);
    println!("Result for Test Part 1: {}", test_result_part_1);

    let result_part_1 = run_part_1(&puzzle_input, 100);
    println!("Result for Puzzle Part 1: {}", result_part_1);

    let test_result_part_2 = run_part_2(&test_input, 5);
    println!("Result for Test Part 2: {}", test_result_part_2);

    let result_part_2 = run_part_2(&puzzle_input, 100);
    println!("Result for Puzzle Part 2: {}", result_part_2);
}

fn load_puzzle_input(path: &str, expected_size: usize) -> LightGrid {
    let lines: Vec<String> = io::BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let mut puzzle_input = LightGrid::new(expected_size);

    for (y, line) in lines.iter().enumerate() {
        for (x, character) in line.chars().enumerate() {
            puzzle_input.set_lamp(x, y, character == '#');
        }
    }

    puzzle_input
}

fn run_part_1(puzzle_input: &LightGrid, steps: i64) -> usize {
    let mut current_grid = puzzle_input.copy();

    for _ in 0..steps {
        current_grid = current_grid.calculate_next_grid();
    }

    current_grid.grid.iter().map(|row| row.iter().filter(|light| **light).count()).sum()
}

fn run_part_2(puzzle_input: &LightGrid, steps: i64) -> usize {
    let mut current_grid = puzzle_input.copy();
    current_grid.set_lamp(0, 0, true);
    current_grid.set_lamp(0, current_grid.size - 1, true);
    current_grid.set_lamp(current_grid.size - 1, 0, true);
    current_grid.set_lamp(current_grid.size - 1, current_grid.size - 1, true);

    for _ in 0..steps {
        current_grid = current_grid.calculate_next_grid();
        current_grid.set_lamp(0, 0, true);
        current_grid.set_lamp(0, current_grid.size - 1, true);
        current_grid.set_lamp(current_grid.size - 1, 0, true);
        current_grid.set_lamp(current_grid.size - 1, current_grid.size - 1, true);
    }

    current_grid.grid.iter().map(|row| row.iter().filter(|light| **light).count()).sum()
}