use color_eyre::eyre::eyre;
use nom::combinator::{all_consuming, map};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::{bytes::complete::tag, character::complete as cc};
use nom_locate::LocatedSpan;

type IResult<'a, T> = nom::IResult<Span<'a>, T>;
type Span<'a> = LocatedSpan<&'a str>;

use crate::{Point, Scan};

fn point(i: Span) -> IResult<Point> {
    map(separated_pair(cc::i64, tag(","), cc::i64), Point::from)(i)
}

fn line(i: Span) -> IResult<Scan> {
    map(separated_list1(tag(" -> "), point), |points| Scan(points))(i)
}

pub(crate) fn scan_line(input: &str) -> color_eyre::Result<Scan> {
    all_consuming(line)(input.into())
        .map_err(|e| eyre!("{:?}", e))
        .map(|(_, scan)| scan)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan() -> color_eyre::Result<()> {
        let input = "498,4 -> 498,6 -> 496,6";
        let scan = scan_line(input)?;
        let expected = Scan(vec![Point(498, 4), Point(498, 6), Point(496, 6)]);
        assert_eq!(expected, scan);
        Ok(())
    }
}
