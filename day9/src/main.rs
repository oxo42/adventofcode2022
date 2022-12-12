use std::{collections::HashSet, fmt::Debug, str::FromStr};

use color_eyre::eyre::{bail, eyre};
use itertools::Itertools;
use rusttype::Point;

#[allow(dead_code)]
const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Direction::*;
        match s {
            "U" => Ok(Up),
            "D" => Ok(Down),
            "L" => Ok(Left),
            "R" => Ok(Right),
            _ => bail!("{} not valid", s),
        }
    }
}

fn point(x: i32, y: i32) -> Point<i32> {
    Point { x, y }
}

impl Direction {
    fn point(&self) -> Point<i32> {
        match self {
            Direction::Up => point(0, 1),
            Direction::Down => point(0, -1),
            Direction::Left => point(-1, 0),
            Direction::Right => point(1, 0),
        }
    }
}

#[derive(Debug)]
struct Move {
    dir: Direction,
    count: usize,
}

impl FromStr for Move {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, count) = s
            .split(" ")
            .collect_tuple()
            .ok_or(eyre!("can't split string"))?;
        Ok(Move {
            dir: str::parse(dir)?,
            count: str::parse(count)?,
        })
    }
}

fn parse_moves(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(str::parse::<Move>)
        .map(|m| m.unwrap())
        .collect_vec()
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // let moves = parse_moves(INPUT);
    let moves = parse_moves(include_str!("input.txt"));

    println!("{:?}", moves);

    let mut engine = Engine::new();

    for mv in moves {
        engine.apply(mv);
    }

    println!("{:?}", engine);
    println!("Tail locations: {}", engine.count_tail_locations());

    Ok(())
}

struct Engine {
    head: Point<i32>,
    tail: Point<i32>,
    tail_locations: HashSet<Point<i32>>,
}

impl Debug for Engine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Engine")
            .field("head", &(&self.head.x, &self.head.y))
            .field("tail", &(&self.tail.x, &self.tail.y))
            .field(
                "tail_locations",
                &self
                    .tail_locations
                    .iter()
                    .map(|p| (p.x, p.y))
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}

trait PointExt {
    fn is_touching(&self, other: &Point<i32>) -> bool;
    fn add_delta(&self, delta: &Point<i32>) -> Point<i32>;
    fn sub_delta(&self, delta: &Point<i32>) -> Point<i32>;
}

impl PointExt for Point<i32> {
    fn is_touching(&self, other: &Point<i32>) -> bool {
        self.x.abs_diff(other.x) <= 1 && self.y.abs_diff(other.y) <= 1
    }

    fn add_delta(&self, delta: &Point<i32>) -> Point<i32> {
        point(self.x + delta.x, self.y + delta.y)
    }
    fn sub_delta(&self, delta: &Point<i32>) -> Point<i32> {
        point(self.x - delta.x, self.y - delta.y)
    }
}

impl Engine {
    fn new() -> Self {
        Self {
            head: point(0, 0),
            tail: point(0, 0),
            tail_locations: HashSet::from([point(0, 0)]),
        }
    }
    fn apply(&mut self, mv: Move) {
        // println!("Move: {:?}", mv);
        for _ in 0..mv.count {
            let delta: Point<_> = mv.dir.point();
            self.head = self.head.add_delta(&delta);
            if !self.head.is_touching(&self.tail) {
                self.tail = self.head.sub_delta(&delta);
                self.tail_locations.insert(self.tail.clone());
            }
            // println!("{:?}", self);
        }
    }

    fn count_tail_locations(&self) -> usize {
        self.tail_locations.iter().count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROPE2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";

    #[test]
    fn test_rope_2() -> color_eyre::Result<()> {
        let moves = parse_moves(ROPE2);

        let mut engine = Engine::new();
        println!("{:?}", engine);
        for mv in moves {
            println!("{:?}", &mv);
            engine.apply(mv);
            println!("{:?}", &engine);
        }
        println!("{:?}", engine);

        assert_eq!(36, engine.count_tail_locations());

        Ok(())
    }

    #[test]
    fn test_tail_count() -> color_eyre::Result<()> {
        let moves = parse_moves(INPUT);

        let mut engine = Engine::new();
        println!("{:?}", engine);
        for mv in moves {
            println!("{:?}", &mv);
            engine.apply(mv);
            println!("{:?}", &engine);
        }
        println!("{:?}", engine);

        assert_eq!(13, engine.count_tail_locations());

        Ok(())
    }

    #[test]
    fn test_is_touching() {
        assert!(point(1, 1).is_touching(&point(0, 0)));
        assert!(!point(4, 0).is_touching(&point(0, 0)));
    }
}
