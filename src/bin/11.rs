use itertools::Itertools;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let stones = parse_input(input.trim());
    Some(blink(stones))
}

pub fn part_two(_: &str) -> Option<u64> {
    None
}

fn blink(mut stones: Vec<u64>) -> u64 {
    for _ in 0..25 {
        let mut next = Vec::with_capacity(stones.len() * 3 / 2);
        for stone in &stones {
            match stone {
                0 => next.push(1),
                &n if (n.ilog(10) + 1) % 2 == 0 => {
                    let (first, second) = split_half(n);
                    next.push(first);
                    next.push(second);
                }
                other => next.push(*other * 2024),
            }
        }
        stones = next;
    }
    stones.len() as _
}

fn split_half(number: u64) -> (u64, u64) {
    let num_digits = (number.ilog(10) + 1) as usize;
    debug_assert!(num_digits % 2 == 0);
    debug_assert!(num_digits > 0);
    let number = number.to_string();
    let (first, second) = number.split_at(num_digits / 2);
    (first.parse().unwrap(), second.parse().unwrap())
}

fn parse_input(input: &str) -> Vec<u64> {
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
        assert_eq!(result, None);
    }
}
