advent_of_code::solution!(2);

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use regex::Regex;

#[derive(Debug)]
struct Input {
    lower: u64,
    upper: u64,
}

impl TryFrom<&str> for Input {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let pattern = Regex::new(r"^(\d+)\-(\d+)$")?;
        let result = pattern
            .captures(value.trim())
            .ok_or(anyhow!("Invalid Input"))?;
        Ok(Self {
            lower: result[1].parse()?,
            upper: result[2].parse()?,
        })
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let inputs = input
        .split_terminator(",")
        .inspect(|v| {
            // dbg!(v);
        })
        .map(Input::try_from)
        .collect::<Result<Vec<Input>>>()
        .expect("Failed to parse input");

    let mut sum = 0;
    for input in inputs {
        for i in input.lower..=input.upper {
            let digit = i.ilog10() + 1;
            if digit % 2 == 1 {
                continue;
            }
            let divider = 10u64.pow(digit / 2) + 1;
            // println!("{i}, {digit}, {divider}");
            if i % divider == 0 {
                sum += i;
                // dbg!(i);
            }
        }
    }
    // println!("{:?}", inputs);

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
