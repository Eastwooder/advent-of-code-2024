use std::cell::Cell;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let ids = parse_input(input);
    Some(defrag_2(ids))
}

pub fn part_two(_: &str) -> Option<u64> {
    None
}

fn defrag_2(ids: Vec<Cell<Option<FileId>>>) -> u64 {
    let mut front = ids.iter().enumerate();
    let mut back = ids.iter().enumerate().rev();
    'outer: loop {
        let Some((f_idx, f_val)) = front.next() else {
            break;
        };
        if f_val.get().is_none() {
            let mut b = back.next();
            'inner: loop {
                let Some((b_idx, b_val)) = b else {
                    break 'outer;
                };
                if f_idx >= b_idx {
                    break 'outer;
                }
                if b_val.get().is_some() {
                    f_val.swap(b_val);
                    break 'inner;
                }
                b = back.next();
            }
        }
    }
    ids.into_iter()
        .flat_map(|x| x.get())
        .enumerate()
        .map(|(idx, file_id)| idx as u64 * file_id as u64)
        .sum()
}

type FileId = u16;
type Ids = Vec<Cell<Option<FileId>>>;

fn parse_input(input: &str) -> Ids {
    let mut ids = Ids::with_capacity(u16::MAX as usize);
    let mut is_file = true;
    let mut current = 0;
    let mut file_id = 0;
    for line in input.lines() {
        for num in line.chars() {
            let num = num.to_digit(10).unwrap();
            let id = if is_file {
                let x = Some(file_id);
                file_id += 1;
                x
            } else {
                None
            };
            for i in current..(current + num) {
                ids.insert(i as _, Cell::new(id));
            }
            current += num;
            is_file = !is_file;
        }
    }
    ids
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
