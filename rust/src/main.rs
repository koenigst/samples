use std::fs::read_to_string;
use utilities::read_lines;

mod day01;
mod day02;
mod day03;
mod day04;
mod utilities;

fn main() {
    let day01_input = read_lines("src/day01/input.txt");
    println!("Day 01g: {}", day01::compute_solution_gold(&day01_input));
    println!("Day 01s: {}", day01::compute_solution_silver(&day01_input));

    let day02_input = read_lines("src/day02/input.txt");
    println!("Day 02g: {}", day02::compute_solution_gold(&day02_input));
    println!("Day 02s: {}", day02::compute_solution_silver(&day02_input));

    let day03_input = read_to_string("src/day03/input.txt").unwrap();
    println!("Day 03g: {}", day03::compute_solution_gold(&day03_input));
    println!("Day 03s: {}", day03::compute_solution_silver(&day03_input));

    let day04_input = read_lines("src/day04/input.txt");
    println!("Day 04g: {}", day04::compute_solution_gold(&day04_input));
    println!("Day 04s: {}", day04::compute_solution_silver(&day04_input));
}
