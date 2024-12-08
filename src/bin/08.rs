use glam::{IVec2, UVec2};
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
    let mut antinodes = Antinodes::new();
    for (_, poss) in antennas {
        for a in poss.iter().combinations(2) {
            let (first, second) = (a[0].as_ivec2(), a[1].as_ivec2());
            let distance = first - second;
            let d1 = first + distance;
            let d2 = second - distance;
            if check_in_bounds(d1, map_size) {
                antinodes.insert(d1);
            }
            if check_in_bounds(d2, map_size) {
                antinodes.insert(d2);
            }
        }
    }
    antinodes.len() as u32
}

fn calculate_antinodes_unbound(map_size: MapSize, antennas: Antennas) -> u32 {
    let mut antinodes = Antinodes::new();
    for (_, poss) in antennas {
        for a in poss.iter().combinations(2) {
            let (first, second) = (a[0].as_ivec2(), a[1].as_ivec2());
            let distance = first - second;
            let mut d1 = first;
            while check_in_bounds(d1, map_size) {
                antinodes.insert(d1);
                d1 += distance;
            }
            let mut d2 = second;
            while check_in_bounds(d2, map_size) {
                antinodes.insert(d2);
                d2 -= distance;
            }
        }
    }
    antinodes.len() as u32
}

fn check_in_bounds(IVec2 { x, y }: IVec2, map_size: MapSize) -> bool {
    x >= 0 && x < map_size.0 as i32 && y >= 0 && y < map_size.1 as i32
}

type MapSize = (u32, u32);
type Antenna = UVec2;
type Frequency = char;
type Antennas = HashMap<Frequency, HashSet<Antenna>>;
type Antinode = IVec2;
type Antinodes = HashSet<Antinode>;

fn parse_input(input: &str) -> (MapSize, Antennas) {
    let width = input.lines().next().unwrap().len() as _;
    let height = input.lines().count() as _;
    let mut antennas = Antennas::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if matches!(ch, '0'..='9' | 'a'..='z' | 'A'..='Z') {
                antennas
                    .entry(ch)
                    .or_default()
                    .insert(UVec2::new(x as _, y as _));
            }
        }
    }
    ((width, height), antennas)
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
