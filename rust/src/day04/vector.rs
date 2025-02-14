use super::grid::Grid;
use super::location::*;

#[derive(Copy, Clone)]
pub struct Vector(pub Location, pub Direction);

impl Vector {
    pub fn search(&self, grid: &Grid, term: &str) -> bool {
        term
            .chars()
            .enumerate()
            .map(|(i, c)| ((self.1 * i).and_then(|t| self.0 + t), c))
            .all(|(ml, c)| ml.and_then(|l| grid.try_get(l)).is_some_and(|&v| v == c))
    }
}
