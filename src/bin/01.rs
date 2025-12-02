advent_of_code::solution!(1);
use anyhow::Result;
use anyhow::anyhow;
use num::traits::Euclid;

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
        match input {
            Direction::Left(val) => {
                dial -= val;
            }
            Direction::Right(val) => {
                dial += val;
            }
        }
        dial = dial.rem_euclid(100);
        println!("{:?}", dial);
        if dial == 0 {
            password += 1;
        }
    }
    Some(password)
}

pub fn part_two(input: &str) -> Option<i64> {
    None
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
        assert_eq!(result, None);
    }
}
