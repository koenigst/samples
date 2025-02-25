use std::collections::HashSet;
use parsing::Parsed;
use processing::*;
use crate::{utilities::IntoLines, vector::Position};

mod direction;
mod map;
mod processing;
mod parsing;

pub fn compute_solution_gold<'a>(input: impl IntoLines<'a>) -> usize
{
    compute_solution::<GuardPositions>(input)
}

pub fn compute_solution_silver<'a>(input: impl IntoLines<'a>) -> usize
{
    compute_solution::<Obstructions>(input)
}

fn compute_solution<'a, T>(input: impl IntoLines<'a>) -> usize
where T: Iterator<Item = Position> + From<Parsed>
{
    let parsed : Parsed = input.into();
    let iter : T = parsed.into();
    let positions : HashSet<_> = iter.collect();
    positions.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gold_simple() {
        let result = compute_solution_gold([">"]);
        assert_eq!(result, 1);
    }

    #[test]
    fn gold_only_unique() {
        let result = compute_solution_gold([">.#", ".##"]);
        assert_eq!(result, 2);
    }

    #[test]
    fn gold_with_rotation() {
        let result = compute_solution_gold([">.#", "..."]);
        assert_eq!(result, 3);
    }

    #[test]
    fn silver_simple() {
        let result = compute_solution_silver([".#..", ">..#", "#...", ".#.."]);
        assert_eq!(result, 2);
    }
}
