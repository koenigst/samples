use crate::vector::Position;

use super::direction::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Guard(pub Position, pub CardinalDirection);

impl Guard {
    pub fn try_move(&self, map: & impl Maplike) -> Option<Guard> {
        let mut new_direction = self.1;
        let mut new_position = (self.0 + new_direction.into()).ok()?;

        if map.get(new_position)? {
            new_position = self.0;
            new_direction = new_direction.turn_right();
        }

        Some(Guard(new_position, new_direction))
    }
}

pub trait Maplike {
    fn get(&self, position: Position) -> Option<bool>;
    fn next_position(&self, current: Position) -> Option<Position>;
}

pub struct Map(pub Vec<Vec<bool>>);

impl Map {
    pub fn with_obstruction<'a>(&'a self, position: Position) -> Option<ObstructedMap<'a>> {
        match self.get(position) {
            Some(false) => Some(ObstructedMap(self, position)),
            _ => None,
        }
    }
}

impl Maplike for Map {
    fn get(&self, position: Position) -> Option<bool> {
        self.0.get(position.0).and_then(|l| l.get(position.1)).copied()
    }

    fn next_position(&self, current: Position) -> Option<Position> {
        fn next_index<T>(slice: &[T], index: usize) -> Option<usize> {
            let next = index + 1;
            if next < slice.len() {
                Some(next)
            } else {
                None
            }
        }

        self.0
            .get(current.0)
            .and_then(|l| next_index(&l, current.1)
                .map(|i| Position(current.0, i)))
            .or_else(|| next_index(&self.0, current.0)
                .map(|i| Position(i, 0)))
    }
}

pub struct ObstructedMap<'a>(&'a Map, pub Position);

impl<'a> Maplike for ObstructedMap<'a> {
    fn get(&self, position: Position) -> Option<bool> {
        if self.1 == position {
            Some(true)
        } else {
            self.0.get(position)
        }
    }

    fn next_position(&self, current: Position) -> Option<Position> {
        self.0.next_position(current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_move_straight() {
        let map = Map(vec![vec![false, false]]);
        let initial = Guard(Position(0, 0), DIRECTION_RIGHT);
        assert_eq!(initial.try_move(&map), Some(Guard(Position(0, 1), DIRECTION_RIGHT)));
    }

    #[test]
    fn try_move_rotation() {
        let map = Map(vec![vec![false, true]]);
        let initial = Guard(Position(0, 0), DIRECTION_RIGHT);
        assert_eq!(initial.try_move(&map), Some(Guard(Position(0, 0), DIRECTION_DOWN)));
    }

    #[test]
    fn try_move_off_map_left() {
        let map = Map(vec![vec![false]]);
        let initial = Guard(Position(0, 0), DIRECTION_LEFT);
        assert_eq!(initial.try_move(&map), None);
    }

    #[test]
    fn try_move_off_map_right() {
        let map = Map(vec![vec![false]]);
        let initial = Guard(Position(0, 0), DIRECTION_RIGHT);
        assert_eq!(initial.try_move(&map), None);
    }
}
