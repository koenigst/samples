use crate::vector::*;

pub struct Grid {
    content: Vec<Vec<char>>
}

impl Grid {    
    pub fn try_get(&self, position: Position) -> Option<&char> {
        self.content.get(position.0).and_then(|l| l.get(position.1))
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Grid, Position)> {
        self.content
            .iter()
            .enumerate()
            .flat_map(|(li, l)| l.iter().enumerate().map(move |(ci, _)| Position(li, ci)))
            .map(move |l| (self, l))
    }

    pub fn search(&self, vector: &Vector, term: &str) -> bool {
        term
            .chars()
            .enumerate()
            .map(|(i, c)| (i.try_into().and_then(|i| vector.0 + (vector.1 * i)).ok(), c))
            .all(|(ml, c)| ml.and_then(|l| self.try_get(l)).is_some_and(|&v| v == c))
    }
}

impl<'a> FromIterator<&'a str> for Grid
{
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        Grid {
            content: iter
                .into_iter()
                .map(|l| l.chars().collect())
                .collect(),
        }
    }
}
