use std::collections::HashMap;

use color_eyre::eyre::eyre;
use itertools::Itertools;
use regex::{Match, Regex};

#[derive(Debug)]
struct Maze(HashMap<String, Valve>);

impl Maze {
    fn parse(input: &str) -> color_eyre::Result<Maze> {
        let re =
            Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.+)").unwrap();

        let map = input
            .lines()
            .map(|line| {
                re.captures(line)
                    .expect(line)
                    .captures_iter()
                    .inspect(|s| println!("{s:?}"))
                    .collect_tuple::<(_, _, _)>()
                    .map(Valve::try_from)
            })
            .flatten()
            .map(|v| {
                let v = v.unwrap();
                (v.valve.clone(), v)
            })
            .collect::<HashMap<_, _>>();
        Ok(Maze(map))
    }
}

#[derive(Debug)]
struct Valve {
    valve: String,
    rate: usize,
    tunnels: Vec<String>,
}

impl TryFrom<(Option<Match<'_>>, Option<Match<'_>>, Option<Match<'_>>)> for Valve {
    type Error = color_eyre::Report;

    fn try_from(
        (valve, rate, tunnels): (Option<Match>, Option<Match>, Option<Match>),
    ) -> Result<Self, Self::Error> {
        let valve = valve.ok_or(eyre!("no valve"))?.as_str().into();
        let rate = rate.ok_or(eyre!("No rate"))?.as_str().parse()?;
        let tunnels = tunnels
            .ok_or(eyre!("No tunnels"))?
            .as_str()
            .split(", ")
            .map(|t| t.to_string())
            .collect_vec();
        Ok(Valve {
            valve,
            rate,
            tunnels,
        })
    }
}

fn main() -> color_eyre::Result<()> {
    let input = include_str!("sample.txt");
    let maze = Maze::parse(input)?;
    dbg!(maze);
    // maze.0.iter().for_each(|v| println!("{v:?}"));
    Ok(())
}
