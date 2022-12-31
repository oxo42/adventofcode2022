use std::ops::RangeInclusive;

use crate::{point::Point, MAX};

#[derive(Debug)]
pub struct Sensor {
    pub location: Point,
    pub beacon: Point,
    pub exclusion_distance: u64,
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

impl Sensor {
    /// Note: for part 2 so limited to max
    pub fn range_for_row(&self, row: usize) -> Option<RangeInclusive<usize>> {
        let y_dist = row.abs_diff(self.location.1 as usize);
        if y_dist > self.exclusion_distance as usize {
            return None;
        }
        let x_dist = (self.exclusion_distance as usize - y_dist) ;
        let start = self.location.0 - x_dist as i64;
        let start = if start < 0 { 0 } else { start } as usize;
        let end = self.location.0 + x_dist as i64;
        let end = if end as usize > crate::MAX { crate::MAX } else { end as usize};
        Some(start..=end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use test_case::test_case;

    #[test_case(1, Some(5..=11); "in it")]
    #[test_case(16, Some(8..=8); "bottom of range")]
    #[test_case(17, None; "Out of range")]
    #[test_case(7, Some(0..=17); "parts below zero cut off")]
    fn test_2_7(row: usize, expected: Option<RangeInclusive<usize>>) {
        let sensor = Sensor::from((Point(8, 7), Point(2, 10)));
        assert_eq!(expected, sensor.range_for_row(row));
    }
}
