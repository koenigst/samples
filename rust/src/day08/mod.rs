use std::collections::HashSet;
use map::Map;
use crate::{utilities::IntoLines, vector::Position};

mod map;

pub fn compute_solution_gold<'a>(input: impl IntoLines<'a>) -> usize
{
    compute_solution(input, |m| m.nodes_gold().collect())
}

pub fn compute_solution_silver<'a>(input: impl IntoLines<'a>) -> usize
{
    compute_solution(input, |m| m.nodes_silver().collect())
}

fn compute_solution<'a>(input: impl IntoLines<'a>, positions: impl FnOnce(&Map) -> HashSet<Position>) -> usize
{
    positions(&input.into_iter().collect()).len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gold_simple() {
        let result = compute_solution_gold([".aa."]);
        assert_eq!(result, 2);
    }

    #[test]
    fn silver_simple() {
        let result = compute_solution_silver([".aa.."]);
        assert_eq!(result, 5);
    }
}
