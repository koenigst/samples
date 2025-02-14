use grid::Grid;
use location::*;
use vector::Vector;

mod grid;
mod location;
mod vector;

pub fn compute_solution_gold<'a, T>(input: T) -> usize
    where T : 'a + IntoIterator<Item = &'a str>
{
    search_input(input, search_gold).iter().sum()
}

pub fn compute_solution_silver<'a, T>(input: T) -> usize
    where T : 'a + IntoIterator<Item = &'a str>
{
    search_input(input, search_silver).iter().filter(|&&b| b).count()
}

fn search_input<'a, O>(
    input: impl 'a + IntoIterator<Item = &'a str>,
    search: impl Fn(&Grid, Location) -> O) -> Vec<O>
{
    Grid::from_iter(input)
        .iter()
        .map(|(g, l)| search(g, l))
        .collect()
}

static DIRECTIONS_GOLD: &[Direction] = &[
    Direction(1, 0),
    Direction(-1, 0),
    Direction(0, 1),
    Direction(0, -1),
    Direction(1, 1),
    Direction(-1, -1),
    Direction(-1, 1),
    Direction(1, -1),
];

static DIRECTIONS_SILVER: &[Direction] = &[
    Direction(1, 1),
    Direction(-1, -1),
    Direction(-1, 1),
    Direction(1, -1),
];

fn search_gold(grid: &Grid, start: Location) -> usize {
    DIRECTIONS_GOLD
        .iter()
        .map(|&d| Vector(start, d))
        .filter(|v| v.search(grid, "XMAS"))
        .count()
}

fn search_silver(grid: &Grid, start: Location) -> bool {
    DIRECTIONS_SILVER
        .iter()
        .filter_map(|&d| try_get_vector_silver(start, d))
        .filter(|v| v.search(grid, "MAS"))
        .count()
        > 1
}

fn try_get_vector_silver(start: Location, direction: Direction) -> Option<Vector> {
    let start = (start + direction.into())?;
    let direction = direction.invert();
    Some(Vector(start, direction))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gold_straight() {
        let result = compute_solution_gold([
            "XMAS",
            "M..A",
            "A..M",
            "SAMX"]);
        assert_eq!(result, 4);
    }

    #[test]
    fn gold_diagonal() {
        let result = compute_solution_gold([
            "X..SS..X",
            ".MA..AM.",
            ".MA..AM.",
            "X..SS..X"]);
        assert_eq!(result, 4);
    }

    #[test]
    fn silver_horizontal() {
        let result = compute_solution_silver([
            "MSSM",
            ".AA.",
            "MSSM",
            ]);
        assert_eq!(result, 2);
    }

    #[test]
    fn silver_vertical() {
        let result = compute_solution_silver([
            "M.M",
            "SAS",
            "SAS",
            "M.M",
            ]);
        assert_eq!(result, 2);
    }
}
