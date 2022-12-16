use color_eyre::eyre::eyre;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{self as cc, newline};
use nom::combinator::{all_consuming, map};
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{preceded, separated_pair, terminated};
use nom::Finish;
use nom_locate::LocatedSpan;

type Span<'a> = LocatedSpan<&'a str>;
type IResult<'a, T> = nom::IResult<Span<'a>, T>;

use crate::{Item, Pair};

fn item_num(i: Span) -> IResult<Item> {
    map(cc::i64, Item::Value)(i)
}

fn item(i: Span) -> IResult<Item> {
    alt((item_list, item_num))(i)
}

fn item_list(i: Span) -> IResult<Item> {
    map(
        terminated(
            preceded(tag("["), separated_list0(tag(","), item)),
            tag("]"),
        ),
        Item::from,
    )(i)
}

fn parse_line(line: &str) -> Option<Item> {
    all_consuming(item_list)(line.into())
        .finish()
        .map(|(_, items)| items)
        .ok()
}

fn parse_pair(i: Span) -> IResult<Pair> {
    terminated(
        map(separated_pair(item_list, newline, item_list), Pair::from),
        newline,
    )(i)
}

pub(crate) fn parse_input(input: &str) -> color_eyre::Result<Vec<Pair>> {
    all_consuming(separated_list1(newline, parse_pair))(input.into())
        .finish()
        .map_err(|e| eyre!("{:?}", e))
        .map(|(_, p)| p)
        

    // let pairs: Vec<_> = input.lines().map_while(parse_line).collect();
    // pairs
    //     .into_iter()
    //     .collect_tuple()
    //     .map(|(a, b)| Pair::from((a, b)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use color_eyre::eyre::eyre;
    use nom::Finish;
    use test_case::test_case;
    use Item::*;

    #[test_case("[1,2,3]", vec![1,2,3].into())]
    #[test_case("[]", Collection(vec![].into()))]
    fn test_list_of_items(input: &str, items: Item) -> color_eyre::Result<()> {
        let (_, actual) = item_list(input.into()).finish().map_err(|e| {
            eprintln!("{:?}", e);
            eyre!("{:?}", e)
        })?;
        assert_eq!(items, actual);
        Ok(())
    }
}
