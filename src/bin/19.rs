advent_of_code::solution!(19);

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    let (towels, patterns) = parse_input(input);
    Some(find_matching_patterns(towels, patterns))
}

pub fn part_two(_: &str) -> Option<u32> {
    None
}

fn find_matching_patterns<'a>(towels: Towels<'a>, patterns: Patterns<'a>) -> usize {
    patterns
        .iter()
        .filter(|pattern| can_match(pattern, &towels))
        .count()
}

fn can_match<'a>(pattern: Pattern<'a>, towels: &Towels<'a>) -> bool {
    let mut from = vec![0];
    let mut covered = roaring::RoaringBitmap::new();

    while let Some(position) = from.pop() {
        if !covered.insert(position as u32) {
            continue;
        }
        if (0..towels.max_len)
            .filter(|towel| towel + position < pattern.len())
            .map(|towel| &pattern[position..=position + towel])
            .filter(|towel| towels.towels.contains(towel))
            .inspect(|towel| from.push(position + towel.len()))
            .any(|towel| position + towel.len() == pattern.len())
        {
            return true;
        }
    }
    false
}

type Patterns<'a> = Vec<&'a str>;
type Pattern<'a> = &'a str;

fn parse_input(input: &str) -> (Towels<'_>, Patterns<'_>) {
    let (towels, patterns) = input.split_once("\n\n").unwrap();
    let towels = towels.split(',').map(|towel| towel.trim()).collect_vec();
    let min_len = towels.iter().map(|towel| towel.len()).min().unwrap();
    let max_len = towels.iter().map(|towel| towel.len()).max().unwrap();
    let patterns = patterns.lines().map(|pattern| pattern.trim()).collect();
    (
        Towels {
            towels,
            min_len,
            max_len,
        },
        patterns,
    )
}

struct Towels<'a> {
    towels: Vec<&'a str>,
    #[allow(dead_code)]
    min_len: usize,
    max_len: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
