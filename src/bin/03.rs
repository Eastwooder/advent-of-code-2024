#![feature(coroutines)]
#![feature(coroutine_trait)]

use regex::Regex;
use std::ops::{Coroutine, CoroutineState};
use std::pin::Pin;
use std::sync::LazyLock;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        find_all_mul_regex(input)
            .iter()
            .map(|(left, right)| left * right)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut enabled = true;
    let mut result = 0;
    let mut results = find_all_instructions(input);
    while let CoroutineState::Yielded(instruction) = Pin::new(&mut results).resume(()) {
        match instruction {
            Instruction::Mul(left, right) if enabled => result += left * right,
            Instruction::Mul(_, _) => {}
            Instruction::Do => enabled = true,
            Instruction::Dont => enabled = false,
        }
    }
    Some(result)
}

fn find_all_mul_regex(input: &str) -> Vec<(u32, u32)> {
    static REGEX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"mul\((?<first>\d{1,3}),(?<second>\d{1,3})\)").unwrap());
    let mut result = Vec::new();
    for occ in REGEX.captures_iter(input) {
        let first = occ
            .name("first")
            .expect("expected named capture 'first'")
            .as_str()
            .parse()
            .expect("expected 'first' to be a number according to regex");
        let second = occ
            .name("second")
            .expect("expected named capture 'second'")
            .as_str()
            .parse()
            .expect("expected 'second' to be a number according to regex");
        result.push((first, second));
    }
    result
}

fn find_all_instructions(
    input: &str,
) -> impl Coroutine<Yield = Instruction, Return = ()> + use<'_> {
    static REGEX: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(
            r"(?<mul>mul\((?<first>\d{1,3}),(?<second>\d{1,3})\))|(?<do>do\(\))|(?<dont>don't\(\))",
        )
        .unwrap()
    });
    #[coroutine]
    || {
        for occ in REGEX.captures_iter(input) {
            yield if occ.name("mul").is_some() {
                let first = occ
                    .name("first")
                    .expect("expected named capture 'first'")
                    .as_str()
                    .parse()
                    .expect("expected 'first' to be a number according to regex");
                let second = occ
                    .name("second")
                    .expect("expected named capture 'second'")
                    .as_str()
                    .parse()
                    .expect("expected 'second' to be a number according to regex");
                Instruction::Mul(first, second)
            } else if (occ.name("do")).is_some() {
                Instruction::Do
            } else {
                Instruction::Dont
            };
        }
    }
}

enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }

    #[test]
    fn test_parser_combinator() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = find_all_mul_regex(input);
        assert_eq!(result, vec![(2, 4), (5, 5), (11, 8), (8, 5)]);
    }
}
