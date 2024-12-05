use itertools::Itertools;
use std::cell::Cell;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let instructions = parse_input(input);
    Some(sum_correct_middle_pages(instructions))
}

fn sum_correct_middle_pages(instructions: Vec<Instruction>) -> u32 {
    let mut sum_correct = 0;
    let check = build_check_map(&instructions);
    for instruction in &instructions {
        let Instruction::Updates(pages) = instruction else {
            continue;
        };
        'page: for (idx, page) in pages.iter().enumerate() {
            if let Some(before) = check.get(page) {
                for after in before {
                    if pages[..idx].contains(after) {
                        break 'page;
                    }
                }
            }
            if idx == pages.len() - 1 {
                sum_correct += pages[pages.len() / 2];
            }
        }
    }
    sum_correct
}

fn build_check_map(instructions: &[Instruction]) -> HashMap<u32, HashSet<u32>> {
    let mut hm = HashMap::new();
    for instruction in instructions {
        if let Instruction::PageOrderingRule((before, after)) = instruction {
            hm.entry(*before)
                .or_insert_with(HashSet::new)
                .insert(*after);
        }
    }
    hm
}

pub fn part_two(input: &str) -> Option<u32> {
    let instructions = parse_input(input);
    Some(correct_pages(instructions))
}

fn correct_pages(instructions: Vec<Instruction>) -> u32 {
    let mut corrected = 0;
    let check = build_check_map(&instructions);
    for instruction in &instructions {
        let Instruction::Updates(pages) = instruction else {
            continue;
        };
        let corrected_page = pages.iter().map(Cell::new).collect_vec();
        let has_correction = swap_correct(&check, &corrected_page);
        if has_correction {
            while swap_correct(&check, &corrected_page) {
                // fixing correct page
            }
            corrected += corrected_page[pages.len() / 2].get();
        }
    }
    corrected
}

fn swap_correct(check: &HashMap<u32, HashSet<u32>>, corrected_page: &Vec<Cell<&u32>>) -> bool {
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

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect()
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
