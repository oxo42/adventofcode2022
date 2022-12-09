#![allow(dead_code)]
mod parser;

use itertools::Itertools;
use nom::{combinator::all_consuming, Finish};
use std::fmt::Debug;
use std::{fmt::Display, num::ParseIntError};

use crate::parser::{dock_line, parse_mv};

pub const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

#[derive(Debug)]
pub struct Dock(Vec<Vec<Crate>>);

impl Dock {
    pub fn apply(&mut self, mv: Move) {
        for _ in 0..mv.count {
            let el = self.0[mv.source].pop().unwrap();
            self.0[mv.dest].push(el);
        }
    }

    fn last_crates(&self) -> String {
        self.0
            .iter()
            .filter_map(|s| s.last())
            .map(|c| c.0)
            .collect()
    }
}

impl Display for Dock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, stack) in self.0.iter().enumerate() {
            writeln!(f, "Stack {i}: {stack:?}")?;
        }
        Ok(())
    }
}

pub struct Crate(char);
impl Debug for Crate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Move {
    count: usize,
    source: usize,
    dest: usize,
}

impl From<(usize, usize, usize)> for Move {
    fn from((count, source, dest): (usize, usize, usize)) -> Self {
        Move::new(count, source, dest)
    }
}

impl TryFrom<(&str, &str, &str)> for Move {
    type Error = ParseIntError;

    fn try_from((cnt, src, dst): (&str, &str, &str)) -> Result<Self, Self::Error> {
        Ok(Move::new(cnt.parse()?, src.parse()?, dst.parse()?))
    }
}

impl Move {
    pub fn new(count: usize, source: usize, dest: usize) -> Self {
        Self {
            source,
            count,
            dest,
        }
    }
}

fn transpose_rev<T>(v: Vec<Vec<Option<T>>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters = v.into_iter().map(|n| n.into_iter()).collect_vec();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .rev()
                .filter_map(|n| n.next().unwrap())
                .collect_vec()
        })
        .collect()
}

fn main() -> color_eyre::Result<()> {
    let mut lines = include_str!("../input.txt").lines();

    let crate_lines: Vec<_> = lines
        .by_ref()
        .map_while(|line| {
            (all_consuming(dock_line)(line))
                .finish()
                .ok()
                .map(|(_, cl)| cl)
        })
        .collect();

    let crate_cols = transpose_rev(crate_lines);
    println!("{crate_cols:?}");

    lines.next();

    let moves = lines
        .map_while(|line| {
            (all_consuming(parse_mv)(line))
                .finish()
                .ok()
                .map(|(_, mv)| mv)
        })
        .collect_vec();
    let mut dock = Dock(crate_cols);

    println!("{dock}");

    for mv in moves {
        dock.apply(mv);
    }
    println!("{dock}");

    println!("{}", dock.last_crates());

    Ok(())
}

#[cfg(test)]
mod tests {
    pub const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";
}
