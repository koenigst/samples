use std::{fmt::Display, fs::read_to_string, str::Lines};

use crate::cli::*;

pub fn solve<O: Display>(
    exec: &mut CliExecution,
    day: i32, 
    solve_gold: impl FnOnce(&str) -> O, 
    solve_silver: impl FnOnce(&str) -> O) {

    if *exec != day { return; }
    
    let input = &read_to_string(format!("src/day{:02}/input.txt", day)).unwrap();

    solve_part(exec, day, input, ChallengePart::Gold, solve_gold);
    solve_part(exec, day, input, ChallengePart::Silver, solve_silver);

    exec.solved();
}

pub fn solve_lines<O: Display>(
    exec: &mut CliExecution, 
    day: i32,
    solve_gold: impl for<'a> FnOnce(&'a mut Lines<'a>) -> O, 
    solve_silver: impl for<'b> FnOnce(&'b mut Lines<'b>) -> O) {

    fn wrap<O>(solver: impl for<'a> FnOnce(&'a mut Lines<'a>) -> O) -> impl FnOnce(&str) -> O {
        move |s| solver(&mut s.lines())
    }

    solve(exec, day, wrap(solve_gold), wrap(solve_silver));
}

fn solve_part<O: Display>(
    exec: &CliExecution,
    day: i32, 
    input: &str, 
    part: ChallengePart, 
    solve: impl FnOnce(&str) -> O) {
    
    if *exec != part { return; }

    println!("{day:02}{part}: {}", solve(input));
}

#[macro_export]
macro_rules! solve {
    ($args:expr, $day:expr, $mod:ident) => {
        $crate::solver::solve($args, $day, $mod::compute_solution_gold, $mod::compute_solution_silver)
    };
}

#[macro_export]
macro_rules! solve_lines {
    ($args:expr, $day:expr, $mod:ident) => {
        $crate::solver::solve_lines($args, $day, |l| $mod::compute_solution_gold(l), |l| $mod::compute_solution_silver(l))
    };
}
