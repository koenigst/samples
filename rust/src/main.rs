use cli::*;

mod cli;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod solver;
mod utilities;
mod vector;

fn main() {
    let exec = &mut CliArgs::load().into();

    solve_lines!(exec, 8, day08);
    solve_lines!(exec, 7, day07);
    solve_lines!(exec, 6, day06);
    solve_lines!(exec, 5, day05);
    solve_lines!(exec, 4, day04);
    solve!(exec, 3, day03);
    solve_lines!(exec, 2, day02);
    solve_lines!(exec, 1, day01);
}
