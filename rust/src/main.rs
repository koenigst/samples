use utilities::read_lines;

mod day01;
mod day02;
mod utilities;

fn main() {
    let day01_input = read_lines("src/day01/input.txt");
    println!("Day 01g: {}", day01::compute_solution_gold(&day01_input));
    println!("Day 01s: {}", day01::compute_solution_silver(&day01_input));

    let day02_input = read_lines("src/day02/input.txt");
    println!("Day 02g: {}", day02::compute_solution_gold(&day02_input));
    println!("Day 02s: {}", day02::compute_solution_silver(&day02_input));
}
