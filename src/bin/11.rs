use arrayvec::ArrayVec;
use itertools::Itertools;
use rustc_hash::{FxBuildHasher, FxHashMap};

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let stones = parse_input(input.trim());
    Some(blink(stones, 25))
}

pub fn part_two(input: &str) -> Option<u64> {
    let stones = parse_input(input.trim());
    Some(blink(stones, 75))
}

type SVec = ArrayVec<Stone, 2>;
type Stone = u64;
type Count = u64;
type Cache = FxHashMap<Stone, Count>;

fn blink(stones: Vec<u64>, n: usize) -> u64 {
    fn populate_cache(mut map: Cache, val: u64) -> Cache {
        let entry = map.entry(val).or_default();
        *entry += 1;
        map
    }
    let stone_count = Cache::with_capacity_and_hasher(stones.len() * 100, FxBuildHasher);
    let mut stone_count = stones.into_iter().fold(stone_count, populate_cache);
    for _ in 0..n {
        stone_count = blink_generation(stone_count);
    }
    stone_count.values().sum()
}

fn blink_generation(stones: Cache) -> Cache {
    let mut res = Cache::with_capacity_and_hasher(stones.len() * 2, FxBuildHasher);
    for (stone, cnt) in stones {
        for new_stone in step(stone) {
            let entry = res.entry(new_stone).or_default();
            *entry += cnt;
        }
    }
    res
}

fn step(stone: Stone) -> SVec {
    match stone {
        0 => SVec::from_iter([1]),
        n if (n.ilog(10) + 1) % 2 == 0 => split_half(n),
        other => SVec::from_iter([other * 2024]),
    }
}

fn split_half(stone: Stone) -> SVec {
    let num_digits = stone.ilog(10) + 1;
    debug_assert!(num_digits % 2 == 0);
    debug_assert!(num_digits > 0);
    let divisor = 10usize.pow(num_digits / 2);
    SVec::from([stone / divisor as Stone, stone % divisor as Stone])
}

fn parse_input(input: &str) -> Vec<Stone> {
    input
        .split(char::is_whitespace)
        .map(|e| e.parse().unwrap())
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
