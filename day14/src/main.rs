#![allow(unused)]
mod parser;
mod point;

use grid::Grid;
use itertools::{iproduct, Itertools};
use parser::scan_line;
use point::PointPair;
use std::{default::Default, fmt::Display};

pub(crate) use point::Point;

#[derive(Debug, PartialEq, Eq)]
struct Scan(Vec<Point>);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Spot {
    Rock,
    Sand,
    Air,
    Start,
}

impl Display for Spot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Spot::Rock => write!(f, "#"),
            Spot::Sand => write!(f, "o"),
            Spot::Air => write!(f, "."),
            Spot::Start => write!(f, "+"),
        }
    }
}

impl Default for Spot {
    fn default() -> Self {
        Spot::Air
    }
}

struct Cave(Grid<Spot>);

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let first_col = self.first_used_col().expect("Need valid grid");

        write!(f, "    ")?;
        for col in first_col..self.0.cols() {
            if col % 10 == 0 {
                write!(f, "X")?;
            } else {
                write!(f, " ")?;
            }
        }
        writeln!(f)?;
        for row in 0..self.0.rows() {
            write!(f, "{:>3} ", row)?;
            for col in first_col..self.0.cols() {
                write!(f, "{}", self.0[row][col])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Cave {
    fn start() -> Point {
        Point(500, 0)
    }

    fn new(scans: Vec<Scan>) -> Self {
        let max_row = scans
            .iter()
            .flat_map(|s| s.0.iter().map(|p| p.1))
            .max()
            .unwrap()
            + 1;
        let max_col = scans
            .iter()
            .flat_map(|s| s.0.iter().map(|p| p.0))
            .max()
            .unwrap()
            + 1;
        // dbg!(max_row, max_col);
        let mut c = Cave(Grid::new(max_row as usize, max_col as usize));
        c.set(&Self::start(), Spot::Start);

        c.apply_scans(&scans);
        c
    }

    fn first_used_col(&self) -> Option<usize> {
        for col in 0..self.0.cols() {
            let x = (0..self.0.rows()).find(|&row| self.0[row][col] != Spot::Air);
            if x.is_some() {
                return Some(col);
            }
        }
        None
    }

    fn set(&mut self, point: &Point, spot: Spot) {
        let row = point.1 as usize;
        let col = point.0 as usize;
        let grid_point = self
            .0
            .get_mut(row, col)
            .expect(format!("point {row},{col} to exist").as_str());
        *grid_point = spot;
    }

    fn apply(&mut self, scan: &Scan) {
        scan.0 //
            .windows(2)
            .map(PointPair::try_from)
            .for_each(|pp| {
                pp.unwrap()
                    .into_iter()
                    .for_each(|point| self.set(&point, Spot::Rock));
            });
    }

    fn apply_scans(&mut self, scans: &[Scan]) {
        scans.iter().for_each(|s| self.apply(s))
    }

    fn get(&self, point: Point) -> Option<Spot> {
        self.0.get(point.row(), point.col()).map(|p| p.clone())
    }

    /// If None, it's slipped off the map
    fn next_point(&self, point: Point) -> Option<Point> {
        let below = point + (0, 1);
        let below_left = point + (-1, 1);
        let below_right = point + (1, 1);
        if self.get(below).is_none() {
            // gone off the bottom of the map
            return None;
        }
        if self.get(below).unwrap() == Spot::Air {
            return Some(below);
        }
        if self.get(below_left).unwrap() == Spot::Air {
            return Some(below_left);
        }
        if self.get(below_right).unwrap() == Spot::Air {
            return Some(below_right);
        }

        // can't move, stay still
        Some(point)
    }

    fn drop_sand(&mut self) -> bool {
        let mut sand_point = Self::start();
        loop {
            let next_point = self.next_point(sand_point);
            if next_point.is_none() {
                // dropped off
                return false;
            }
            if next_point.unwrap() == sand_point {
                // We're not moving any more
                self.set(&sand_point, Spot::Sand);
                return true;
            }
            sand_point = next_point.unwrap();
        }
    }
}

fn main() -> color_eyre::Result<()> {
    // let input = include_str!("sample.txt");
    let input = include_str!("input.txt");
    let scans: Vec<_> = input
        .lines()
        .map(scan_line)
        .collect::<Result<Vec<_>, _>>()?;

    let mut cave = Cave::new(scans);
    let mut sand = 0;
    loop {
        let finished = !cave.drop_sand();
        if finished {
            break;
        }
        sand+=1;
    }
    println!("{cave}");
    println!("Sand: {sand}");
    Ok(())
}
