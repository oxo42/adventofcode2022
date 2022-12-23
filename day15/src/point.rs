#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Point(pub i64, pub i64); // x, y

impl Point {
    /// Manhattan distance
    pub fn distance(&self, other: &Point) -> u64 {
        let dx = self.0.abs_diff(other.0);
        let dy = self.1.abs_diff(other.1);
        dx + dy
    }
}

impl From<(i64, i64)> for Point {
    fn from((x, y): (i64, i64)) -> Self {
        Point(x, y)
    }
}
