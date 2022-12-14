use crate::Instruction;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{all_consuming, map},
    sequence::preceded,
    Finish, IResult,
};

fn parse_noop(i: &str) -> IResult<&str, Instruction> {
    map(tag("noop"), |_| Instruction::Noop)(i)
}

fn parse_addx(i: &str) -> IResult<&str, Instruction> {
    map(preceded(tag("addx "), nom::character::complete::i64), |o| {
        Instruction::AddX(o)
    })(i)
}

fn parse_line(i: &str) -> IResult<&str, Instruction> {
    alt((parse_noop, parse_addx))(i)
}

pub(crate) fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map_while(|line| all_consuming(parse_line)(line).finish().ok())
        .map(|(_, i)| i)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_works() -> color_eyre::Result<()> {
        color_eyre::install()?;
        let x = parse_addx("addx 15");
        assert_eq!(Instruction::AddX(15), x?.1);
        Ok(())
    }
}
