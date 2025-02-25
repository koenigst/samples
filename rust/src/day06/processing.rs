use std::collections::HashSet;

use crate::vector::Position;

use super::map::*;
use super::parsing::Parsed;

struct Walk<M: Maplike>(M, Option<Guard>);

impl<M: Maplike> Walk<M> {
    fn is_loop(self) -> bool {
        let mut previous = HashSet::new();

        for current in self {
            if !previous.insert(current) {
                return true;
            }
        }

        false
    }
}

impl<M: Maplike> Iterator for Walk<M> {
    type Item = Guard;
    
    fn next(&mut self) -> Option<Self::Item> {
        let guard = self.1?;
        self.1 = guard.try_move(&self.0);
        Some(guard)
    }
}

impl From<Parsed> for Walk<Map> {
    fn from(value: Parsed) -> Self {
        Walk(value.0, Some(value.1))
    }
}

pub struct GuardPositions(Walk<Map>);

impl From<Parsed> for GuardPositions {
    fn from(value: Parsed) -> Self {
        GuardPositions(value.into())
    }
}

impl Iterator for GuardPositions {
    type Item = Position;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|g| g.0)    
    }
}

pub struct Obstructions(Parsed, Option<Position>);

impl From<Parsed> for Obstructions {
    fn from(value: Parsed) -> Self {
        Obstructions(value, Some(Position(0, 0)))
    }
}

impl Iterator for Obstructions {
    type Item = Position;
    
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let current = self.1?;
            self.1 = self.0.0.next_position(current);

            if current != self.0.1.0 {
                if let Some(obstructed) = self.0.0.with_obstruction(current) {
                    if Walk(obstructed, Some(self.0.1)).is_loop() {
                        break Some(current)
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utilities::IntoLines;

    use super::super::Parsed;
    use super::*;

    #[test]
    fn obstruction_regular() {
        assert_obstructions([
            ".#.", 
            ">..", 
            "#..", 
            ".#."],
            &[Position(1, 2)]);
    }

    #[test]
    fn obstruction_at_start() {
        assert_obstructions([
            ".>..#",
            "..#..", 
            "#....", 
            ".#.#."],
            &[]);
    }

    fn assert_obstructions<'a>(input: impl IntoLines<'a>, expected: &[Position]) {
        let parsed : Parsed = input.into();
        let obstructions : Obstructions = parsed.into();
        assert_eq!(obstructions.collect::<Vec<_>>(), expected)
    }
}
