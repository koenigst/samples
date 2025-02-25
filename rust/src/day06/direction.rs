use std::{collections::HashMap, sync::LazyLock};

use crate::vector::Direction;

pub static DIRECTION_RIGHT : CardinalDirection = CardinalDirection(Direction(0, 1));
pub static DIRECTION_DOWN : CardinalDirection = CardinalDirection(Direction(1, 0));
pub static DIRECTION_LEFT : CardinalDirection = CardinalDirection(Direction(0, -1));
pub static DIRECTION_UP : CardinalDirection = CardinalDirection(Direction(-1, 0));

const DIRECTIONS : &[CardinalDirection] = &[
    DIRECTION_RIGHT,
    DIRECTION_DOWN,
    DIRECTION_LEFT,
    DIRECTION_UP,
];

static DIRECTION_MAP : LazyLock<HashMap<char, CardinalDirection>> = LazyLock::new(|| HashMap::from([
    ('>', DIRECTION_RIGHT),
    ('V', DIRECTION_DOWN),
    ('<', DIRECTION_LEFT),
    ('^', DIRECTION_UP),
]));

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct CardinalDirection(Direction);

impl CardinalDirection {
    pub fn turn_right(self) -> CardinalDirection {
        DIRECTIONS
            .iter()
            .skip_while(|d| !self.eq(&d))
            .skip(1)
            .next()
            .map(|&d| d)
            .unwrap_or(DIRECTIONS[0])
    }    
}

impl From<CardinalDirection> for Direction {
    fn from(value: CardinalDirection) -> Self {
        value.0
    }
}

impl TryFrom<char> for CardinalDirection {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        DIRECTION_MAP.get(&value).copied().ok_or(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn turn_down() {
        assert_eq!(DIRECTION_DOWN.turn_right(), DIRECTION_LEFT);
    }

    #[test]
    fn turn_up() {
        assert_eq!(DIRECTION_UP.turn_right(), DIRECTION_RIGHT);
    }
}
