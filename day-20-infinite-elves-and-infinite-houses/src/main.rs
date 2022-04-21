const PUZZLE_INPUT: u64 = 34000000;

fn main() {
    let test = run_test();
    println!("{:?}", test);

    let part_1_solution = run_part_1();
    println!("{}", part_1_solution);

    let part_2_solution = run_part_2();
    println!("{}", part_2_solution);
}

fn run_test() -> Vec<u64> {
    calculate_houses_up_to_part_1(10)
}

fn run_part_1() -> usize {
    let houses = calculate_houses_up_to_part_1((PUZZLE_INPUT / 10) as usize);

    for i in 0..houses.len() {
        if houses[i] >= PUZZLE_INPUT {
            return i;
        }
    }

    PUZZLE_INPUT as usize
}

fn run_part_2() -> usize {
    let houses = calculate_houses_up_to_part_2((PUZZLE_INPUT / 11) as usize);

    for i in 0..houses.len() {
        if houses[i] >= PUZZLE_INPUT {
            return i;
        }
    }

    PUZZLE_INPUT as usize
}

fn calculate_houses_up_to_part_1(max_house: usize) -> Vec<u64> {
    let mut houses: Vec<u64> = Vec::new();

    for _ in 0..=max_house {
        houses.push(0);
    }

    for elf in 1..=max_house {
        let presents = (elf * 10) as u64;

        for house in (elf..=max_house).step_by(elf) {
            houses[house] += presents;
        }
    }

    houses
}

fn calculate_houses_up_to_part_2(max_house: usize) -> Vec<u64> {
    let mut houses: Vec<u64> = Vec::new();

    for _ in 0..=max_house {
        houses.push(0);
    }

    for elf in 1..=max_house {
        let presents = (elf * 11) as u64;

        for house in (elf..=max_house).step_by(elf).take(50) {
            houses[house] += presents;
        }
    }

    houses
}