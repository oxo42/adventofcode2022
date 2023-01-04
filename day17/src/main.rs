#![allow(unused)]

mod parser;
mod pos;
mod rock;

use std::fmt::Debug;

use pos::Pos;
use rock::{Rock, RockIterator};

extern crate derivative;
use derivative::Derivative;

#[derive(Debug)]
pub enum Jet {
    Left,
    Right,
}

#[derive(Derivative)]
#[derivative(Debug)]
struct Chamber {
    settled: Vec<(Rock, Pos)>,
    #[derivative(Debug = "ignore")]
    rock_iter: RockIterator,
    rounds: usize,
    jets: Vec<Jet>,
}

impl Chamber {
    fn parse(input: &str) -> color_eyre::Result<Self> {
        let jets = parser::parse_line(input)?;
        Ok(Self::new(jets))
    }

    fn new(jets: Vec<Jet>) -> Self {
        Self {
            settled: Vec::new(),
            rock_iter: RockIterator::new(),
            rounds: 0,
            jets,
        }
    }

    /// The very highest rock
    fn the_top(&self) -> isize {
        self.settled.last().map_or(0, |(_, p)| p.y)
    }

    fn drop(&mut self) {
        let rock = self.rock_iter.next().unwrap();
        let mut pos = Pos::new(2, self.the_top() + rock.height() + 3);
        let mut jet = self.jets.iter();
        let final_pos = loop {
            let shift = jet.next().expect("not to run out of jets");
            let next_pos = pos + shift;
            if self.collision(rock, next_pos) {
                break pos;
            } else {
                pos = next_pos;
            }

            let next_pos = pos + (0, 1);

            if self.collision(rock, next_pos) {
                break pos;
            } else {
                pos = next_pos;
            }
        };
        self.settled.push((rock, final_pos));
    }

    fn update_tops(&mut self, rock: Rock, pos: Pos) {
        todo!()
    }

    fn has_rock_in(&self, pos: Pos) -> bool {
        if pos.y < 0 {
            return false;
        }
        if pos.y > self.the_top() {
            return false;
        }
        self.settled //
            .iter()
            .rev()
            .flat_map(|(rock, pos)| rock.places_iter().map(|p| *pos + p))
            .any(|p| pos == p)
    }

    fn collision(&self, rock: Rock, pos: Pos) -> bool {
        rock.places_iter() //
            .map(|p| pos + p)
            .any(|rock_pos| self.has_rock_in(rock_pos))
    }

    fn drop_rounds(&mut self, rounds: usize) {
        for _ in 0..rounds {
            self.drop();
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_sample() -> color_eyre::Result<()> {
        let input = include_str!("sample.txt");
        let mut chamber = Chamber::parse(input)?;
        chamber.drop_rounds(2022);
        assert_eq!(3068, chamber.the_top());
        Ok(())
    }
}
