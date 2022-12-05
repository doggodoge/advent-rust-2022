use advent_2022::day_one::{puzzle_one, puzzle_two};

const DAY_ONE_INPUT: &str = include_str!("day_one/data/input.txt");

fn main() {
    println!("Day one:");
    println!("\tPuzzle one: {} kcals", puzzle_one(DAY_ONE_INPUT));
    println!("\tPuzzle two: {} kcals", puzzle_two(DAY_ONE_INPUT));
}
