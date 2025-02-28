use std::{collections::HashMap, num::TryFromIntError, ops::{Div, Rem}, slice::Iter};

use crate::vector::{Direction, DirectionElementType, Position, Vector};

pub struct MapSquare {
    position: Position,
    frequency: Option<char>,
}

pub struct Map(Vec<Vec<MapSquare>>);

impl Map {
    pub fn nodes_gold<'a>(&'a self) -> NodesGold<'a> {
        NodesGold {
            map: self,
            pairs: self.antenna_pairs(),
            current: vec![],
        }
    }

    pub fn nodes_silver<'a>(&'a self) -> NodesSilver<'a> {
        NodesSilver {
            squares: self.squares(),
            lines: self
                .antenna_pairs()
                .filter_map(|(l, r)| l.node_line_silver(&r).ok())
                .collect(),
        }
    }

    fn squares<'a>(&'a self) -> Squares<'a> {
        Squares {
            rows: self.0.iter(),
            current: None,
        }
    }

    fn antennas<'a>(&'a self) -> Antennas<'a> {
        Antennas {
            squares: self.squares(),
        }
    }

    fn antenna_pairs<'a>(&'a self) -> AntennaPairs<'a> {
        AntennaPairs {
            antennas: self.antennas(),
            current: vec![],
            previous: HashMap::new(),
        }
    }

    fn is_in_bounds(&self, position: Position) -> bool {
        self.0
            .get(position.0)
            .map(|r| position.1 < r.len())
            .unwrap_or(false)
    }
}

impl<'a> FromIterator<&'a str> for Map {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        fn parse_line(index: usize, line: &str) -> Vec<MapSquare> {
            line
                .chars()
                .enumerate()
                .map(|(i, c)| parse_character(index, i, c))
                .collect()
        }

        fn parse_character(row: usize, column: usize, character: char) -> MapSquare {
            let position = Position(row, column);

            let frequency = if character == '.' { 
                None 
            } else {
                Some(character)
            };

            MapSquare { position, frequency, }
        }

        let rows = iter
            .into_iter()
            .enumerate()
            .map(|(i, l)| parse_line(i, l))
            .collect();

        Map(rows)
    }
}

#[derive(Clone, Copy)]
struct Antenna {
    position: Position,
    frequency: char,
}

impl Antenna {
    fn node_gold(&self, other: &Antenna) -> Result<Position, TryFromIntError> {
        self.position + (2 * ((other.position - self.position)?))
    }
    
    fn node_line_silver(&self, other: &Antenna) -> Result<NodeLine, TryFromIntError> {
        Ok(NodeLine(Vector(self.position, (other.position - self.position)?)))
    }
}

struct Squares<'a> {
    rows: Iter<'a, Vec<MapSquare>>,
    current: Option<Iter<'a, MapSquare>>,
}

impl<'a> Iterator for Squares<'a> {
    type Item = &'a MapSquare;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(row) = self.current.as_mut() {
                if let Some(value) = row.next() {
                    break Some(value)
                }
            }
    
            if let Some(row) = self.rows.next() {
                self.current = Some(row.iter());
            } else {
                break None
            }
        }
    }
}

struct Antennas<'a> {
    squares: Squares<'a>,
}

impl<'a> Iterator for Antennas<'a> {
    type Item = Antenna;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let square = self.squares.next()?;
            if let Some(frequency) = square.frequency {
                break Some(Antenna { position: square.position, frequency, })
            }
        }
    }
}

struct AntennaPairs<'a> {
    antennas: Antennas<'a>,
    current: Vec<(Antenna, Antenna)>,
    previous: HashMap<char, Vec<Antenna>>,
}

impl<'a> Iterator for AntennaPairs<'a> {
    type Item = (Antenna, Antenna);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(pair) = self.current.pop() {
                break Some(pair)
            }
    
            let antenna = self.antennas.next()?;
            let entry = self.previous.entry(antenna.frequency);
            let previous = entry.or_insert_with(|| vec![]);

            for &other in previous.iter() {
                self.current.push((other, antenna));
            }

            previous.push(antenna);
        }
    }
}

pub struct NodesGold<'a> {
    map: &'a Map,
    pairs: AntennaPairs<'a>,
    current: Vec<Position>,
}

impl<'a> Iterator for NodesGold<'a> {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(position) = self.current.pop() {
                break Some(position)
            }
    
            let pair = self.pairs.next()?;
            let positions = [
                    pair.0.node_gold(&pair.1),
                    pair.1.node_gold(&pair.0),
                ]
                .into_iter()
                .filter_map(|p| p.ok())
                .filter(|&p| self.map.is_in_bounds(p));

            for position in positions {
                self.current.push(position);
            }
        }
    }
}

struct NodeLine(Vector);

impl NodeLine {
    fn is_node(&self, position: Position) -> bool {
        fn compute_scale(long: Direction, short: Direction) -> Option<DirectionElementType>
        {
            if short.0 == 0 {
                if short.1 == 0 {
                    None
                } else {
                    compute_scale_for(long.1, short.1)
                }
            } else {
                compute_scale_for(long.0, short.0)
            }
        }

        fn compute_scale_for<T>(long: T, short: T) -> Option<<T as Div>::Output> 
        where
            T: Rem<T> + Div + Copy,
            <T as Rem>::Output: PartialEq + Default,
        {
            if long % short == <T as Rem>::Output::default() {
                Some(long / short)
            } else {
                None
            }
        }


        if self.0.0 == position {
            return true
        }

        (position - self.0.0)
            .ok()
            .and_then(|d| compute_scale(d, self.0.1))
            .and_then(|s| (self.0.0 + s * self.0.1).ok())
            .map(|p| p == position)
            .unwrap_or(false)
    }
}

pub struct NodesSilver<'a> {
    squares: Squares<'a>,
    lines: Vec<NodeLine>,
}

impl<'a> Iterator for NodesSilver<'a> {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let square = self.squares.next()?;
            for line in self.lines.iter() {
                if line.is_node(square.position) {
                    return Some(square.position);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn parse_map() {
        let map : Map = [".a."].into_iter().collect();
        assert_eq!(map.squares().find(|s| s.position == Position(0, 1)).map(|a| a.frequency).flatten(), Some('a'));
        assert_eq!(map.antennas().count(), 1);
    }

    #[test]
    fn antenna_pairs() {
        let map : Map = ["baaba"].into_iter().collect();
        assert_eq!(map.antenna_pairs().count(), 4);
    }

    #[test]
    fn nodes_gold() {
        let map : Map = ["baaba"].into_iter().collect();
        assert_nodes(map.nodes_gold(), &[Position(0, 0), Position(0, 3),]);
    }

    #[test]
    fn nodes_silver() {
        let map : Map = ["a..", "aa.", "..."].into_iter().collect();
        assert_nodes(map.nodes_silver(), &[Position(0, 0), Position(1, 0), Position(1, 1), Position(1, 2), Position(2, 0), Position(2, 2),]);
    }

    fn assert_nodes(iter: impl Iterator<Item = Position>, expected: &[Position]) {
        assert_eq!(iter.collect::<BTreeSet<_>>().into_iter().collect::<Vec<_>>(), expected);
    }
}
