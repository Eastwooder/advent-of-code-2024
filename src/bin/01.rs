use anyhow::bail;
use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let Ok((mut left, mut right)) = parse_input(input) else {
        return None;
    };

    left.sort();
    right.sort();

    let total_distance = left
        .iter()
        .zip(right.iter())
        .map(|(left, right)| left.abs_diff(*right))
        .sum::<u32>();

    Some(total_distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    let Ok((left, right)) = parse_input(input) else {
        return None;
    };

    let right_frequency = {
        let mut hm = HashMap::new();
        right
            .iter()
            .for_each(|num| *hm.entry(num).or_insert(0) += 1);
        hm
    };
    let similarity_score: u32 = left
        .iter()
        .map(|l| l * right_frequency.get(l).unwrap_or(&0))
        .sum();

    Some(similarity_score)
}

#[allow(clippy::type_complexity)]
fn parse_input(input: &str) -> anyhow::Result<(Box<[u32]>, Box<[u32]>)> {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let Some((left_num, right_num)) = line
            .split_once(char::is_whitespace)
            .map(|(left, right)| (left.trim(), right.trim()))
            .map(|(left, right)| (left.parse::<u32>(), right.parse::<u32>()))
        else {
            bail!("Not a string with 2 numbers separated by white space: {line}");
        };
        left.push(left_num?);
        right.push(right_num?);
    }

    Ok((left.into(), right.into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
