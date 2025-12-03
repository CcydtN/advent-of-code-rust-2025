advent_of_code::solution!(3);

use std::ops::Deref;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use itertools::Itertools;

#[derive(Debug)]
struct Battery(u64);

impl TryFrom<char> for Battery {
    type Error = Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        value
            .to_digit(10)
            .ok_or(anyhow!(format!("Unknow digit, {value}")))
            .map(|val| Battery(val.into()))
    }
}

impl Deref for Battery {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
struct Bank(Vec<Battery>);

impl TryFrom<&str> for Bank {
    type Error = Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        Ok(Self(
            value
                .chars()
                .map(Battery::try_from)
                .collect::<Result<Vec<_>>>()?,
        ))
    }
}

impl Deref for Bank {
    type Target = Vec<Battery>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let inputs = input
        .lines()
        .map(Bank::try_from)
        .inspect(|val| {
            // dbg!(val);
        })
        .collect::<Result<Vec<_>>>()
        .expect("Parse input error");

    let mut sum = 0u64;
    for bank in inputs {
        let mut buffer = vec![vec![]; 10];
        for (i, battery) in bank.iter().enumerate() {
            buffer[battery.0 as usize].push(i);
        }
        // dbg!(&buffer);
        'outer: for i in (1..=9u64).rev() {
            let Some(start) = buffer[i as usize].first() else {
                continue;
            };
            for j in (1..=9u64).rev() {
                let Some(end) = buffer[j as usize].last() else {
                    continue;
                };
                if start >= end {
                    continue;
                }
                sum += i * 10 + j;
                // dbg!(i * 10 + j);
                break 'outer;
            }
        }
    }
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
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
