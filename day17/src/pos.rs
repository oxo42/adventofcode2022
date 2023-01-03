use std::ops::Add;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl Add<(usize, usize)> for Pos {
    type Output = Pos;

    fn add(self, (x, y): (usize, usize)) -> Self::Output {
        Pos::new(&self.x + x, &self.y + y)
    }
}
