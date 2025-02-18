use std::{collections::HashMap, sync::LazyLock};

pub const DIRECTION_RIGHT : Direction = Direction(0, 1);
pub const DIRECTION_DOWN : Direction = Direction(1, 0);
pub const DIRECTION_LEFT : Direction = Direction(0, -1);
pub const DIRECTION_UP : Direction = Direction(-1, 0);

const DIRECTIONS : &[Direction] = &[
    DIRECTION_RIGHT,
    DIRECTION_DOWN,
    DIRECTION_LEFT,
    DIRECTION_UP,
];

static DIRECTION_MAP : LazyLock<HashMap<char, Direction>> = LazyLock::new(|| HashMap::from([
    ('>', DIRECTION_RIGHT),
    ('V', DIRECTION_DOWN),
    ('<', DIRECTION_LEFT),
    ('^', DIRECTION_UP),
]));

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Direction(pub i32, pub i32);

impl Direction {
    pub fn turn_right(self) -> Direction {
        DIRECTIONS
            .iter()
            .skip_while(|d| !self.eq(&d))
            .skip(1)
            .next()
            .map(|&d| d)
            .unwrap_or(DIRECTIONS[0])
    }    
}

impl TryFrom<char> for Direction {
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
