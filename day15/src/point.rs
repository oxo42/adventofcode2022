use std::fmt::Debug;

#[derive( PartialEq, Eq, Hash, Clone)]
pub struct Point(pub i64, pub i64); // x, y

impl Point {
    /// Manhattan distance
    pub fn distance(&self, other: &Point) -> u64 {
        let dx = self.0.abs_diff(other.0);
        let dy = self.1.abs_diff(other.1);
        dx + dy
    }

    pub fn tuning_frequency(&self) -> i64 {
        self.0 * 4000000 + self.1
    }
}

impl From<(i64, i64)> for Point {
    fn from((x, y): (i64, i64)) -> Self {
        Point(x, y)
    }
}

impl Debug for Point{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}