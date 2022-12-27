use std::{collections::HashMap, hash::Hash};

use itertools::Itertools;
use pathfinding::prelude::dijkstra;
use regex::{Match, Regex};

#[derive(Debug, Clone, Eq)]
struct Valve {
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
    valves: HashMap<String, Valve>,
    pub pressure_released: usize,
    time_remaining: usize,
    current: String,
    moving: bool,
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
                        let ts = tunnels.as_str().split('\n').map(String::from).collect_vec();
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
            time_remaining: 0,
            moving: false,
        })
    }

    fn open_valves_pressure(&self) -> usize {
        self.valves
            .iter()
            .filter(|v| v.1.open)
            .map(|v| v.1.rate)
            .sum()
    }

    fn distance_between(&self, start: &Valve, end: &Valve) -> Option<usize> {
        let path = dijkstra(
            start,
            |valve| {
                let tunnels = &self.valves.get(&valve.name).unwrap().tunnels;
                self.valves
                    .iter()
                    .filter(|(name, _v)| tunnels.contains(name))
                    .map(|v| (v.1.clone(), 2))
            },
            |v| v == end,
        );
        path.map(|p| p.1)
    }
    
    fn path_to(&self, start: &Valve, end: &Valve) -> Option<usize> {
        let path = dijkstra(
            start,
            |valve| {
                let tunnels = &self.valves.get(&valve.name).unwrap().tunnels;
                self.valves
                    .iter()
                    .filter(|(name, _v)| tunnels.contains(name))
                    .map(|v| (v.1.clone(), 2))
            },
            |v| v == end,
        );
        path.map(|p| p.1)
    }


    fn next_valve(&self) -> String {
        let other_valves = self
            .valves
            .iter()
            .filter(|(name, _)| **name != self.current)
            .map(|(_, v)| v.clone())
            .collect_vec();
        let current = self.valves.get(&self.current).unwrap();
        let closest = other_valves
            .iter() //
            .filter_map(|v| self.distance_between(&current, v))
            .inspect(|v| println!("{v:?}"))
            .position_max()
            .unwrap();

        todo!()
    }

    pub fn release_pressure(&mut self) {
        while self.time_remaining > 0 {
            self.time_remaining -= 1;
            self.pressure_released += self.open_valves_pressure();
            if self.moving {
                self.moving = false;
            } else {
                self.moving = true;
                self.curent = self.next_valve();
            }
        }
    }
}
