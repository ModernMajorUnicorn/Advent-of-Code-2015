const SECRET_KEY: &str = "bgvyzdsv";

fn main() {
    let (part_1_solution, part_2_solution) = solve(SECRET_KEY);

    println!("Part 1 Solution: {}", part_1_solution);
    println!("Part 2 Solution: {}", part_2_solution);
}

fn solve(secret_key: &str) -> (i64, i64) {
    (solve_for_prefix(secret_key, "00000"), solve_for_prefix(secret_key, "000000"))
}

fn solve_for_prefix(secret_key: &str, prefix: &str) -> i64 {
    let mut counter = 0;

    loop {
        let candidate = format!("{}{}", secret_key, counter);
        let hash = format!("{:x}", md5::compute(candidate));

        if hash.starts_with(prefix) {
            return counter;
        }

        counter += 1;
    }
}