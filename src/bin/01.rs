advent_of_code::solution!(1);

use anyhow::Result;
use anyhow::anyhow;

#[derive(Debug)]
enum Direction {
    Left(i64),
    Right(i64),
}

impl TryFrom<&str> for Direction {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (dir, val) = value.split_at(1);
        match dir {
            "L" => Ok(Direction::Left(val.parse()?)),
            "R" => Ok(Direction::Right(val.parse()?)),
            _ => Err(anyhow!("Input Error")),
        }
    }
}

impl Direction {
    fn to_value(&self) -> i64 {
        match self {
            Direction::Left(val) => -val,
            Direction::Right(val) => *val,
        }
    }
}

fn parseInput(input: &str) -> Result<Vec<Direction>> {
    input
        .lines()
        .map(|value| Direction::try_from(value))
        .collect()
}

pub fn part_one(input: &str) -> Option<i64> {
    let inputs = parseInput(input).expect("Parse Error");
    let mut dial = 50i64;
    let mut password = 0;
    for input in inputs {
        dial += input.to_value();
        dial = dial.rem_euclid(100);
        if dial == 0 {
            password += 1;
        }
    }
    Some(password)
}

pub fn part_two(input: &str) -> Option<i64> {
    let inputs = parseInput(input).expect("Parse Error");
    let mut stupid_dial = 50i64;
    let mut stupid_password = 0;
    for input in inputs {
        match input {
            Direction::Left(val) => {
                for _ in 0..val {
                    stupid_dial -= 1;
                    if stupid_dial % 100 == 0 {
                        stupid_dial = 0;
                        stupid_password += 1;
                    }
                }
            }
            Direction::Right(val) => {
                for _ in 0..val {
                    stupid_dial += 1;
                    if stupid_dial % 100 == 0 {
                        stupid_dial = 0;
                        stupid_password += 1;
                    }
                }
            }
        }
        // println!("{input:?}, {stupid_password:?}, {stupid_dial:?}");
    }
    Some(stupid_password)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two_customs() {
        let result = part_two(&advent_of_code::template::read_file("customs", DAY));
        assert_eq!(result, Some(11));
    }
}
