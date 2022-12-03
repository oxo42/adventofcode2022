use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, newline, digit1, multispace0, one_of},
    combinator::{eof, map, map_res, recognize},
    multi::{many0, many1, many_till},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

fn integer(input: &str) -> IResult<&str, i64> {
    map_res(digit1, |s: &str| s.parse::<i64>())(input)
}

fn one_elf(input: &str) -> IResult<&str, i64> {
    map(many0(terminated(integer, newline)), |v| v.into_iter().sum())(input)
}

pub fn parse_whole_file(input: &str) -> IResult<&str, Vec<i64>> {
    many1(terminated(one_elf, newline))(input)
}

// pub fn x_parse_whole_file(input: &str) -> IResult<&str, Vec<i64>> {
//     map(tuple((almost_whole_file, one_elf)), |(mut v, e)| {
//         v.push(e);
//         v
//     })(input)
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int_works() -> eyre::Result<()> {
        let input = "1000\n2000\n4000\n\n500\n\n";
        let x = integer(input)?;
        assert_eq!("\n2000\n4000\n\n500\n\n", x.0);
        assert_eq!(1000, x.1);
        Ok(())
    }

    #[test]
    fn test_one_elf() -> eyre::Result<()> {
        let input = "1000\n2000\n4000\n\n500\n\n";
        let full_line = one_elf(input);
        assert_eq!(7000, full_line?.1);
        Ok(())
    }

    #[test]
    fn test_whole_file() -> eyre::Result<()> {
        let input = "1000\n2000\n4000\n\n500\n100\n\n";
        let expected = vec![7000, 600];
        let x = parse_whole_file(input);
        dbg!(&x);
        assert_eq!(expected, x?.1);
        Ok(())
    }
}
