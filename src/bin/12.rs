use glam::{IVec2, UVec2};
use rustc_hash::{FxBuildHasher, FxHashSet};

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u64> {
    let plot_matrix = parse_input(input);
    Some(calculate_cost(plot_matrix))
}

pub fn part_two(_: &str) -> Option<u32> {
    None
}

type Matrix = Vec<Vec<char>>;
type Pos = UVec2;
type PosOffset = IVec2;
type Area = u32;
type Perimeter = u32;
type Cost = u64;

fn calculate_cost(plot_matrix: Matrix) -> u64 {
    let mut price = 0u64;
    let mut mem_visisted = FxHashSet::with_capacity_and_hasher(plot_matrix.len(), FxBuildHasher);
    for row in 0..plot_matrix.len() {
        for col in 0..plot_matrix[0].len() {
            let pos = PosOffset::new(row as i32, col as i32);
            if !mem_visisted.contains(&pos) {
                price +=
                    visit_and_calculate_search(pos.as_uvec2(), &plot_matrix, &mut mem_visisted);
            }
        }
    }
    price
}

fn visit_and_calculate_search(
    start: Pos,
    map: &Matrix,
    mem_visit: &mut FxHashSet<PosOffset>,
) -> Cost {
    mem_visit.insert(start.as_ivec2());
    let mut stack = {
        let mut v = Vec::with_capacity(map.len() / 8);
        v.push(start.as_ivec2());
        v
    };
    let mut area = 0 as Area;
    let mut perimiter = 0 as Perimeter;
    let plant = map[start.x as usize][start.y as usize];

    while let Some(curr) = stack.pop() {
        perimiter += 4;
        area += 1;
        find_direct_neighbours(curr.as_uvec2(), map, plant).for_each(|neighbor| {
            if mem_visit.insert(neighbor) {
                stack.push(neighbor);
            }
            perimiter -= 1;
        });
    }
    area as Cost * perimiter as Cost
}

fn find_direct_neighbours(
    pos: Pos,
    map: &Matrix,
    plant: char,
) -> impl Iterator<Item = PosOffset> + use<'_> {
    const TOP: PosOffset = PosOffset::new(0, -1);
    const RIGHT: PosOffset = PosOffset::new(-1, 0);
    const BOT: PosOffset = PosOffset::new(0, 1);
    const LEFT: PosOffset = PosOffset::new(1, 0);
    [TOP, RIGHT, BOT, LEFT]
        .into_iter()
        .map(move |dir| pos.as_ivec2() + dir)
        .filter(|kind| (kind.x as usize) < map.len())
        .filter(|kind| (kind.y as usize) < map[0].len())
        .filter(move |kind| map[kind.x as usize][kind.y as usize] == plant)
}

fn parse_input(input: &str) -> Matrix {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
