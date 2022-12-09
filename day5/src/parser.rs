#![allow(unused)]
use crate::Dock;
use crate::Move;
use std::collections::HashMap;
use std::collections::VecDeque;

use color_eyre::eyre;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::newline;
use nom::character::complete::one_of;
use nom::character::complete::u32;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::combinator::opt;
use nom::multi::many0;
use nom::multi::many0_count;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;

use crate::Crate;

fn crate_there(input: &str) -> IResult<&str, Option<Crate>> {
    map(delimited(tag("["), anychar, tag("]")), |c| Some(Crate(c)))(input)
}
fn crate_missing(input: &str) -> IResult<&str, Option<Crate>> {
    map(tag("   "), |_| None)(input)
}

fn opt_crate(input: &str) -> IResult<&str, Option<Crate>> {
    alt((crate_there, crate_missing))(input)
}

pub fn dock_line(input: &str) -> IResult<&str, Vec<Option<Crate>>> {
    many0(terminated(opt_crate, opt(tag(" "))))(input)
}

fn num(input: &str) -> IResult<&str, usize> {
    map(u32, |n| n as _)(input)
}
fn pile_num(input: &str) -> IResult<&str, usize> {
    map(num, |n| n - 1)(input)
}

pub fn parse_mv(input: &str) -> IResult<&str, Move> {
    map_res(
        tuple((
            preceded(tag("move "), num),
            preceded(tag(" from "), pile_num),
            preceded(tag(" to "), pile_num),
        )),
        Move::try_from,
    )(input)
}

#[cfg(test)]
mod tests {
    use crate::tests::INPUT;
    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;

    #[test]
    fn test_parse_move() -> color_eyre::Result<()> {
        let input = "move 1 from 2 to 1\n";
        let x = parse_mv(input)?;
        assert_eq!(Move::new(1, 2, 1), x.1);
        Ok(())
    }
}
