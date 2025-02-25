use std::sync::LazyLock;
use regex::Regex;
use crate::utilities::IntoLines;

pub fn compute_solution_gold<'a>(input: impl IntoLines<'a>) -> usize
{
    0
}

pub fn compute_solution_silver<'a>(input: impl IntoLines<'a>) -> usize
{
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gold_simple() {
        let result = compute_solution_gold([]);
        assert_eq!(result, 1);
    }

    #[test]
    fn silver_simple() {
        let result = compute_solution_silver([]);
        assert_eq!(result, 1);
    }
}
