// Monkey 0:
//   Starting items: 79, 98
//   Operation: new = old * 19
//   Test: divisible by 23
//     If true: throw to monkey 2
//     If false: throw to monkey 3

use std::collections::VecDeque;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, newline},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated},
    Finish, IResult,
};

use crate::{Monkey, Operand, Operation};

fn monkey_id(i: &str) -> IResult<&str, u32> {
    terminated(
        delimited(tag("Monkey "), nom::character::complete::u32, tag(":")),
        newline,
    )(i)
}

fn starting_items(i: &str) -> IResult<&str, VecDeque<i64>> {
    terminated(
        preceded(
            tag("  Starting items: "),
            map(
                separated_list1(tag(", "), nom::character::complete::i64),
                VecDeque::from,
            ),
        ),
        newline,
    )(i)
}

fn operand(i: &str) -> IResult<&str, Operand> {
    alt((
        map(tag("old"), |_| Operand::Old),
        map(nom::character::complete::i64, Operand::Num),
    ))(i)
}

fn operation(i: &str) -> IResult<&str, Operation> {
    //   Operation: new = old * 19
    terminated(
        preceded(
            tag("  Operation: new = old "),
            map(separated_pair(anychar, tag(" "), operand), |(c, op)| {
                Operation::from(c, op)
            }),
        ),
        newline,
    )(i)
}

fn parse_test(i: &str) -> IResult<&str, i64> {
    //   Test: divisible by 23
    terminated(
        preceded(tag("  Test: divisible by "), nom::character::complete::i64),
        newline,
    )(i)
}

fn true_monkey(i: &str) -> IResult<&str, u32> {
    //     If true: throw to monkey 2
    terminated(
        preceded(
            tag("    If true: throw to monkey "),
            nom::character::complete::u32,
        ),
        newline,
    )(i)
}

fn false_monkey(i: &str) -> IResult<&str, u32> {
    //     If false: throw to monkey 3
    terminated(
        preceded(
            tag("    If false: throw to monkey "),
            nom::character::complete::u32,
        ),
        newline,
    )(i)
}

fn parse_monkey(i: &str) -> IResult<&str, Monkey> {
    let (i, index) = monkey_id(i)?;
    let (i, items) = starting_items(i)?;
    let (i, operation) = operation(i)?;
    let (i, test_divisor) = parse_test(i)?;
    let (i, true_monkey) = true_monkey(i)?;
    let (i, false_monkey) = false_monkey(i)?;

    let monkey = Monkey {
        index,
        items,
        operation,
        test_divisor,
        true_monkey: true_monkey.try_into().unwrap(),
        false_monkey: false_monkey.try_into().unwrap(),
        inspections: 0,
    };

    Ok((i, monkey))
}

pub(crate) fn parse_monkeys(i: &str) -> color_eyre::Result<Vec<Monkey>> {
    let x = (many1(terminated(parse_monkey, newline))(i)).finish();
    let y = x.ok().unwrap();
    Ok(y.1)
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::bail;

    use super::*;

    #[test]
    fn test_one_monkey() -> color_eyre::Result<()> {
        color_eyre::install()?;

        let input = include_str!("sample.txt");
        let (rem, monkey) = match parse_monkey(input) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("{}", e);
                bail!("fok");
            }
        };

        dbg!(rem);

        assert_eq!(0, monkey.index);

        Ok(())
    }

    #[test]
    fn test_monkey_3() -> color_eyre::Result<()> {
        let input = "Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

        let (rem, monkey) = match parse_monkey(input) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("{}", e);
                bail!("fok");
            }
        };

        dbg!(rem);

        assert_eq!(3, monkey.index);

        Ok(())
    }
}
