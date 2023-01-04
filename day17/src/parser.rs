use nom::{
    branch::alt,
    character::complete as ncc,
    combinator::{all_consuming, map},
    multi::many1,
    Finish,
};

use crate::Jet;

type Span<'a> = nom_locate::LocatedSpan<&'a str>;
type IResult<'a, T> = nom::IResult<Span<'a>, T>;

fn left(i: Span) -> IResult<Jet> {
    map(ncc::char('<'), |_| Jet::Left)(i)
}

fn right(i: Span) -> IResult<Jet> {
    map(ncc::char('>'), |_| Jet::Right)(i)
}

fn line(i: Span) -> IResult<Vec<Jet>> {
    many1(alt((left, right)))(i)
}

pub fn parse_line(i: &str) -> color_eyre::Result<Vec<Jet>> {
    let i = i.strip_suffix("\n").unwrap();
    all_consuming(line)(i.into())
        .finish()
        .map_err(|e| color_eyre::eyre::eyre!("{e:?}"))
        .map(|(_, v)| v)
}
