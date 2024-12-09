use rustc_hash::FxHashMap;
use std::cell::Cell;
use std::collections::VecDeque;
use std::iter::{Enumerate, Peekable, Rev};
use std::slice::Iter;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let (ids, _, _) = parse_input(input);
    Some(defrag(ids))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (ids, _, _) = parse_input(input);
    Some(defrag_2(ids))
}

fn defrag_2(ids: Ids) -> u64 {
    let mut back = ids.iter().enumerate().rev().peekable();
    'outer: loop {
        let Some((to, from, _file_id)) = find_block_back(&mut back) else {
            break 'outer;
        };
        let size = (from..=to).count();
        let mut front = ids.iter().enumerate();
        let (f_from, f_to) = 'find_candidate: loop {
            let Some((f_from, f_to)) = find_free_range_forward(&mut front) else {
                continue 'outer;
            };
            let slot = (f_from..f_to).count();
            if slot < size {
                continue 'find_candidate;
            }
            if f_from >= to {
                continue 'outer;
            }
            break 'find_candidate (f_from, f_to);
        };
        for (back, front) in (from..=to).zip(f_from..f_to) {
            let back = &ids[back];
            let front = &ids[front];
            debug_assert!(back.get().is_some());
            debug_assert!(front.get().is_none());
            back.swap(front);
        }
    }
    ids.into_iter()
        .enumerate()
        .map(|(idx, file_id)| match file_id.get() {
            Some(file_id) => idx as u64 * file_id as u64,
            None => 0,
        })
        .sum()
}

fn find_free_range_forward(
    iter: &mut Enumerate<Iter<Cell<Option<FileId>>>>,
) -> Option<(usize, usize)> {
    let mut idx_from = None;
    #[allow(clippy::while_let_on_iterator)]
    while let Some((idx, inner)) = iter.next() {
        if idx_from.is_none() && inner.get().is_some() {
            continue;
        }
        if inner.get().is_none() {
            if idx_from.is_none() {
                idx_from = Some(idx);
            }
        } else {
            return Some((idx_from.unwrap(), idx));
        }
    }
    None
}

#[allow(clippy::type_complexity)]
fn find_block_back(
    iter: &mut Peekable<Rev<Enumerate<Iter<Cell<Option<FileId>>>>>>,
) -> Option<(usize, usize, FileId)> {
    let mut chr = None;
    let mut idx_from = None;
    let mut idx_to = None;
    loop {
        if let Some((idx, inner)) = iter.peek() {
            if let Some(c) = chr {
                if let Some(inner) = inner.get() {
                    if c == inner {
                        idx_to = Some(*idx);
                    } else {
                        return Some((idx_from.unwrap(), idx_to.unwrap(), c));
                    }
                }
            } else if let Some(inner) = inner.get() {
                chr = Some(inner);
                idx_from = Some(*idx);
                continue;
            }
            let _ = iter.next();
        } else {
            return None;
        }
    }
}

fn defrag(ids: Ids) -> u64 {
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
type FileMap = FxHashMap<FileId, (u32, u32)>;
type FileMapOrd = VecDeque<(FileId, (u32, u32))>;

fn parse_input(input: &str) -> (Ids, FileMap, FileMapOrd) {
    let mut ids = Ids::with_capacity(u16::MAX as usize);
    let mut file_map = FileMap::default();
    let mut file_map_ord = FileMapOrd::default();
    let mut is_file = true;
    let mut current = 0;
    let mut file_id = 0;
    for line in input.lines() {
        for num in line.chars() {
            let num = num.to_digit(10).unwrap();
            let id = if is_file {
                let x = Some(file_id);
                file_map.insert(file_id, (current, (current + num)));
                file_map_ord.push_back((file_id, (current, (current + num))));
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
    (ids, file_map, file_map_ord)
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
        assert_eq!(result, Some(2858));
    }
}
