use std::cmp::Ordering;
use crate::utilities::IntoLines;

pub fn compute_solution_gold<'a>(input: impl IntoLines<'a>) -> i32
{
    compute_solution(
        input, 
        |u, o| is_correct_order(u, o), 
        |u, _| get_middle(u))
}

pub fn compute_solution_silver<'a>(input: impl IntoLines<'a>) -> i32
{
    compute_solution(
        input, 
        |u, o| !is_correct_order(u, o), 
        |u, o| get_middle(&correct_order(u, o)))
}

fn compute_solution<'a>(
    input: impl IntoLines<'a>,
    mut filter: impl FnMut(&[i32], &InstructionOrdering) -> bool,
    mut map: impl FnMut(&[i32], &InstructionOrdering) -> i32,
) -> i32
{
    let (instructions, updates) = parse(input);
    let ordering = InstructionOrdering(&instructions);

    updates
        .iter()
        .filter(|u| filter(u, &ordering))
        .map(|u| map(u, &ordering))
        .sum()
}

fn parse<'a>(input: impl IntoLines<'a>) -> (Vec<OrderingInstruction>, Vec<Vec<i32>>) {
    let mut iter = input.into_iter();

    let instructions = iter
        .by_ref()
        .take_while(|s| !s.is_empty())
        .map(parse_instruction)
        .collect();

    let updates = iter
        .by_ref()
        .map(parse_update)
        .collect();

    (instructions, updates)
}

fn parse_instruction(line: &str) -> OrderingInstruction {
    let pages : Vec<i32> = line
        .split("|")
        .take(2)
        .map(|s| s.parse().unwrap())
        .collect();

    OrderingInstruction { before: pages[0], after: pages[1], }
}

fn parse_update(line: &str) -> Vec<i32> {
    line
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect()
}

fn is_correct_order(updates: &[i32], ordering: &InstructionOrdering) -> bool {
    updates.is_sorted_by(|&l, &r| ordering.is_correct_order(l, r))
}

fn correct_order(updates: &[i32], ordering: &InstructionOrdering) -> Vec<i32> {
    let mut ordered : Vec<_> = updates.to_vec();
    ordered.sort_by(|l, r| ordering.cmp(*l, *r));
    ordered
}

fn get_middle(updates: &[i32]) -> i32 {
    updates[updates.len() / 2]
}

#[derive(Eq, PartialEq, Debug)]
struct OrderingInstruction {
    before: i32,
    after: i32,
}

struct InstructionOrdering<'a>(&'a [OrderingInstruction]);

impl<'a> InstructionOrdering<'a> {
    fn is_correct_order(&self, left: i32, right: i32) -> bool {
        !self.0.contains(&OrderingInstruction { before: right, after: left, })
    }

    fn cmp(&self, left: i32, right: i32) -> Ordering {
        for instruction in self.0 {
            if instruction.after == left && instruction.before == right {
                return Ordering::Greater;
            }

            if instruction.before == left && instruction.after == right {
                return Ordering::Less;
            }
        }

        Ordering::Equal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple() {
        let (instructions, updates) = parse(["1|2", "", "1,2,3"]);
        assert_eq!(instructions, [ OrderingInstruction { before: 1, after: 2, }, ]);
        assert_eq!(updates, [[1, 2, 3]]);
    }

    #[test]
    fn gold_simple() {
        let result = compute_solution_gold(["1|2", "", "1,2,3"]);
        assert_eq!(result, 2);
    }

    #[test]
    fn gold_incorrect() {
        let result = compute_solution_gold(["1|2", "2|3", "", "4,5,6", "3,2,1"]);
        assert_eq!(result, 5);
    }

    #[test]
    fn silver_simple() {
        let result = compute_solution_silver(["1|2", "2|3", "1|3", "", "3,2,1"]);
        assert_eq!(result, 2);
    }
}
