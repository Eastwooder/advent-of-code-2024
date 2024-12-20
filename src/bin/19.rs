advent_of_code::solution!(19);

use itertools::Itertools;
use rustc_hash::FxHashMap;

pub fn part_one(input: &str) -> Option<usize> {
    let (towels, patterns) = parse_input(input);
    Some(find_matching_patterns(towels, patterns))
}

pub fn part_two(input: &str) -> Option<usize> {
    let (towels, patterns) = parse_input(input);
    Some(find_matching_patterns_combinations(towels, patterns))
}

fn find_matching_patterns<'a>(towels: Towels<'a>, patterns: Patterns<'a>) -> usize {
    patterns
        .iter()
        .filter(|pattern| can_match(pattern, &towels))
        .count()
}

fn find_matching_patterns_combinations<'a>(towels: Towels<'a>, patterns: Patterns<'a>) -> usize {
    patterns
        .iter()
        .map(|pattern| count_match(pattern, &towels.without(pattern)))
        .sum()
}

fn count_match<'a>(pattern: Pattern<'a>, towels: &Towels<'a>) -> usize {
    fn count_match(
        memoize: &mut FxHashMap<usize, usize>,
        pattern: Pattern<'_>,
        towels: &Towels<'_>,
        current: usize,
    ) -> usize {
        if let Some(result) = memoize.get(&current) {
            return *result;
        }
        let forward = |towel: Pattern<'_>| {
            if current + towel.len() == pattern.len() {
                1
            } else {
                count_match(memoize, pattern, towels, current + towel.len())
            }
        };
        let count = (0..towels.max_len)
            .filter(|pos| pos + current < pattern.len())
            .map(|pos| &pattern[current..=current + pos])
            .filter(|towel| towels.towels.contains(towel))
            .map(forward)
            .sum();
        memoize.insert(current, count);
        count
    }
    count_match(&mut FxHashMap::default(), pattern, towels, 0)
}

fn can_match<'a>(pattern: Pattern<'a>, towels: &Towels<'a>) -> bool {
    let mut from = vec![0];
    let mut covered = roaring::RoaringBitmap::new();

    while let Some(position) = from.pop() {
        if !covered.insert(position as u32) {
            continue;
        }
        if (0..towels.max_len)
            .filter(|pos| pos + position < pattern.len())
            .map(|pos| &pattern[position..=position + pos])
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
    towels: Vec<Pattern<'a>>,
    #[allow(dead_code)]
    min_len: usize,
    max_len: usize,
}

impl<'a> Towels<'a> {
    fn without(&self, pattern: Pattern<'a>) -> Self {
        let towels = self
            .towels
            .iter()
            .filter(|&&towel| towel != pattern)
            .copied()
            .collect_vec();
        let min_len = towels.iter().map(|towel| towel.len()).min().unwrap();
        let max_len = towels.iter().map(|towel| towel.len()).max().unwrap();
        Towels {
            towels,
            min_len,
            max_len,
        }
    }
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
        assert_eq!(result, Some(16));
    }
}
