use crate::utilities::IntoLines;
use crate::vector::Position;
use super::direction::CardinalDirection;
use super::map::*;

pub struct Parsed(pub Map, pub Guard);

impl<'a, T: IntoLines<'a>> From<T> for Parsed {
    fn from(value: T) -> Self {
        let parsed : Vec<Vec<ParsedPoint>> = value
            .into_iter()
            .map(|l| l.chars().map(|c| c.into()).collect())
            .collect();
    
        let map = parsed
            .iter()
            .map(|l| l.iter().map(|t| t.0).collect())
            .collect();
    
        let guard = parsed
            .iter()
            .enumerate()
            .flat_map(|(x, l)| l.iter().enumerate().map(move |(y, p)| (Position(x, y), p.1)))
            .filter_map(|(p, md)| md.map(|d| Guard(p, d)))
            .next()
            .unwrap();
    
        Parsed(Map(map), guard)
    }
}

struct ParsedPoint(bool, Option<CardinalDirection>);

impl From<char> for ParsedPoint {
    fn from(value: char) -> Self {
        match value {
            '#' => Self(true, None),
            _ => Self(false, value.try_into().ok())
        }
    }
}
