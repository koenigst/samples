use utilities::read_lines;

mod day01;
mod utilities;

fn main() {
    let day01_input = read_lines("src/day01/input.txt");
    println!("Day 01g: {}", day01::compute_solution_gold(&day01_input));
    println!("Day 01s: {}", day01::compute_solution_silver(&day01_input));
}
