use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() {
    let input = parse_input();

    let (part_1_solution, part_2_solution) = solve(input);

    println!("Part 1 Solution: {}", part_1_solution);
    println!("Part 2 Solution: {}", part_2_solution);

    println!("qjhvhtzxzqqjkmpb is nice is {}", is_nice_2(&"qjhvhtzxzqqjkmpb".to_string()));
    println!("xxyxx is nice is {}", is_nice_2(&"xxyxx".to_string()));
    println!("uurcxstgmygtbstg is nice is {}", is_nice_2(&"uurcxstgmygtbstg".to_string()));
    println!("ieodomkazucvgmuy is nice is {}", is_nice_2(&"ieodomkazucvgmuy".to_string()));
    println!("abaaaa is nice is {}", is_nice_2(&"abaaaa".to_string()));
}

fn parse_input() -> Vec<String> {
    io::BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|line| line.unwrap())
        .collect()
}

fn solve(input: Vec<String>) -> (usize, i64) {
    let mut nice_strings_1 = 0;
    let mut nice_strings_2 = 0;

    for word in input {
        if is_nice_1(&word) {
            nice_strings_1 += 1;
        }

        if is_nice_2(&word) {
            nice_strings_2 += 1;
        }
    }

    (nice_strings_1, nice_strings_2)
}

fn is_nice_1(word: &String) -> bool {
    if vowel_count(word) < 3 {
        return false;
    }

    let pairs = get_pairs_with_position(word);

    !contains_rule_3_pairs(&pairs) && contains_two_consecutive_letters(&pairs)
}

fn is_nice_2(word: &String) -> bool {
    let pairs = get_pairs_with_position(word);

    contains_a_pair_twice_without_overlap(&pairs) && contains_repeat_letter_with_one_in_between(word)
}

fn vowel_count(word: &String) -> usize {
    word.chars().filter(|character| match character {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false
    }).count()
}

fn get_pairs_with_position(word: &String) -> Vec<(char, char, usize)> {
    let characters: Vec<char> = word.chars().collect();
    let mut result: Vec<(char, char, usize)> = Vec::new();

    for i in 0..characters.len() - 1 {
        result.push((characters[i], characters[i + 1], i));
    }

    result
}

fn contains_rule_3_pairs(pairs: &Vec<(char, char, usize)>) -> bool {
    pairs.iter().any(|(a, b, _)| match (a, b) {
        ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y') => true,
        _ => false
    })
}

fn contains_two_consecutive_letters(pairs: &Vec<(char, char, usize)>) -> bool {
    pairs.iter().any(|(a, b, _)| a == b)
}

fn contains_a_pair_twice_without_overlap(pairs: &Vec<(char, char, usize)>) -> bool {
    for (a, b, i) in pairs.iter() {
        if pairs.iter().filter(|(_, _, pos)| !pair_positions_overlap(*i, *pos)).any(|(c, d, _)| a == c && b == d) {
            return true;
        }
    }

    false
}

fn pair_positions_overlap(pos_1: usize, pos_2: usize) -> bool {
    if pos_1 > 0 && pos_1 - 1 == pos_2 {
        return true;
    }

    pos_1 == pos_2 || pos_1 + 1 == pos_2
}

fn contains_repeat_letter_with_one_in_between(word: &String) -> bool {
    let characters: Vec<char> = word.chars().collect();

    for i in 0..characters.len() - 2 {
        let a = characters[i];
        let b = characters[i + 2];

        if a == b {
            return true;
        }
    }

    false
}