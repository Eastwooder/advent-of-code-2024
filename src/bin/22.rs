use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::iter::successors;
use std::num::Wrapping;

advent_of_code::solution!(22);

pub fn part_one(input: &str) -> Option<Secret> {
    let secret = parse_input(input);
    Some(calculate(&secret))
}

pub fn part_two(_: &str) -> Option<u32> {
    None
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

type Secret = u64;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
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
}
