use roaring::{MultiOps, RoaringBitmap};
use std::cell::Cell;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u32> {
    let (disk, ids) = parse_input(input);
    let defraged = defrag(disk, &ids);
    Some(calc_checksum(defraged))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn defrag(mut disk: RoaringBitmap, ids: &[Cell<Option<FileId>>]) -> Vec<FileId> {
    let mut x = 0;
    loop {
        let free = RoaringBitmap::from_iter(0..=disk.max().unwrap());
        let free = [free, disk.clone()].into_iter().difference();
        println!("defragging {} elements", free.len());
        if free.is_empty() {
            break;
        }
        for f in free {
            let Some(last) = disk.iter().last() else {
                panic!("expected there to be a last element")
            };
            // println!("free {last} for {f}");
            assert!(disk.remove(last));
            assert!(disk.insert(f));
            let _ = &ids[last as usize].swap(&ids[f as usize]);
        }
    }
    for id in ids {
        if let Some(id) = id.get() {
            print!("{id}")
        } else {
            print!(" ")
        }
    }
    ids.into_iter().flat_map(|x| x.get()).collect()
}

fn calc_checksum(disk: Vec<FileId>) -> u32 {
    disk.into_iter()
        .enumerate()
        .map(|(idx, file_id)| idx as u32 * file_id as u32)
        .sum()
}

type Disk = RoaringBitmap;
type FileId = u16;
type Ids = Vec<Cell<Option<FileId>>>;

fn parse_input(input: &str) -> (Disk, Ids) {
    let mut disk = Disk::new();
    let mut ids = Ids::with_capacity(u16::MAX as usize);
    let mut is_file = true;
    let mut current = 0;
    let mut file_id = 0;
    for line in input.lines() {
        for num in line.chars() {
            let num = num.to_digit(10).unwrap();
            let id = if is_file {
                disk.insert_range(current..(current + num));
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
    (disk, ids)
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
