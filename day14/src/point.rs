use std::{array::IntoIter, ops::Add};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Point(pub i64, pub i64); // x,y == col,row

impl Point {
    pub fn row(&self) -> usize {
        self.1 as usize
    }

    pub fn col(&self) -> usize {
        self.0 as usize
    }
}

#[derive(Debug)]
pub struct PointPair(pub Point, pub Point);

impl From<(i64, i64)> for Point {
    fn from((x, y): (i64, i64)) -> Self {
        Point(x, y)
    }
}

impl TryFrom<&[Point]> for PointPair {
    type Error = color_eyre::Report;

    fn try_from(points: &[Point]) -> Result<Self, Self::Error> {
        if let [p1, p2] = points {
            Ok(PointPair(p1.clone(), p2.clone()))
        } else {
            Err(color_eyre::eyre::eyre!("Invalid number of points"))
        }
    }
}

impl IntoIterator for PointPair {
    type Item = Point;
    type IntoIter = PointIterator;

    fn into_iter(self) -> Self::IntoIter {
        PointIterator {
            p1: self.0.clone(),
            p2: self.1.clone(),
            current: self.0.clone(),
            finished: false,
        }
    }
}

impl Add<(i64, i64)> for Point {
    type Output = Point;

    fn add(self, (dx, dy): (i64, i64)) -> Self::Output {
        Point(self.0 + dx, self.1 + dy)
    }
}

pub struct PointIterator {
    p1: Point,
    p2: Point,
    current: Point,
    finished: bool,
}

impl PointIterator {
    fn direction(&self) -> (i64, i64) {
        let dx = if self.p1.0 == self.p2.0 {
            0
        } else if self.p1.0 > self.p2.0 {
            -1
        } else {
            1
        };
        let dy = if self.p1.1 == self.p2.1 {
            0
        } else if self.p1.1 > self.p2.1 {
            -1
        } else {
            1
        };

        (dx, dy)
    }
}

impl Iterator for PointIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            None
        } else {
            if self.current == self.p2 {
                self.finished = true;
            }
            let current = self.current;
            let next = self.current + self.direction();
            self.current = next;
            Some(current)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_iterator() {
        let pp = PointPair(Point(1, 1), Point(3, 1));
        let mut i = pp.into_iter();
        assert_eq!(Some(Point(1, 1)), i.next());
        assert_eq!(Some(Point(2, 1)), i.next());
        assert_eq!(Some(Point(3, 1)), i.next());
        assert_eq!(None, i.next());
    }

    #[test]
    fn test_point_iterator_reverse() {
        let pp = PointPair(Point(1, 3), Point(1, 1));
        let mut i = pp.into_iter();
        assert_eq!(Some(Point(1, 3)), i.next());
        assert_eq!(Some(Point(1, 2)), i.next());
        assert_eq!(Some(Point(1, 1)), i.next());
        assert_eq!(None, i.next());
    }
}
