use std::ops::RangeInclusive;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::{map, map_res},
    multi::many1,
    sequence::{separated_pair, terminated},
    IResult,
};

pub trait ContainsExt {
    fn contains_range(&self, other: &Self) -> bool;

    fn contains_or_is_contained(&self, other: &Self) -> bool {
        self.contains_range(other) || other.contains_range(self)
    }

    fn overlaps_range(&self, other: &Self) -> bool;

    fn overlaps_or_is_overlapped(&self, other: &Self) -> bool {
        self.overlaps_range(other) || other.overlaps_range(self)
    }
}

impl<T> ContainsExt for RangeInclusive<T>
where
    T: PartialOrd,
{
    fn contains_range(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }

    fn overlaps_range(&self, other: &Self) -> bool {
        self.contains(other.start()) || self.contains(other.end())
    }
}

fn nom_u32(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

fn range(input: &str) -> IResult<&str, RangeInclusive<u32>> {
    map(separated_pair(nom_u32, tag("-"), nom_u32), |(s, e)| (s..=e))(input)
}

fn line(input: &str) -> IResult<&str, (RangeInclusive<u32>, RangeInclusive<u32>)> {
    separated_pair(range, tag(","), range)(input)
}

fn file(input: &str) -> IResult<&str, Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>> {
    many1(terminated(line, newline))(input)
}

pub fn parse_file(
    input: &str,
) -> color_eyre::Result<Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>> {
    match file(&input) {
        Ok((_, games)) => Ok(games),
        Err(e) => {
            eprintln!("Error: {e:?}");
            Err(eyre::eyre!("Went boom"))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::INPUT;

    use super::*;

    #[test]
    fn test_parse_pair() -> color_eyre::Result<()> {
        let x = line(INPUT)?;
        assert_eq!(((2..=4), (6..=8)), x.1);
        Ok(())
    }

    #[test]
    fn test_overlaps() {
        assert!((2..=8).contains_or_is_contained(&(3..=7)));
        assert!((6..=6).contains_or_is_contained(&(4..=6)));

        // not overlaps
        assert!(!(2..=4).contains_or_is_contained(&(6..=8)));
        assert!(!(5..=7).contains_or_is_contained(&(7..=9)));
    }

    #[test]
    fn test_sample_data() -> eyre::Result<()> {
        Ok(())
    }
}
