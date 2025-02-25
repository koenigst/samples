use std::sync::LazyLock;
use regex::Regex;
use crate::utilities::IntoLines;

pub fn compute_solution_gold<'a>(input: impl IntoLines<'a>) -> i64
{
    compute_solution(input, apply_gold)
}

pub fn compute_solution_silver<'a>(input: impl IntoLines<'a>) -> i64
{
    compute_solution(input, apply_silver)
}

fn compute_solution<'a>(input: impl IntoLines<'a>, apply: ApplyOperators) -> i64
{
    input
        .into_iter()
        .map(|l| parse(l, apply))
        .filter(is_solvable)
        .map(|e| e.result)
        .sum()
}

type ApplyOperators = fn(&mut Vec<i64>, i64, i64);

fn apply_gold(results: &mut Vec<i64>, left: i64, right: i64) {
    results.push(left * right);
    results.push(left + right);
}

fn apply_silver(results: &mut Vec<i64>, left: i64, right: i64) {
    apply_gold(results, left, right);

    results.push(format!("{left}{right}").parse().unwrap());
}

fn parse(line: &str, apply: ApplyOperators) -> Equation {
    static REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d+").unwrap());

    let mut values = REGEX
        .find_iter(line)
        .filter_map(|m| m.as_str().parse().ok());

    Equation {
        result: values.next().unwrap(),
        operands: values.collect(),
        apply,
    }
}

fn is_solvable(equation: &Equation) -> bool {
    let mut current = vec![equation.operands[0]];
    let mut next = vec![];

    for &operand in &equation.operands[1..] {
        for accumulator in current {
            (equation.apply)(&mut next, accumulator, operand);
        }

        current = next;
        next = vec![];
    }

    current
        .iter()
        .any(|&r| r == equation.result)
}

struct Equation {
    result: i64,
    operands: Vec<i64>,
    apply: ApplyOperators,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gold_simple() {
        let result = compute_solution_gold(["50: 5 10", "75: 4 10 5 5"]);
        assert_eq!(result, 125);
    }

    #[test]
    fn gold_not_solvable() {
        let result = compute_solution_gold(["14: 5 10", "11: 4 10 5"]);
        assert_eq!(result, 0);
    }

    #[test]
    fn silver_simple() {
        let result = compute_solution_silver(["510: 5 10"]);
        assert_eq!(result, 510);
    }
}
