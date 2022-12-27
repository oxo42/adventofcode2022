#![allow(unused)]
mod maze;

use maze::Maze;

fn main() -> color_eyre::Result<()> {
    let input = include_str!("sample.txt");
    let mut maze = Maze::parse(input)?;
    maze.release_pressure();

    println!("Pressure released: {}", maze.pressure_released);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part_1() -> color_eyre::Result<()> {
        let mut maze = Maze::parse(include_str!("sample.txt"))?;
        maze.release_pressure();
        assert_eq!(1651, maze.pressure_released);
        Ok(())
    }
}
