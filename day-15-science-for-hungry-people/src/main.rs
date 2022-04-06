use std::fs::File;
use std::io;
use std::io::BufRead;
use itertools::Itertools;

#[derive(Copy, Clone)]
struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64
}

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let test_result = run_part_1("test_input.txt");
    println!("Result for test input part 1: {}", test_result);

    let puzzle_result = run_part_1("input.txt");
    println!("Result for puzzle input part 1: {}", puzzle_result);
}

fn part_2() {
    let test_result = run_part_2("test_input.txt");
    println!("Result for test input part 2: {}", test_result);

    let puzzle_result = run_part_2("input.txt");
    println!("Result for puzzle input part 2: {}", puzzle_result);
}

fn run_part_1(path: &str) -> i64 {
    let puzzle_input = load_puzzle_input(path);
    let all_recipes = puzzle_input.iter().combinations_with_replacement(100);

    all_recipes
        .map(|recipe| calculate_recipe_score(&recipe))
        .max()
        .unwrap()
}

fn run_part_2(path: &str) -> i64 {
    let puzzle_input = load_puzzle_input(path);
    let all_recipes = puzzle_input.iter().combinations_with_replacement(100);

    all_recipes
        .filter(|recipe| calculate_recipe_calories(recipe) == 500)
        .map(|recipe| calculate_recipe_score(&recipe))
        .max()
        .unwrap()
}

fn load_puzzle_input(path: &str) -> Vec<Ingredient> {
    io::BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let line_tokens: Vec<&str> = line.split(' ').collect();
            let capacity: i64 = line_tokens[2].replace(",", "").parse().unwrap();
            let durability: i64 = line_tokens[4].replace(",", "").parse().unwrap();
            let flavor: i64 = line_tokens[6].replace(",", "").parse().unwrap();
            let texture: i64 = line_tokens[8].replace(",", "").parse().unwrap();
            let calories: i64 = line_tokens[10].parse().unwrap();
            Ingredient { capacity, durability, flavor, texture, calories }
        }).collect()
}

fn calculate_recipe_score(recipe: &Vec<&Ingredient>) -> i64 {
    let total_capacity: i64 = recipe.iter().map(|ingredient| ingredient.capacity).sum();
    let total_durability: i64 = recipe.iter().map(|ingredient| ingredient.durability).sum();
    let total_flavor: i64 = recipe.iter().map(|ingredient| ingredient.flavor).sum();
    let total_texture: i64 = recipe.iter().map(|ingredient| ingredient.texture).sum();

    if total_capacity < 0 || total_durability < 0 || total_flavor < 0 || total_texture < 0 {
        0
    } else {
        total_capacity * total_durability * total_flavor * total_texture
    }
}

fn calculate_recipe_calories(recipe: &Vec<&Ingredient>) -> i64 {
    recipe.iter().map(|ingredient| ingredient.calories).sum()
}