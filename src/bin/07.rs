#![feature(array_windows)]

use anyhow::{bail, Context};
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Mul};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let equations = parse_input(input).unwrap();
    Some(count_solvable(equations, &[u64::add, u64::mul]))
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = parse_input(input).unwrap();
    Some(count_solvable(equations, &[u64::add, u64::mul, concat]))
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

fn count_solvable(equations: Vec<Equation>, operators: &[fn(u64, u64) -> u64]) -> u64 {
    let mut sum_total_valid = 0;
    for equation in equations {
        let num_operands = equation.candidates.len() - 1;
        // println!("\n{equation:?}");
        for fun_candidates in generate_permutations(operators, num_operands) {
            // print_formula(&equation, &fun_candidates);
            let mut can = equation.candidates.iter();
            let mut sum = *can.next().expect("expected the candidates to be not empty");
            for (fun, &next) in fun_candidates.iter().zip(can) {
                sum = fun(sum, next);
                // sum = fun(sum, *next);
                if sum > equation.total {
                    break;
                }
            }
            if sum == equation.total {
                sum_total_valid += equation.total;
                break;
            }
        }
    }
    sum_total_valid
}

fn generate_permutations<T: Copy>(input: &[T], k: usize) -> Vec<Vec<T>> {
    if k == 0 {
        return vec![vec![]];
    }

    let mut result = Vec::new();
    for &item in input {
        for mut sub_perm in generate_permutations(input, k - 1) {
            let mut perm = vec![item];
            perm.append(&mut sub_perm);
            result.push(perm);
        }
    }

    result
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
}
