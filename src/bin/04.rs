advent_of_code::solution!(4);

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use itertools::Itertools;
use itertools::iproduct;

#[derive(Debug, PartialEq, Eq)]
enum Cell {
    Empty,
    Paper,
}

impl TryFrom<char> for Cell {
    type Error = Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            '@' => Ok(Self::Paper),
            '.' => Ok(Self::Empty),
            _ => Err(anyhow!("Invalid char")),
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = input
        .lines()
        .map(|row| row.chars().map(Cell::try_from).collect::<Result<Vec<_>>>())
        .collect::<Result<Vec<_>>>()
        .expect("Parse error");
    // dbg!(&grid);
    let row_count = grid.len();
    let col_count = grid[0].len();

    let mut sum = 0;
    for (i, j) in iproduct!(0..row_count, 0..col_count) {
        match grid[i][j] {
            Cell::Empty => {}
            Cell::Paper => {
                let row_range = i.saturating_sub(1)..(i + 2).min(row_count);
                let col_range = j.saturating_sub(1)..(j + 2).min(col_count);
                let mut count = 0;
                for (x, y) in iproduct!(row_range, col_range) {
                    if (x, y) == (i, j) {
                        continue;
                    }
                    count += if grid[x][y] == Cell::Paper { 1 } else { 0 };
                }
                if count < 4 {
                    // println!("{i},{j}: {count}");
                    sum += 1;
                }
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = input
        .lines()
        .map(|row| row.chars().map(Cell::try_from).collect::<Result<Vec<_>>>())
        .collect::<Result<Vec<_>>>()
        .expect("Parse error");
    // dbg!(&grid);
    let row_count = grid.len();
    let col_count = grid[0].len();

    let mut sum = 0;
    let mut trigger = true;
    while trigger {
        trigger = false;
        for (i, j) in iproduct!(0..row_count, 0..col_count) {
            match grid[i][j] {
                Cell::Empty => {}
                Cell::Paper => {
                    let row_range = i.saturating_sub(1)..(i + 2).min(row_count);
                    let col_range = j.saturating_sub(1)..(j + 2).min(col_count);
                    let mut count = 0;
                    for (x, y) in iproduct!(row_range, col_range) {
                        if (x, y) == (i, j) {
                            continue;
                        }
                        count += if grid[x][y] == Cell::Paper { 1 } else { 0 };
                    }
                    if count < 4 {
                        // println!("{i},{j}: {count}");
                        grid[i][j] = Cell::Empty;
                        trigger = true;
                        sum += 1;
                    }
                }
            }
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
