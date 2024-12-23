#![feature(iter_next_chunk)]

use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<usize> {
    let networks = parse_input(input);
    Some(find_historian(networks, 't'))
}

pub fn part_two(input: &str) -> Option<String> {
    let networks = parse_input(input);
    Some(find_historian_largest(networks))
}

type Followers<'a> = FxHashSet<&'a str>;
type Networks<'a> = FxHashMap<&'a str, Followers<'a>>;
type LanNetworks<'a> = FxHashSet<[&'a str; 3]>;

fn find_historian_largest(networks: Networks) -> String {
    let lan_groups = build_lan_friends_map(&networks);
    let max = lan_groups.iter().map(|x| x.len()).max().unwrap();
    lan_groups
        .iter()
        .filter(|x| x.len() == max)
        .dedup()
        .flatten()
        .sorted()
        .join(",")
}

fn build_lan_friends_map<'a>(networks: &Networks<'a>) -> Vec<Followers<'a>> {
    let mut lan_groups = vec![];
    for &candidate in networks.keys() {
        lan_groups.push(FxHashSet::from_iter([candidate]));
    }
    for lan_group in &mut lan_groups {
        for (&friend, followers) in networks {
            if lan_group.iter().all(|other| followers.contains(other)) {
                lan_group.insert(friend);
            }
        }
    }
    lan_groups
}

fn find_historian(networks: Networks, prefix: char) -> usize {
    build_network_map(&networks)
        .iter()
        .filter(|cs| cs.iter().any(|c| c.starts_with(prefix)))
        .count()
}

fn build_network_map<'a>(networks: &'a Networks<'a>) -> LanNetworks<'a> {
    let mut lan_networks = LanNetworks::default();
    for (candidate, followers) in networks {
        for direct in followers {
            for indirect in followers.intersection(&networks[direct]) {
                lan_networks.insert(
                    [candidate, direct, indirect]
                        .into_iter()
                        .sorted()
                        .copied()
                        .next_chunk::<3>()
                        .unwrap(),
                );
            }
        }
    }
    lan_networks
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
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
