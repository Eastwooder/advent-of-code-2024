use itertools::Itertools;
use std::cell::Cell;
use std::collections::HashMap;
use std::str::FromStr;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (ordering_rules, print_orders) = parse_input2(input);
    Some(sum_correct_middle_pages(&ordering_rules, &print_orders))
}

pub fn part_two(input: &str) -> Option<u32> {
    let (ordering_rules, print_orders) = parse_input2(input);
    Some(correct_pages(&ordering_rules, &print_orders))
}

fn sum_correct_middle_pages(ordering_rules: &[(u32, u32)], print_orders: &[Vec<u32>]) -> u32 {
    let mut sum_correct = 0;
    let check = build_check_map(ordering_rules);
    for print_order in print_orders {
        'page: for (idx, page) in print_order.iter().enumerate() {
            if let Some(before) = check.get(page) {
                for after in before {
                    if print_order[..idx].contains(after) {
                        break 'page;
                    }
                }
            }
            if idx == print_order.len() - 1 {
                sum_correct += print_order[print_order.len() / 2];
            }
        }
    }
    sum_correct
}

fn build_check_map(ordering_rules: &[(u32, u32)]) -> HashMap<u32, Vec<u32>> {
    let mut hm = HashMap::new();
    for (before, after) in ordering_rules {
        hm.entry(*before).or_insert_with(Vec::new).push(*after);
    }
    hm
}

fn correct_pages(ordering_rules: &[(u32, u32)], print_orders: &[Vec<u32>]) -> u32 {
    let mut corrected = 0;
    let check = build_check_map(ordering_rules);
    for print_order in print_orders {
        let corrected_order = print_order.iter().map(Cell::new).collect_vec();
        if swap_correct(&check, &corrected_order) {
            while swap_correct(&check, &corrected_order) {
                // correcting pages
            }
            corrected += corrected_order[print_order.len() / 2].get();
        }
    }
    corrected
}

fn swap_correct(check: &HashMap<u32, Vec<u32>>, corrected_page: &Vec<Cell<&u32>>) -> bool {
    let mut has_error = false;
    for (idx, page) in corrected_page.iter().enumerate() {
        if let Some(before) = check.get(page.get()) {
            for after in before {
                let pos = corrected_page[..idx]
                    .iter()
                    .find_position(|&x| x.get() == after);
                if let Some((pos, _)) = pos {
                    has_error = true;
                    corrected_page[pos].swap(&corrected_page[idx]);
                }
            }
        }
    }
    has_error
}

#[derive(Debug)]
enum Instruction {
    PageOrderingRule((u32, u32)),
    Updates(Vec<u32>),
}

impl FromStr for Instruction {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rule = s.split_once('|');
        if let Some((before, after)) = rule {
            let before = before.parse()?;
            let after = after.parse()?;
            Ok(Instruction::PageOrderingRule((before, after)))
        } else {
            let numbers: Result<_, _> = s.split(',').map(|s| s.parse()).collect();
            Ok(Instruction::Updates(numbers?))
        }
    }
}

fn parse_input2(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let mut rules = vec![];
    let mut pages = vec![];
    let mut iter = input.lines();
    loop {
        let line = iter.next();
        let Some(line) = line else { break };
        if line.is_empty() {
            break;
        }
        if let Instruction::PageOrderingRule((before, after)) = line.parse().unwrap() {
            rules.push((before, after));
        }
    }
    loop {
        let line = iter.next();
        let Some(line) = line else { break };
        if line.is_empty() {
            break;
        }
        if let Instruction::Updates(p) = line.parse().unwrap() {
            pages.push(p);
        }
    }
    (rules, pages)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
