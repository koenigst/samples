use std::iter::zip;
use crate::utilities::group_join;

pub fn compute_solution_gold<'a, T>(input: T) -> i32
    where T : 'a, T : IntoIterator<Item = &'a str>
{
    let (mut left, mut right) = parse(input);

    left.sort();
    right.sort();

    zip(left, right)
        .map(|t| (t.0 - t.1).abs())
        .sum()
}

pub fn compute_solution_silver<'a, T>(input: T) -> i32
    where T : 'a, T : IntoIterator<Item = &'a str>
{
    let (left, right) = parse(input);

    group_join(left, right, |i| *i, |i| *i, compute_line_silver)
        .sum()
}

fn compute_line_silver(left: &i32, rights: &[i32]) -> i32 {
    left * i32::try_from(rights.len()).unwrap()
}

fn parse<'a, T>(input: T) -> (Vec<i32>, Vec<i32>)
    where T : 'a, T : IntoIterator<Item = &'a str>
{
    input
        .into_iter()
        .map(parse_line)
        .unzip()
}

fn parse_line(input: &str) -> (i32, i32) {
    let values: Vec<_> = input
        .split_whitespace()
        .filter_map(try_parse_value)
        .take(2)
        .collect();

    (values[0], values[1])
}

fn try_parse_value(input: &str) -> Option<i32> {
    str::parse::<i32>(input).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gold_simple() {
        let result = compute_solution_gold(["1 2"]);
        assert_eq!(result, 1);
    }

    #[test]
    fn gold_whitespace() {
        let result = compute_solution_gold(["1    2"]);
        assert_eq!(result, 1);
    }

    #[test]
    fn gold_unordered() {
        let result = compute_solution_gold(["5 2", "3 4", "4 6"]);
        assert_eq!(result, 2);
    }
}
