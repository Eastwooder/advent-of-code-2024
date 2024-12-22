use itertools::{iterate, Itertools};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::iter::successors;
use std::num::Wrapping;

advent_of_code::solution!(22);

pub fn part_one(input: &str) -> Option<Secret> {
    let secret = parse_input(input);
    Some(calculate(&secret))
}

pub fn part_two(input: &str) -> Option<Offset> {
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

fn calculate_with_diffs(secret: &[Secret]) -> Offset {
    secret.iter().flat_map(|&sec| find_first(sec)).sum()
}

fn find_first(secret: Secret) -> Option<Offset> {
    const SECRET_SEQUENCE: [DiffToPrev; 4] = [-2, 1, -1, 3];
    iterate((secret, secret % 10, 0), |&sec| sec_evo_price(sec.0))
        .take(2000)
        .tuple_windows()
        .find(|&(s1, s2, s3, s4)| [s1.2, s2.2, s3.2, s4.2] == SECRET_SEQUENCE)
        .map(|(_, _, _, s4)| s4.1)
}

type Secret = u64;
type LastDigit = u64;
type DiffToPrev = i64;
type Offset = u64;

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
fn sec_evo_price(secret: Secret) -> (Secret, LastDigit, DiffToPrev) {
    let next = sec_evo(secret);
    let next_ld = next % 10;
    let ld = secret % 10;
    (next, next_ld, next_ld as DiffToPrev - ld as DiffToPrev)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

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

    #[rstest]
    #[case(1, Some(7))]
    #[case(2, Some(7))]
    #[case(3, None)]
    #[case(2024, Some(9))]
    fn test_find_first(#[case] secret: Secret, #[case] expected: Option<Offset>) {
        let result = find_first(secret);
        assert_eq!(result, expected);
    }
}
