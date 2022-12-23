use color_eyre::eyre::eyre;
use nom::combinator::{all_consuming, map};
use nom::sequence::{preceded, tuple};
use nom::Finish;
use nom::{bytes::complete::tag, character::complete as cc};
use nom_locate::LocatedSpan;

type IResult<'a, T> = nom::IResult<Span<'a>, T>;
type Span<'a> = LocatedSpan<&'a str>;

use crate::Sensor;
use crate::point::Point;

fn sensor(i: Span) -> IResult<Point> {
    // Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    map(
        tuple((
            preceded(tag("Sensor at x="), cc::i64),
            preceded(tag(", y="), cc::i64),
        )),
        Point::from,
    )(i)
}

fn beacon(i: Span) -> IResult<Point> {
    // Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    map(
        tuple((
            preceded(tag(": closest beacon is at x="), cc::i64),
            preceded(tag(", y="), cc::i64),
        )),
        Point::from,
    )(i)
}

pub fn line(i: &str) -> color_eyre::Result<Sensor> {
    all_consuming(tuple((sensor, beacon)))(i.into())
        .finish()
        .map_err(|e| eyre!("{:?}", e))
        .map(|(_, sb)| sb)
        .map(Into::into)
}
