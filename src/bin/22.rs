use itertools::{iterate, Itertools};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use rustc_hash::{FxHashMap, FxHashSet};
use std::iter::successors;
use std::num::Wrapping;

advent_of_code::solution!(22);

pub fn part_one(input: &str) -> Option<Secret> {
    let secret = parse_input(input);
    Some(calculate(&secret))
}

pub fn part_two(input: &str) -> Option<TotalCost> {
    let secret = parse_input(input);
    Some(calculate_with_diffs(&secret))
}

fn calculate(secret: &[Secret]) -> Secret {
    fn evo_inner(sec: &Secret) -> Option<Secret> {
        Some(sec_evo(*sec))
    }
    secret
        .par_iter()
        .map(|&sec| successors(Some(sec), evo_inner).nth(2000))
        .flatten()
        .sum()
}

fn calculate_with_diffs(secrets: &[Secret]) -> TotalCost {
    let valid_sequences_with_costs: Vec<_> = secrets
        .par_iter()
        .map(|sec| find_sequences_with_costs(*sec))
        .collect();
    fn flatten_sequences(
        mut acc: FxHashSet<Sequence>,
        curr: &FxHashMap<Sequence, Cost>,
    ) -> FxHashSet<Sequence> {
        acc.extend(curr.keys());
        acc
    }
    fn unify_sequences(
        curr: FxHashSet<Sequence>,
        next: FxHashSet<Sequence>,
    ) -> FxHashSet<Sequence> {
        curr.union(&next).copied().collect()
    }
    let sequence_to_cost = |sequence: Sequence| -> TotalCost {
        valid_sequences_with_costs
            .par_iter()
            .map(|curr| curr.get(&sequence))
            .flatten()
            .map(|&cost| cost as TotalCost)
            .sum()
    };
    valid_sequences_with_costs
        .par_iter()
        .fold(FxHashSet::default, flatten_sequences)
        .reduce(FxHashSet::default, unify_sequences)
        .into_par_iter()
        .map(sequence_to_cost)
        .max()
        .unwrap()
}

fn find_sequences_with_costs(secret: Secret) -> FxHashMap<Sequence, Cost> {
    let mut sequences = FxHashMap::default();
    for (s1, s2, s3, s4) in iterate((secret, 0, 0), |&sec| sec_evo_price(sec.0))
        .take(2000)
        .map(|(_, cost, delta)| (delta, cost))
        .tuple_windows()
    {
        let sequence = [s1.0, s2.0, s3.0, s4.0];
        let cost = s4.1;
        // inserting the first cost for the sequence
        sequences.entry(sequence).or_insert(cost);
    }
    sequences
}

type Secret = u64;
type Cost = u8;
type TotalCost = u32;
type DiffToPrev = i64;
type Sequence = [DiffToPrev; 4];

fn parse_input(input: &str) -> Vec<Secret> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[inline]
fn sec_evo(secret: Secret) -> Secret {
    let mut secret = Wrapping(secret);
    // mul 64
    secret ^= secret << 6;
    secret %= 16777216;
    // div 32
    secret ^= secret >> 5;
    secret %= 16777216;
    // mul 2048
    secret ^= secret << 11;
    secret %= 16777216;
    secret.0
}

#[inline]
fn sec_evo_price(secret: Secret) -> (Secret, Cost, DiffToPrev) {
    let ld = secret % 10;
    let next = sec_evo(secret);
    let next_ld = next % 10;
    (
        next,
        next_ld as Cost,
        next_ld as DiffToPrev - ld as DiffToPrev,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(23));
    }

    #[test]
    fn test_sec_evo() {
        let mut sec = 123;
        sec = sec_evo(sec);
        assert_eq!(sec, 15887950);
        sec = sec_evo(sec);
        assert_eq!(sec, 16495136);
        sec = sec_evo(sec);
        assert_eq!(sec, 527345);
        sec = sec_evo(sec);
        assert_eq!(sec, 704524);
        sec = sec_evo(sec);
        assert_eq!(sec, 1553684);
        sec = sec_evo(sec);
        assert_eq!(sec, 12683156);
        sec = sec_evo(sec);
        assert_eq!(sec, 11100544);
        sec = sec_evo(sec);
        assert_eq!(sec, 12249484);
        sec = sec_evo(sec);
        assert_eq!(sec, 7753432);
        sec = sec_evo(sec);
        assert_eq!(sec, 5908254);
    }

    #[test]
    fn test_sec_evo_price() {
        let mut sec = sec_evo_price(123);
        assert_eq!(sec, (15887950, 0, -3));
        sec = sec_evo_price(sec.0);
        assert_eq!(sec, (16495136, 6, 6));
        sec = sec_evo_price(sec.0);
        assert_eq!(sec, (527345, 5, -1));
        sec = sec_evo_price(sec.0);
        assert_eq!(sec, (704524, 4, -1));
        sec = sec_evo_price(sec.0);
        assert_eq!(sec, (1553684, 4, 0));
        sec = sec_evo_price(sec.0);
        assert_eq!(sec, (12683156, 6, 2));
        sec = sec_evo_price(sec.0);
        assert_eq!(sec, (11100544, 4, -2));
        sec = sec_evo_price(sec.0);
        assert_eq!(sec, (12249484, 4, 0));
        sec = sec_evo_price(sec.0);
        assert_eq!(sec, (7753432, 2, -2));
        sec = sec_evo_price(sec.0);
        assert_eq!(sec, (5908254, 4, 2));
    }
}
