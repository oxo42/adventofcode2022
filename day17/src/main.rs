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
enum Jet {
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
        let pos = Pos::new(2, self.the_top() + rock.height() + 3);
    }
}

fn main() {
    println!("Hello, world!");
}
