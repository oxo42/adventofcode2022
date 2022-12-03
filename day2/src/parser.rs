use std::fs;

use nom::{
    character::complete::{char, newline, one_of, space1},
    combinator::map,
    multi::many1,
    sequence::{separated_pair, terminated},
    IResult,
};

use crate::game::Game;

fn one_line(input: &str) -> IResult<&str, Game> {
    map(
        separated_pair(one_of("ABC"), space1, one_of("XYZ")),
        |(e, m)| Game::from_chars(e, m),
    )(input)
}

fn whole_file(input: &str) -> IResult<&str, Vec<Game>> {
    many1(terminated(one_line, newline))(input)
}

pub fn games(file_path: &str) -> eyre::Result<Vec<Game>> {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    match whole_file(&contents) {
        Ok((_, games)) => Ok(games),
        Err(e) => {
            eprintln!("Error: {e:?}");
            Err(eyre::eyre!("Went boom"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::{Game, Hand};
    const INPUT: &str = "B X\nA Z\nA Y\n";

    #[test]
    fn check_one_line_works() -> eyre::Result<()> {
        let x = one_line(INPUT)?;
        assert_eq!(Game::from_chars('B', 'X'), x.1);
        Ok(())
    }

    #[test]
    fn check_all() -> eyre::Result<()> {
        let x = whole_file(INPUT)?;
        let games = x.1;
        assert_eq!(games.len(), 3);
        assert_eq!(
            vec![
                Game::from_chars('B', 'X'),
                Game::from_chars('A', 'Z'),
                Game::from_chars('A', 'Y'),
            ],
            games
        );

        Ok(())
    }
}
