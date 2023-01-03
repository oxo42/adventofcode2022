use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Debug, Eq, PartialEq, FromPrimitive, ToPrimitive, Clone, Copy)]
pub enum Rock {
    Dash = 0,
    Plus = 1,
    Elle = 2,
    I = 3,
    Box = 4,
}
impl Rock {
    pub fn height(&self) -> usize {
        use Rock::*;
        match self {
            Dash => 1,
            Plus => 3,
            Elle => 3,
            I => 4,
            Box => 2,
        }
    }
}

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
