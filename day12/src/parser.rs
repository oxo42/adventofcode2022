use crate::{Map, MapItem};
use grid::Grid;
use nom::{
    character::complete::one_of,
    combinator::{all_consuming, map},
    multi::many1,
    Finish, IResult,
};

fn parse_char(i: &str) -> IResult<&str, MapItem> {
    map(one_of("abcdefghijklmnopqrstuvwxyzSE"), Into::into)(i)
}

fn parse_line(i: &str) -> IResult<&str, Vec<MapItem>> {
    many1(parse_char)(i)
}

pub(crate) fn parse_grid(input: &str) -> Map {
    let v: Vec<_> = input
        .lines()
        .map_while(|line| all_consuming(parse_line)(line).finish().ok())
        .map(|(_, line)| line)
        .collect();
    let cols = v.first().unwrap().len();
    Map(Grid::from_vec(v.into_iter().flatten().collect(), cols))
}
