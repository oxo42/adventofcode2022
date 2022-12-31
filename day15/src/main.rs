#![allow(unused)]

use std::{fmt::Display, ops::RangeInclusive};

use grid::Grid;
use itertools::{iproduct, Itertools};
use point::Point;
use sensor::Sensor;

mod parser;
mod point;
mod sensor;

#[cfg(not(test))]
const MAX: usize = 4_000_000;
#[cfg(test)]
const MAX: usize = 20;

struct Zone {
    sensors: Vec<Sensor>,
}

impl Zone {
    fn new(sensors: Vec<Sensor>) -> Self {
        Self { sensors }
    }

    fn is_clear(&self, point: &Point) -> bool {
        self.sensors
            .iter()
            .any(|sensor| point.distance(&sensor.location) <= sensor.exclusion_distance)
    }

    fn num_clear_on_row(&self, row: i64) -> usize {
        let closest_sensor = self
            .sensors
            .iter()
            .map(|sensor| sensor.location.clone())
            .min_by(|a, b| {
                let x = a.0.abs_diff(b.0) as i64;
                let t = Point(x / 2, row);
                let a_dist = a.distance(&t);
                let b_dist = b.distance(&t);
                a_dist.cmp(&b_dist)
            })
            .unwrap();
        let starting_point = Point(closest_sensor.0, row);
        if !self.is_clear(&starting_point) {
            panic!("Expected starting point to be clear.  Go think again");
        }

        let right_start = starting_point.0 + 1;
        let right_count = (right_start..)
            .take_while(|&x| self.is_clear(&Point(x, row)))
            .count();
        let mut left_index = starting_point.0 + 1;
        loop {
            if !self.is_clear(&Point(left_index, row)) {
                break;
            }
            left_index -= 1;
        }
        let left_count = (starting_point.0 - left_index) as usize;

        let beacons_on_row = self
            .sensors
            .iter()
            .map(|sensor| sensor.beacon.clone())
            .unique()
            .filter(|b| b.1 == row)
            .count();

        let count = left_count + right_count - beacons_on_row;

        count
    }

    fn brute_force_beacon_search(&self, max: usize) -> Option<Point> {
        iproduct!(0..max, 0..max)
            .into_iter()
            .map(|(x, y)| Point(x as i64, y as i64))
            .inspect(|p| {
                if p.0 % 1_000 == 0 && p.1 % 1_000 == 0 {
                    println!("{p:?}");
                }
            })
            .find(|p| !self.is_clear(p))
    }

    fn ranges_for_row(&self, row: usize) -> Vec<RangeInclusive<usize>> {
        self.sensors
            .iter()
            .filter_map(|s| s.range_for_row(row)) //
            .sorted_by(|a, b| a.start().cmp(b.start()))
            .coalesce(|a, b| {
                if a.contains(&b.start()){
                    let start = a.start().min(b.start());
                    let end = a.end().max(b.end());
                    Ok(*start..=*end)
                } else {
                    Err((a, b))
                }
            })
            .collect()
    }

    fn tuning_frequency(&self, max: usize) -> i64 {
        let row = (0..MAX)
            .find(|row| {
                let r = self.ranges_for_row(*row);
                r.len() > 1
            })
            .expect("ranges to exist");
        let ranges = self.ranges_for_row(row);
        let x = ranges[0].end()+1;
        let p = Point(x as i64,row as i64);
        // let p = self.brute_force_beacon_search(max).unwrap();
        p.tuning_frequency()
    }
}

fn load_zone(input: &str) -> color_eyre::Result<Zone> {
    let lines: Vec<_> = input.lines().map(parser::line).try_collect()?;
    Ok(Zone::new(lines))
}

fn main() -> color_eyre::Result<()> {
    let input = include_str!("input.txt");
    let zone = load_zone(input)?;

    println!("Num clear on row {}", zone.num_clear_on_row(2_000_000));
    println!("Tuning freq {}", zone.tuning_frequency(MAX));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_for_sample_11() -> color_eyre::Result<()> {
        let input = include_str!("sample.txt");
        let zone = load_zone(input)?;
        let r = zone.ranges_for_row(11);
        assert_eq!(vec![(0..=13),(15..=20)], r);
        Ok(())
    }

    #[test]
    fn test_zone_checks_out() -> color_eyre::Result<()> {
        let input = include_str!("sample.txt");
        let zone = load_zone(input)?;
        assert!(zone.is_clear(&Point(-2, 10)));
        assert!(zone.is_clear(&Point(2, 10)));
        assert!(zone.is_clear(&Point(24, 10)));

        assert!(!zone.is_clear(&Point(-3, 10)));
        assert!(!zone.is_clear(&Point(25, 10)));
        Ok(())
    }

    #[test]
    fn test_sample() -> color_eyre::Result<()> {
        let input = include_str!("sample.txt");
        let zone = load_zone(input)?;
        assert_eq!(26, zone.num_clear_on_row(10));
        Ok(())
    }

    #[test]
    fn test_tuning_freq() -> color_eyre::Result<()> {
        let input = include_str!("sample.txt");
        let zone = load_zone(input)?;
        assert_eq!(56_000_011, zone.tuning_frequency(20));
        Ok(())
    }
}
