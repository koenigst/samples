use regex::{Captures, Regex};

const MUL_PATTERN: &str = r"mul\((?<left>\d{1,3}),(?<right>\d{1,3})\)";

pub fn compute_solution_gold(input: &str) -> i32
{
    Regex::new(MUL_PATTERN)
        .unwrap()
        .captures_iter(input)
        .map(compute_capture_gold)
        .sum()
}

pub fn compute_solution_silver(input: &str) -> i32
{
    Regex::new(&format!(r"(?<do>do\(\))|(?<dont>don't\(\))|({})", MUL_PATTERN))
        .unwrap()
        .captures_iter(input)
        .map(compute_capture_silver)
        .fold((true, 0), fold_silver)
        .1
}

fn compute_capture_gold(capture: Captures) -> i32 {
    extract_mul(&capture).into()
}

fn extract_mul(capture: &Captures) -> Mul {
    let left = extract_mul_argument(capture, "left");
    let right = extract_mul_argument(capture, "right");
    Mul(left, right)
}

fn extract_mul_argument(capture: &Captures, name: &str) -> i32 {
    capture.name(name).unwrap().as_str().parse().unwrap()
}

fn compute_capture_silver(capture: Captures) -> Operation {
    use Operation::*;

    if capture.name("do").is_some() {
        Do
    } else if capture.name("dont").is_some() {
        Dont
    } else {
        Mul(extract_mul(&capture))
    }
}

fn fold_silver(accumulator: (bool, i32), element: Operation) -> (bool, i32) {
    use Operation::*;

    fn sum(enabled: bool, accumulator: i32, element: i32) -> i32 {
        if enabled {
            accumulator + element
        } else {
            accumulator
        }
    }

    match element {
        Do => (true, accumulator.1),
        Dont => (false, accumulator.1),
        Mul(m) => (accumulator.0, sum(accumulator.0, accumulator.1, m.into())),
    }
}

struct Mul(i32, i32);

impl Into<i32> for Mul {
    fn into(self) -> i32 {
        self.0 * self.1
    }
}

enum Operation {
    Do,
    Dont,
    Mul(Mul),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gold_simple() {
        let result = compute_solution_gold("mul(2,2)");
        assert_eq!(result, 4);
    }

    #[test]
    fn gold_with_noise() {
        let result = compute_solution_gold("amul(2,2)mul(2,2]mul ( 2 , 2 )");
        assert_eq!(result, 4);
    }

    #[test]
    fn silver_simple() {
        let result = compute_solution_silver("mul(2,2)don't()mul(5,5)do()mul(3,4)");
        assert_eq!(result, 16);
    }
}
