use crate::Jet;
use std::ops::Add;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
}

impl Pos {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl Add<(isize, isize)> for Pos {
    type Output = Pos;

    fn add(self, (x, y): (isize, isize)) -> Self::Output {
        Pos::new(&self.x + x, &self.y + y)
    }
}

impl Add<&Jet> for Pos {
    type Output = Pos;

    fn add(self, jet: &Jet) -> Self::Output {
        match jet {
            Jet::Left => self + (-1, 0),
            Jet::Right => self + (1, 0),
        }
    }
}
