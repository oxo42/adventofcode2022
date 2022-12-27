use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
};

use itertools::Itertools;
use pathfinding::prelude::dijkstra;
use regex::{Match, Regex};

#[derive(Debug, Clone, Eq)]
pub struct Valve {
    name: String,
    rate: usize,
    tunnels: Vec<String>,
    open: bool,
}

impl Hash for Valve {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.rate.hash(state);
        self.tunnels.hash(state);
        self.open.hash(state);
    }
}

impl PartialEq for Valve {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Valve {
    fn new(name: String, rate: usize, tunnels: Vec<String>) -> Self {
        Self {
            name,
            rate,
            tunnels,
            open: false,
        }
    }
}

#[derive(Debug)]
pub struct Maze {
    pub valves: HashMap<String, Valve>,
    pub pressure_released: usize,
    time_remaining: usize,
    current: String,
    path: Option<VecDeque<String>>,
}

impl Maze {
    pub fn parse(input: &str) -> color_eyre::Result<Maze> {
        let re = Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.+)")
            .unwrap();

        let mut valves: HashMap<String, Valve> = input
            .lines()
            .filter_map(|line| {
                re.captures(line)
                    .expect(line)
                    .iter()
                    .skip(1)
                    .flatten()
                    .collect_tuple::<(_, _, _)>()
                    .map(|(name, rate, tunnels)| {
                        let ts = tunnels.as_str().split(", ").map(String::from).collect_vec();
                        Valve::new(
                            name.as_str().to_string(),
                            str::parse::<usize>(rate.as_str()).unwrap(),
                            ts,
                        )
                    })
            })
            .map(|v| (v.name.clone(), v))
            .collect::<HashMap<_, _>>();

        Ok(Maze {
            valves,
            pressure_released: 0,
            current: "AA".to_string(),
            time_remaining: 30,
            path: None,
        })
    }

    fn open_valves_pressure(&self) -> usize {
        self.valves
            .values()
            .filter(|v| v.open)
            .map(|v| v.rate)
            .sum()
    }

    fn distance_to(&self, dest: &str) -> usize {
        self.path_to(dest).1
    }

    fn path_to(&self, dest: &str) -> (Vec<String>, usize) {
        let start = &self.current;
        let path = dijkstra(
            start,
            |valve| {
                let tunnels = &self.valves.get(valve).unwrap().tunnels;
                tunnels.iter().map(|t| (t.clone(), 1))
            },
            |v| v == dest,
        );
        let (p, c) = path.unwrap_or_else(|| panic!("path to {} to exist", dest));
        // skip first node
        (p.into_iter().skip(1).collect(), c)
    }

    fn next_valve(&self) -> String {
        // best valve is most pressure over time = (current time - time to move) * rate
        // is_open
        let start = self.valves.get(&self.current).unwrap().clone();
        let x = self
            .valves
            .values() //
            .filter(|v| !v.open && v.rate > 0)
            .map(|v| {
                (
                    v.name.clone(),
                    (self.time_remaining - self.distance_to(&v.name)) * v.rate,
                )
            })
            .inspect(|v| println!("{} will release {}", v.0, v.1))
            .max_by(|a, b| a.1.cmp(&b.1));
        x.unwrap().0
    }

    fn open_current(&mut self) {
        self.valves.get_mut(&self.current).unwrap().open = true;
    }

    pub fn release_pressure(&mut self) {
        while self.time_remaining > 0 {
            self.time_remaining -= 1;
            self.pressure_released += self.open_valves_pressure();
            if self.path.is_some() {
                // we're moving
                if let Some(next) = self.path.as_mut().unwrap().pop_front() {
                    println!("Moving to {next}");
                    self.current = next;
                } else {
                    self.path = None; // next cycle find a new path
                    self.open_current();
                }
            } else {
                let next_valve = self.next_valve();
                println!("Going to open {next_valve}");
                let path: VecDeque<_> = VecDeque::from(self.path_to(&next_valve).0);
                self.path = Some(path);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_path() -> color_eyre::Result<()> {
        let mut maze = Maze::parse(include_str!("sample.txt"))?;

        assert_eq!((vec!["DD".to_string()], 1), maze.path_to("DD"));

        maze.current = "DD".into();

        assert_eq!(
            (vec!["CC".to_string(), "BB".to_string()], 2),
            maze.path_to("BB")
        );

        // assert_eq!()
        Ok(())
    }
}
