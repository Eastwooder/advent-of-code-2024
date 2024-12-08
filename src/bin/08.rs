use advent_of_code::map2d::print_map;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let (map_size, antennas) = parse_input(input);
    Some(calculate_antinodes_pairwise(map_size, antennas))
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map_size, antennas) = parse_input(input);
    Some(calculate_antinodes_unbound(map_size, antennas))
}

fn calculate_antinodes_pairwise(map_size: MapSize, antennas: Antennas) -> u32 {
    // debug_map(map_size, &antennas, None);
    let mut groups = HashMap::new();
    for (&pos, &f) in &antennas {
        groups.entry(f).or_insert(HashSet::new()).insert(pos);
    }
    // println!("{:?}", groups);
    let mut antinodes = Antinodes::new();
    for (_, poss) in groups {
        for a in poss.iter().combinations(2) {
            let (first, second) = (a[0], a[1]);
            let distance = (
                first.0 as i32 - second.0 as i32,
                first.1 as i32 - second.1 as i32,
            );
            // println!("{a:?} distance: {distance:?}");
            let d1 = (first.0 as i32 + distance.0, first.1 as i32 + distance.1);
            let d2 = (second.0 as i32 - distance.0, second.1 as i32 - distance.1);
            if check_in_bounds(d1, map_size) {
                antinodes.insert((d1.0 as u32, d1.1 as u32));
            }
            if check_in_bounds(d2, map_size) {
                antinodes.insert((d2.0 as u32, d2.1 as u32));
            }
        }
    }
    // debug_map(map_size, &antennas, Some(&antinodes));
    antinodes.len() as u32
}

fn calculate_antinodes_unbound(map_size: crate::MapSize, antennas: crate::Antennas) -> u32 {
    // debug_map(map_size, &antennas, None);
    let mut groups = HashMap::new();
    for (&pos, &f) in &antennas {
        groups.entry(f).or_insert(HashSet::new()).insert(pos);
    }
    let mut antinodes = Antinodes::new();
    for (_, poss) in groups {
        for a in poss.iter().combinations(2) {
            let (first, second) = (a[0], a[1]);
            let distance = (
                first.0 as i32 - second.0 as i32,
                first.1 as i32 - second.1 as i32,
            );
            let mut d1 = (first.0 as i32, first.1 as i32);
            while check_in_bounds(d1, map_size) {
                antinodes.insert((d1.0 as u32, d1.1 as u32));
                d1 = (d1.0 + distance.0, d1.1 + distance.1);
            }
            let mut d2 = (second.0 as i32, second.1 as i32);
            while check_in_bounds(d2, map_size) {
                antinodes.insert((d2.0 as u32, d2.1 as u32));
                d2 = (d2.0 - distance.0, d2.1 - distance.1);
            }
        }
    }
    // debug_map(map_size, &antennas, Some(&antinodes));
    antinodes.len() as u32
}

fn check_in_bounds((x, y): (i32, i32), map_size: MapSize) -> bool {
    x >= 0 && x < map_size.0 as i32 && y >= 0 && y < map_size.1 as i32
}

type X = u32;
type Y = u32;
type MapSize = (X, Y);
type Antenna = (X, Y);
type Frequency = char;
type Antennas = HashMap<Antenna, Frequency>;
type Antinode = (X, Y);
type Antinodes = HashSet<Antinode>;

fn parse_input(input: &str) -> (MapSize, Antennas) {
    let width = input.lines().next().unwrap().len() as _;
    let height = input.lines().count() as _;
    let mut antennas = Antennas::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if matches!(ch, '0'..='9' | 'a'..='z' | 'A'..='Z') {
                antennas.insert((x as _, y as _), ch);
            }
        }
    }
    ((width, height), antennas)
}

#[allow(unused)]
fn debug_map(map_size: MapSize, antennas: &Antennas, antinodes: Option<&HashSet<(X, Y)>>) {
    let mut hm = HashMap::new();
    for (&pos, &f) in antennas {
        hm.insert(pos, f);
    }
    if let Some(antinodes) = antinodes {
        for &antinode in antinodes {
            hm.insert(antinode, '#');
        }
    }
    print_map(map_size, '.', &hm);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
