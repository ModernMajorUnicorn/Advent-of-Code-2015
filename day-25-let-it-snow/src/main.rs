fn main() {
    let (row, column) = get_puzzle_input();
    let mut current_code = 20151125;
    let mut current_row = 1;
    let mut current_column = 1;

    loop {
        if current_row == row && current_column == column {
            println!("Part 1 solution: {}", current_code);
            return;
        }

        current_code = calculate_next_code(current_code);
        (current_row, current_column) = calculate_next_position(current_row, current_column);
    }
}

fn get_puzzle_input() -> (i64, i64) {
    (3010, 3019)
}

fn calculate_next_code(previous_code: i64) -> i64 {
    (previous_code * 252533) % 33554393
}

fn calculate_next_position(current_row: i64, current_column: i64) -> (i64, i64) {
    let next_row;
    let next_column;

    if current_row == 1 {
        next_row = current_column + 1;
        next_column = 1;
    } else {
        next_row = current_row - 1;
        next_column = current_column + 1;
    }

    (next_row, next_column)
}