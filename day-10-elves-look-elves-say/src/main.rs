fn main() {
    let input: String = "1113222113".to_string();
    let result_1 = solve(&input, 40);
    let result_2 = solve(&input, 50);
    println!("Part 1: {}", result_1);
    println!("Part 2: {}", result_2);
}

fn solve(input: &String, iterations: i64) -> usize {
    let mut characters = input.chars();
    let mut current_character = characters.next().unwrap();
    let mut current_count = 1;
    let mut groups: Vec<(i64, char)> = Vec::new();

    for character in characters {
        if character == current_character {
            current_count += 1
        } else {
            groups.push((current_count, current_character));
            current_count = 1;
            current_character = character;
        }
    }

    groups.push((current_count, current_character));

    let mut new_string = String::new();

    for (count, character) in groups {
        new_string += &count.to_string();
        new_string.push(character);
    }

    if iterations == 1 {
        new_string.len()
    } else {
        solve(&new_string, iterations - 1)
    }
}
