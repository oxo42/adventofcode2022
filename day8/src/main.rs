use std::fmt::Display;

use grid::Grid;
#[macro_use]
extern crate itertools;

const INPUT: &str = "30373
25512
65332
33549
35390
";

fn parse_grid(input: &str) -> Grid<u8> {
    let mut grid: Grid<u8> = Grid::new(0, 0);
    for line in input.lines() {
        let row = line.chars().map(|c| c as u8 - b'0').collect();
        grid.push_row(row);
    }
    grid
}

fn print_grid<T: Display>(grid: &Grid<T>) {
    for r in 0..grid.rows() {
        for c in 0..grid.cols() {
            print!("{}", grid.get(r, c).unwrap());
        }
        println!();
    }
}

fn is_tree_visible<T: PartialOrd>(grid: &Grid<T>, row: usize, col: usize) -> bool {
    let tree = grid.get(row, col).unwrap();
    let xplus = (row + 1..grid.rows()).all(|r| tree > grid.get(r, col).unwrap());
    let xmin = (0..=row - 1).all(|r| tree > grid.get(r, col).unwrap());
    let yplus = (col + 1..grid.cols()).all(|c| tree > grid.get(row, c).unwrap());
    let ymin = (0..=col - 1).all(|c| tree > grid.get(row, c).unwrap());
    // dbg!(row, col, xplus, xmin, yplus, ymin);

    xplus || xmin || yplus || ymin
}

fn tree_scenic_score(grid: &Grid<u8>, row: usize, col: usize) -> u64 {
    let tree = grid.get(row, col).unwrap();

    let up = {
        let mut total = 0;
        for row in (0..=row - 1).rev() {
            total += 1;
            if tree <= grid.get(row, col).unwrap() {
                break;
            }
        }
        total
    };
    let down = {
        let mut total = 0;
        for row in row + 1..grid.rows() {
            total += 1;
            if tree <= grid.get(row, col).unwrap() {
                break;
            }
        }
        total
    };
    let left = {
        let mut total = 0;
        for col in (0..=col - 1).rev() {
            total += 1;
            if tree <= grid.get(row, col).unwrap() {
                break;
            }
        }
        total
    };
    let right = {
        let mut total = 0;
        for col in col + 1..grid.cols() {
            total += 1;
            if tree <= grid.get(row, col).unwrap() {
                break;
            }
        }
        total
    };

    up * down * left * right
}

fn count_visible_trees<T>(grid: &Grid<T>) -> usize
where
    T: PartialOrd,
{
    let mut count = 0usize;
    for r in 1..grid.rows() - 1 {
        for c in 1..grid.cols() - 1 {
            if is_tree_visible(grid, r, c) {
                count += 1;
            }
        }
    }
    count + grid.rows() * 2 + grid.cols() * 2 - 4
}

fn main() -> color_eyre::Result<()> {
    // let grid = parse_grid(INPUT);
    let grid = parse_grid(include_str!("../input.txt"));
    // print_grid(&grid);

    dbg!(count_visible_trees(&grid));

    let most_scenic = iproduct!(1..grid.rows(), 1..grid.cols())
        .map(|(row, col)| tree_scenic_score(&grid, row, col))
        .max();

    dbg!(most_scenic);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schenic_scores() {
        let grid = parse_grid(INPUT);
        print_grid(&grid);
        assert_eq!(4, tree_scenic_score(&grid, 1, 2));
        assert_eq!(8, tree_scenic_score(&grid, 3, 2));
    }

    #[test]
    fn test_is_visible() {
        let grid = parse_grid(INPUT);
        print_grid(&grid);
        assert!(is_tree_visible(&grid, 1, 1));
        assert!(is_tree_visible(&grid, 1, 2));
        assert!(!is_tree_visible(&grid, 1, 3));
        assert!(is_tree_visible(&grid, 2, 1));
        assert!(!is_tree_visible(&grid, 2, 2));
        assert!(is_tree_visible(&grid, 2, 3));
        assert!(!is_tree_visible(&grid, 3, 1));
        assert!(is_tree_visible(&grid, 3, 2));
        assert!(!is_tree_visible(&grid, 3, 3));
    }
}
