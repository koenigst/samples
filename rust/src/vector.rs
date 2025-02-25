use std::{num::TryFromIntError, ops::{Add, Mul, Sub}};

pub type PositionElementType = usize;
pub type DirectionElementType = i64;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct Position(pub PositionElementType, pub PositionElementType);

impl TryFrom<Direction> for Position {
    type Error = TryFromIntError;

    fn try_from(value: Direction) -> Result<Self, Self::Error> {
        Ok(Position(value.0.try_into()?, value.1.try_into()?))
    }
}

impl Sub for Position {
    type Output = Result<Direction, TryFromIntError>;

    fn sub(self, rhs: Self) -> Self::Output {
        let l: Direction = self.try_into()?;
        let r: Direction = rhs.try_into()?;
        Ok(l - r)
    }
}

impl Add<Direction> for Position {
    type Output = Result<Position, TryFromIntError>;

    fn add(self, rhs: Direction) -> Self::Output {
        let l: Direction = self.try_into()?;
        (l + rhs).try_into()
    }
}

impl Sub<Direction> for Position {
    type Output = Result<Position, TryFromIntError>;

    fn sub(self, rhs: Direction) -> Self::Output {
        let l: Direction = self.try_into()?;
        (l - rhs).try_into()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct Direction(pub DirectionElementType, pub DirectionElementType);

impl TryFrom<Position> for Direction {
    type Error = TryFromIntError;

    fn try_from(value: Position) -> Result<Self, Self::Error> {
        Ok(Direction(value.0.try_into()?, value.1.try_into()?))
    }
}

impl Add for Direction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Direction(
            self.0 + rhs.0,
            self.1 + rhs.1,
        )
    }
}

impl Sub for Direction {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Direction(
            self.0 - rhs.0,
            self.1 - rhs.1,
        )
    }
}

impl Mul<DirectionElementType> for Direction {
    type Output = Self;

    fn mul(self, rhs: DirectionElementType) -> Self::Output {
        Direction(
            rhs * self.0,
            rhs * self.1,
        )
    }
}

impl Mul<Direction> for DirectionElementType {
    type Output = Direction;

    fn mul(self, rhs: Direction) -> Self::Output {
        rhs * self
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct Vector(pub Position, pub Direction);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub_mul_mul_add_sub() {
        let d = (Position(1, 2) - Position(3, 1)).map(|d| 2 * d * 3);
        let p = d.and_then(|d| Position(14, 9) + d).and_then(|p| p - Direction(-2, 3));
        assert_eq!(p, Ok(Position(4, 12)));
    }
}
