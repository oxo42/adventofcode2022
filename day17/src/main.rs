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
    top_spots: [Pos; 7],
}

impl Chamber {
    fn parse(input: &str) -> color_eyre::Result<Self> {
        let jets = parser::parse_line(input)?;
        Ok(Self::new(jets))
    }

    fn new(jets: Vec<Jet>) -> Self {
        // top is floor
        let t: Vec<Pos> = (0..6).into_iter().map(|x| Pos::new(x, 0)).collect();
        let top_spots: [Pos; 7] = t.try_into().expect("should be fine");
        Self {
            settled: Vec::new(),
            rock_iter: RockIterator::new(),
            rounds: 0,
            jets,
            top_spots,
        }
    }

    fn the_top(&self) -> usize {
        self.top_spots.iter().map(|p| p.y).max().unwrap()
    }

    fn drop(&mut self) {
        let rock = self.rock_iter.next().unwrap();
        let mut pos = Pos::new(2, self.the_top() + rock.height() + 3);
        loop {
            let next_pos = pos + (0, 1);
            if self.collision(rock, next_pos) {
                // we have collided, record current pos into new top_spots and break
                self.update_tops(rock, pos);
            } else {
                pos = next_pos;
            }
        }
    }

    fn update_tops(&mut self, rock: Rock, pos: Pos) {
        todo!()
    }

    fn collision(&self, rock: Rock, pos: Pos) -> bool {
        todo!()
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
