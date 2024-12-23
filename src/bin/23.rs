#![feature(iter_next_chunk)]

use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<usize> {
    let networks = parse_input(input);
    Some(find_historian(networks, 't'))
}

pub fn part_two(_: &str) -> Option<u32> {
    None
}

type Followers<'a> = FxHashSet<&'a str>;
type Networks<'a> = FxHashMap<&'a str, Followers<'a>>;

fn find_historian(networks: Networks, prefix: char) -> usize {
    let mut lan_networks = FxHashSet::default();
    for (candidate, followers) in networks.iter().filter(|(c, _)| c.starts_with(prefix)) {
        for direct in followers {
            for indirect in followers.intersection(&networks[direct]) {
                lan_networks.insert(
                    [candidate, direct, indirect]
                        .into_iter()
                        .sorted()
                        .next_chunk::<3>()
                        .unwrap(),
                );
            }
        }
    }
    lan_networks.len()
}

fn parse_input(input: &str) -> Networks<'_> {
    let mut networks = Networks::default();
    for (one, two) in input.lines().map(|line| line.split_once('-').unwrap()) {
        networks
            .entry(one)
            .or_insert_with(FxHashSet::default)
            .insert(two);
        networks
            .entry(two)
            .or_insert_with(FxHashSet::default)
            .insert(one);
    }
    networks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
