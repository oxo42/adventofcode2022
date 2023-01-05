#![allow(unused)]

mod parser;
mod pos;
mod rock;

use std::fmt::{Debug, Display};

use pos::Pos;
use rock::{Rock, RockIterator};

extern crate derivative;
use derivative::Derivative;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Jet {
    Left,
    Right,
}

impl Display for Jet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Jet::Left => "<",
                Jet::Right => ">",
            }
        )
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
struct Chamber {
    settled: Vec<(Rock, Pos)>,
    #[derivative(Debug = "ignore")]
    rock_iter: RockIterator,
    rounds: usize,
    jets: std::iter::Cycle<std::vec::IntoIter<Jet>>,
}

impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in (0..=self.the_top()).rev() {
            for col in (0..=6) {
                let rc = Pos::new(col, row);
                let rock = self
                    .settled
                    .iter()
                    .rev()
                    .flat_map(|(rock, pos)| rock.pos_iter(*pos))
                    .any(|p| p == rc);
                if rock {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
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
            jets: jets.into_iter().cycle(),
        }
    }

    /// The very highest rock
    fn the_top(&self) -> isize {
        // -1 is the floor
        self.settled.last().map_or(-1, |(_, p)| p.y)
    }

    fn drop(&mut self) {
        let rock = self.rock_iter.next().unwrap();
        let mut pos = Pos::new(2, self.the_top() + rock.height() + 3);
        // println!("New rock {rock:?} at {pos}");
        let final_pos = loop {
            let shift = self.jets.next().expect("Not to run out of jets");
            let next_pos = {
                let np = pos + shift;
                if rock.pos_iter(np).any(|p| p.x >= 7 || p.x < 0) {
                    pos
                } else {
                    np
                }
            };
            // println!(" Gas to {next_pos}");
            if self.collision(rock, next_pos) {
                break pos;
            } else {
                pos = next_pos;
            }

            let next_pos = pos + (0, -1); // Rock drops
                                          // println!("    Dropped to {next_pos}");

            if self.collision(rock, next_pos) {
                break pos;
            } else {
                pos = next_pos;
            }
        };
        // println!("Placing {rock:?} at {final_pos}");
        self.settled.push((rock, final_pos));
    }

    fn update_tops(&mut self, rock: Rock, pos: Pos) {
        todo!()
    }

    fn is_empty(&self, pos: Pos, debug: bool) -> bool {
        if pos.y < 0 {
            return false;
        }
        if self.settled.is_empty() {
            return true;
        }
        let connect_with_settled = self
            .settled //
            .iter()
            .rev()
            .flat_map(|(rock, pos)| rock.pos_iter(*pos))
            .any(|p| pos == p);

        // println!("{pos} connects with settled {connect_with_settled}");

        !connect_with_settled
    }

    fn collision(&self, rock: Rock, pos: Pos) -> bool {
        let d = false;
        let d = rock == Rock::Box;
        let is_all_rock_empty = rock
            .pos_iter(pos) //
            .all(|rock_pos| self.is_empty(rock_pos, d));
        if (d) {
            // println!("{rock:?} at {pos} is empty: {is_all_rock_empty}");
        }
        !is_all_rock_empty
    }

    fn drop_rounds(&mut self, rounds: usize) {
        for _ in 0..rounds {
            self.drop();
            // println!("{self}");
            // println!();
        }
    }

    fn height(&self) -> isize {
        self.the_top() + 1
    }
}

fn main() -> color_eyre::Result<()> {
    let input = include_str!("sample.txt");
    let mut chamber = Chamber::parse(input)?;
    chamber.drop_rounds(2022);
    println!("Chamber heigh {}", chamber.height());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(2, 4)]
    #[test_case(3, 6)]
    #[test_case(4, 7)]
    #[test_case(5, 9)]
    #[test_case(2022, 3068)]
    fn test_sample(rounds: usize, height: isize) -> color_eyre::Result<()> {
        let input = include_str!("sample.txt");
        let mut chamber = Chamber::parse(input)?;
        chamber.drop_rounds(rounds);
        assert_eq!(height, chamber.height());
        Ok(())
    }
}
