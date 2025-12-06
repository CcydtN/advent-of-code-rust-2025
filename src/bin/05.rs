advent_of_code::solution!(5);

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use itertools::Itertools;
use rangemap::RangeInclusiveSet;
use rangemap::RangeSet;
use regex::Regex;
use std::ops::RangeInclusive;

struct Input {
    range: Vec<RangeInclusive<u64>>,
    ingredient: Vec<u64>,
}

impl TryFrom<&str> for Input {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let t = value.split("\n").collect_vec();
        let (position, _) = t
            .iter()
            .find_position(|&val| val == &"")
            .ok_or(anyhow!("should have a empty line"))?;
        let (first, second) = t.split_at(position);
        let range = first
            .iter()
            .inspect(|value| {
                // dbg!(value);
            })
            .map(|val| {
                let pattern = Regex::new(r"^(\d+)-(\d+)$").expect(&"");
                let t = pattern.captures(val).expect(&"");
                t[1].parse().expect("")..=t[2].parse::<u64>().expect("")
            })
            .collect_vec();
        let ingredient = second
            .iter()
            .filter(|val| !val.is_empty())
            .map(|val| val.trim().parse::<u64>().expect(""))
            .collect_vec();
        Ok(Self { range, ingredient })
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let input: Input = input.try_into().expect("Parse fail");
    let set = input.range.into_iter().collect::<RangeInclusiveSet<_>>();
    Some(
        input
            .ingredient
            .into_iter()
            .filter(|value| set.contains(value))
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let input: Input = input.try_into().expect("Parse fail");
    let set = input.range.into_iter().collect::<RangeInclusiveSet<_>>();
    // dbg!(&set);
    Some(set.into_iter().flatten().count() as u64)
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
        assert_eq!(result, Some(14));
    }
}
