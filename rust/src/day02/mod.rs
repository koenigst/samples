pub fn compute_solution_gold<'a, T>(input: T) -> i32
    where T : 'a, T : IntoIterator<Item = &'a str>
{
    compute_solution(input, is_safe_gold)
}

pub fn compute_solution_silver<'a, T>(input: T) -> i32
    where T : 'a, T : IntoIterator<Item = &'a str>
{
    compute_solution(input, is_safe_silver)
}

fn compute_solution<'a, T, P>(input: T, is_safe: P) -> i32
where 
    T : 'a + IntoIterator<Item = &'a str>,
    P : Fn(&[i32]) -> bool,
{
    parse(input)
        .filter(|v| is_safe(&v.as_slice()))
        .count()
        .try_into()
        .unwrap()
}

fn parse<'a, T>(input: T) -> impl Iterator<Item = Vec<i32>>
    where T : 'a, T : IntoIterator<Item = &'a str>
{
    input
        .into_iter()
        .map(parse_line)
}

fn parse_line(input: &str) -> Vec<i32> {
    input
        .split_whitespace()
        .filter_map(try_parse_value)
        .collect()
}

fn try_parse_value(input: &str) -> Option<i32> {
    str::parse::<i32>(input).ok()
}

fn is_safe_gold(input: &[i32]) -> bool {
    let mut last = &input[0];
    let signum = (input[1] - last).signum();
    let values = &input[1..];
    for value in values {
        let difference = value - last;
        last = value;

        if  difference.signum() != signum ||
            difference == 0 ||
            difference.abs() > 3 {
            return false
        }
    }

    true
}

fn is_safe_silver(input: &[i32]) -> bool {
    for i in 0..input.len() {
        let current : Vec<_> = input
            .iter()
            .enumerate()
            .filter_map(|t| if t.0 == i { None } else { Some(*t.1) })
            .collect();

        if is_safe_gold(&current) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gold_simple() {
        let result = compute_solution_gold(["1 2 3", "3 2 1"]);
        assert_eq!(result, 2);
    }

    #[test]
    fn gold_order() {
        let result = compute_solution_gold(["1 3 2", "2 1 3"]);
        assert_eq!(result, 0);
    }

    #[test]
    fn gold_distance() {
        let result = compute_solution_gold(["1 1", "1 5", "5 1"]);
        assert_eq!(result, 0);
    }

    #[test]
    fn silver_all() {
        let result = compute_solution_silver(["1 2 3", "1 2 5 3", "17 2 1", "1 6 1", "1 2 17"]);
        assert_eq!(result, 4);
    }
}
