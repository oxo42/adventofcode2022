mod parser;

use std::fmt::Display;

use grid::Grid;
use pathfinding::prelude::dijkstra;

#[derive(Debug)]
struct Map(Grid<MapItem>);

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.0.rows() {
            for item in self.0.iter_row(row) {
                write!(f, "{}", item)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct Pos(usize, usize);

impl Pos {
    fn surrounds(&self) -> Vec<Pos> {
        let Pos(row, col) = self;
        let mut v = Vec::new();
        if *row > 0 {
            v.push(Pos(row - 1, *col));
            // v.push(Pos(row - 1, col + 1));
            // if *col > 0 {
            //     v.push(Pos(row - 1, col - 1));
            // }
        }
        if *col > 0 {
            v.push(Pos(*row, col - 1));
            // v.push(Pos(row + 1, col - 1));
        }
        v.push(Pos(row + 1, *col));
        // v.push(Pos(row + 1, col + 1));
        v.push(Pos(*row, col + 1));
        v
    }
}

impl Map {
    fn start(&self) -> Option<Pos> {
        for row in 0..self.0.rows() {
            for col in 0..self.0.cols() {
                if let Some(item) = self.0.get(row, col) {
                    if *item == MapItem::Start {
                        return Some(Pos(row, col));
                    }
                }
            }
        }
        None
    }
    fn end(&self) -> Option<Pos> {
        for row in 0..self.0.rows() {
            for col in 0..self.0.cols() {
                if let Some(item) = self.0.get(row, col) {
                    if *item == MapItem::End {
                        return Some(Pos(row, col));
                    }
                }
            }
        }
        None
    }

    fn the_a_points(&self) -> Vec<Pos> {
        let mut v = Vec::new();
        for row in 0..self.0.rows() {
            for col in 0..self.0.cols() {
                if let Some(item) = self.0.get(row, col) {
                    if *item == MapItem::Ground('a') {
                        v.push(Pos(row, col));
                    }
                }
            }
        }
        v
    }

    fn surrounds(&self, pos: Pos) -> Vec<Pos> {
        pos.surrounds()
            .into_iter()
            .filter(|Pos(row, col)| *row < self.0.rows() && *col < self.0.cols())
            .collect()
    }

    fn neighbors(&self, pos: Pos) -> impl Iterator<Item = (Pos, usize)> + '_ {
        self.surrounds(pos)
            .into_iter()
            .filter(move |p| {
                let current = *self.0.get(pos.0, pos.1).unwrap();
                let m = *self.0.get(p.0, p.1).unwrap();
                current.can_step(&m)
            })
            .map(|p| (p.clone(), 1))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MapItem {
    Start,
    End,
    Ground(char),
}
impl Display for MapItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            MapItem::Start => 'S',
            MapItem::End => 'E',
            MapItem::Ground(c) => *c,
        };
        write!(f, "{}", c)
    }
}

impl MapItem {
    fn can_step(&self, other: &MapItem) -> bool {
        use MapItem::*;
        match self {
            Start => true,
            End => true,
            Ground(c) => match other {
                Start => false,
                End => *c == 'y' || *c == 'z',
                Ground(d) => (*c as i32 + 1) >= (*d as i32),
            },
        }
    }
}

impl From<char> for MapItem {
    fn from(c: char) -> Self {
        use MapItem::*;
        match c {
            'S' => Start,
            'E' => End,
            'a'..='z' => Ground(c),
            _ => panic!("invalid item {c}"),
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    // let input = include_str!("sample.txt");
    let grid = parser::parse_grid(input);
    let start = grid.start().unwrap();
    println!("{}", grid);
    println!("Start: {:?}, end: {:?}", grid.start(), grid.end());
    println!("Shortest path: {}", shortest_path(&grid, start));
    println!("Scenic path: {}", scenic_path(&grid));
}

fn shortest_path(map: &Map, start: Pos) -> usize {
    let end = map.end().unwrap();
    let result = dijkstra(&start, |&n| map.neighbors(n), |&n| n == end);
    println!("{:?}", result);
    match result {
        Some(x) => x.1,
        None => 999999999,
    }
}

fn scenic_path(map: &Map) -> usize {
    map.the_a_points()
        .into_iter()
        .map(|start| shortest_path(map, start))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_weight() {
        let input = include_str!("sample.txt");
        let grid = parser::parse_grid(input);
        let start = grid.start().unwrap();
        let weight = shortest_path(&grid, start);
        assert_eq!(31, weight);
    }

    #[test]
    fn test_scenic() {
        let input = include_str!("sample.txt");
        let grid = parser::parse_grid(input);
        let weight = scenic_path(&grid);
        assert_eq!(29, weight);
    }

    #[test]
    fn test_can_step() {
        assert!(MapItem::Ground('c').can_step(&MapItem::Ground('d')));
        assert!(MapItem::Ground('y').can_step(&MapItem::End));
        assert!(MapItem::Ground('z').can_step(&MapItem::End));
        assert!(!MapItem::Ground('c').can_step(&MapItem::Ground('r')));
        assert!(!MapItem::Ground('c').can_step(&MapItem::Ground('t')));
        assert!(!MapItem::Ground('c').can_step(&MapItem::End));
    }
}
