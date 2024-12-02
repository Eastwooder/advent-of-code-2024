use itertools::Itertools;
use std::cmp::Ordering;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        parse_input(input)
            .into_iter()
            .map(|report| check_report(&report))
            .filter(|r| *r)
            .count() as _,
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    Some(
        parse_input(_input)
            .iter()
            .map(|report| check_report_with_dampener(report))
            .filter(|r| *r)
            .count() as _,
    )
}

fn check_report_with_dampener(report: &[u32]) -> bool {
    if check_report(report) {
        true
    } else {
        (0..report.len()).any(|ignore| {
            check_report(
                &report
                    .iter()
                    .enumerate()
                    .filter(|(idx, _)| *idx != ignore)
                    .map(|(_, val)| val)
                    .copied()
                    .collect_vec(),
            )
        })
    }
}

fn check_report(report: &[u32]) -> bool {
    assert!(
        report.len() >= 2,
        "expect the report to have at least 2 elements"
    );
    let order = report[0].cmp(&report[1]);
    if matches!(order, Ordering::Equal) {
        return false;
    }
    for pair in report.windows(2) {
        let first = pair[0];
        let second = pair[1];
        if first.cmp(&second) != order {
            return false;
        }
        if first.abs_diff(second) > 3 {
            return false;
        }
    }
    true
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split(char::is_whitespace)
                .map(|i| i.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
