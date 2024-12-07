#![feature(array_windows)]

use anyhow::{bail, Context};
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Mul};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let equations = parse_input(input).unwrap();
    Some(count_solvable(equations, calculate_2))
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = parse_input(input).unwrap();
    Some(count_solvable(equations, calculate_3))
}

fn concat(first: u64, second: u64) -> u64 {
    let factor = count_digits(second);
    first * (10u64.pow(factor)) + second
}

#[inline]
fn count_digits(n: u64) -> u32 {
    if n == 0 {
        return 1;
    }
    let mut count = 0;
    let mut num = n;
    while num > 0 {
        num /= 10;
        count += 1;
    }
    count
}

fn count_solvable(equations: Vec<Equation>, rec_fn: fn(u64, u64, &[u64]) -> bool) -> u64 {
    equations
        .into_iter()
        .filter(|equation| rec_fn(equation.total, 0, &equation.candidates))
        .map(|equation| equation.total)
        .sum()
}

fn calculate_2(limit: u64, curr_sum: u64, nums: &[u64]) -> bool {
    if limit == curr_sum {
        return true;
    }
    if let Some(&num) = nums.first() {
        if curr_sum > limit {
            false
        } else {
            calculate_2(limit, curr_sum + num, &nums[1..])
                || calculate_2(limit, curr_sum * num, &nums[1..])
        }
    } else {
        false
    }
}

fn calculate_3(limit: u64, curr_sum: u64, nums: &[u64]) -> bool {
    if limit == curr_sum {
        return true;
    }
    if let Some(&num) = nums.first() {
        if curr_sum > limit {
            false
        } else {
            calculate_3(limit, curr_sum + num, &nums[1..])
                || calculate_3(limit, curr_sum * num, &nums[1..])
                || calculate_3(limit, concat(curr_sum, num), &nums[1..])
        }
    } else {
        false
    }
}

#[allow(unused)]
fn dyn_count_solvable<const N: usize>(
    equations: Vec<Equation>,
    operators: [fn(u64, u64) -> u64; N],
) -> u64 {
    equations
        .into_iter()
        .filter(|equation| dyn_calculate(equation.total, 0, &equation.candidates, operators))
        .map(|equation| equation.total)
        .sum()
}

#[allow(unused)]
fn dyn_calculate<const N: usize>(
    limit: u64,
    curr_sum: u64,
    nums: &[u64],
    operators: [fn(u64, u64) -> u64; N],
) -> bool {
    if limit == curr_sum {
        return true;
    }
    if let Some(&num) = nums.first() {
        if curr_sum > limit {
            false
        } else {
            for op in operators {
                if dyn_calculate(limit, op(curr_sum, num), &nums[1..], operators) {
                    return true;
                }
            }
            operators
                .iter()
                .any(|op| dyn_calculate(limit, op(curr_sum, num), &nums[1..], operators))
        }
    } else {
        false
    }
}

#[allow(dead_code)]
struct FormatFormula<'a>(&'a Equation, &'a [fn(u64, u64) -> u64]);

impl Debug for FormatFormula<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Self(equation, fun_candidates) = self;
        let mut start = equation.candidates.iter();
        write!(f, "{}", start.next().unwrap())?;
        // print.push_str(&format!("{}", start.next().unwrap()));
        for (fun, num) in fun_candidates.iter().zip(start) {
            let fun = if fun == &(u64::add as fn(u64, u64) -> u64) {
                "+"
            } else if fun == &(u64::mul as fn(u64, u64) -> u64) {
                "*"
            } else if fun == &(concat as fn(u64, u64) -> u64) {
                "||"
            } else {
                panic!("unknown function");
            };
            write!(f, " {} {}", fun, num)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Equation {
    total: u64,
    candidates: Vec<u64>,
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Equation>> {
    let mut equations = vec![];
    for line in input.lines() {
        let Some((total, candidates)) = line.split_once(':') else {
            bail!("not a valid input: {line}")
        };
        let total = total.trim().parse().context("unable to parse total")?;
        let candidates = candidates
            .split(char::is_whitespace)
            .filter(|e| !e.is_empty())
            .map(|e| {
                e.trim()
                    .parse()
                    .context(format!("unable to parse candidate '{e}'"))
            })
            .collect::<Result<_, _>>()?;
        equations.push(Equation { total, candidates })
    }
    Ok(equations)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }

    #[test]
    fn test_formula_printer() {
        let equation = Equation {
            total: 123,
            candidates: vec![55, 14, 34, 99, 12],
        };
        let ff = FormatFormula(&equation, &[u64::add, u64::mul, concat, u64::add]);
        assert_eq!("55 + 14 * 34 || 99 + 12", format!("{ff:?}"));
    }

    #[test]
    fn test_part_one_alternative() {
        let equations = parse_input(&advent_of_code::template::read_file("examples", DAY)).unwrap();
        let result = dyn_count_solvable(equations, [u64::add, u64::mul]);
        assert_eq!(result, 3749);
    }

    #[test]
    fn test_part_two_alternative() {
        let equations = parse_input(&advent_of_code::template::read_file("examples", DAY)).unwrap();
        let result = dyn_count_solvable(equations, [u64::add, u64::mul, concat]);
        assert_eq!(result, 11387);
    }
}
