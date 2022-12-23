#![allow(unused)]

use std::fmt::Display;

use grid::Grid;
use itertools::{iproduct, Itertools};
use point::Point;

mod parser;
mod point;

pub struct Sensor {
    location: Point,
    beacon: Point,
    exclusion_distance: u64,
}

impl From<(Point, Point)> for Sensor {
    fn from((location, beacon): (Point, Point)) -> Self {
        let exclusion_distance = location.distance(&beacon);
        Self {
            location,
            beacon,
            exclusion_distance,
        }
    }
}

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

    fn tuning_frequency(&self, max: usize) -> i64 {
        let p = self.brute_force_beacon_search(max).unwrap();
        p.0 * 4000000 + p.1
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
    let max = 4_000_000;
    println!("Tuning freq {}", zone.tuning_frequency(max));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
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
