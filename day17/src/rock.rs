use num_derive::{FromPrimitive, ToPrimitive};

use crate::pos::Pos;

#[derive(Debug, Eq, PartialEq, FromPrimitive, ToPrimitive, Clone, Copy)]
pub enum Rock {
    Dash = 0,
    Plus = 1,
    Elle = 2,
    I = 3,
    Box = 4,
}

impl Rock {
    pub fn height(&self) -> isize {
        use Rock::*;
        match self {
            Dash => 1,
            Plus => 3,
            Elle => 3,
            I => 4,
            Box => 2,
        }
    }

    pub fn places_iter(&self) -> impl Iterator<Item = (isize, isize)> {
        use Rock::*;
        match self {
            Dash => vec![(0, 0), (1, 0), (2, 0), (3, 0)].into_iter(),
            Plus => vec![(1, 0), (1, -1), (1, -2), (0, -1), (2, -1)].into_iter(),
            Elle => vec![(2, 0), (2, -1), (2, -2), (1, -2), (0, -2)].into_iter(),
            I => vec![(0, 0), (0, -1), (0, -2), (0, -3)].into_iter(),
            Box => vec![(0, 0), (0, -1), (1, 0), (1, -1)].into_iter(),
        }
    }

    pub fn pos_iter(&self, pos: Pos) -> impl Iterator<Item = Pos> {
        self.places_iter().map(move |p| pos + p)
    }
}

pub struct RockPlaceIter {
    rock: Rock,
}

// impl Iterator for RockPlaceIter {
//     type Item = (isize,isize);

//     fn next(&mut self) -> Option<Self::Item> {
//         match self.rock
//     }
// }

pub struct RockIterator {
    item: Rock,
}

impl RockIterator {
    pub fn new() -> Self {
        RockIterator { item: Rock::Box }
    }
}

impl Iterator for RockIterator {
    type Item = Rock;

    fn next(&mut self) -> Option<Self::Item> {
        let mut i = self.item as u8;
        i += 1;
        if i > 4 {
            i = 0;
        }
        self.item = num::FromPrimitive::from_u8(i).unwrap();
        Some(self.item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rock_iterator() {
        use Rock::*;

        let mut iter = RockIterator::new();
        assert_eq!(iter.next(), Some(Dash));
        assert_eq!(iter.next(), Some(Plus));
        assert_eq!(iter.next(), Some(Elle));
        assert_eq!(iter.next(), Some(I));
        assert_eq!(iter.next(), Some(Box));
        assert_eq!(iter.next(), Some(Dash));
        assert_eq!(iter.next(), Some(Plus));
        assert_eq!(iter.next(), Some(Elle));
        assert_eq!(iter.next(), Some(I));
        assert_eq!(iter.next(), Some(Box));
    }
}
