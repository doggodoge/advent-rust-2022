use advent_2022::day_one;
use advent_2022::day_two;

const DAY_ONE_INPUT: &str = include_str!("day_one/data/input.txt");
const DAY_TWO_INPUT: &str = include_str!("day_two/data/input.txt");

fn main() {
    println!("Day one:");
    println!("\tPuzzle one: {} kcals", day_one::puzzle_one(DAY_ONE_INPUT));
    println!("\tPuzzle two: {} kcals", day_one::puzzle_two(DAY_ONE_INPUT));

    println!();

    println!("Day two:");
    println!("\tPuzzle one: {} score", day_two::puzzle_one(DAY_TWO_INPUT));
    println!("\tPuzzle two: {} score", day_two::puzzle_two(DAY_TWO_INPUT));
}
