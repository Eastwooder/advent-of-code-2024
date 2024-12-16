use glam::IVec2;
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<Cost> {
    let (map, start, end) = parse_input(input);
    Some(find_target(map, start, end))
}

pub fn part_two(_: &str) -> Option<Cost> {
    None
}

fn find_target(map: Map, start: Elk, end: End) -> Cost {
    let mut visited = FxHashSet::default();
    let mut point_costs = FxHashMap::default();
    let mut queue = BinaryHeap::new();
    queue.push(Reverse(PointCost {
        pos: start.0,
        dir: start.1,
        cost: 0,
    }));
    while let Some(Reverse(PointCost { pos, dir, cost })) = queue.pop() {
        // println!("run {cnt}: {pos} {}", map[pos.y as usize][pos.x as usize]);
        // let mut overlay = FxHashMap::default();
        // overlay.insert(pos, 'X');
        // print_map(&map, overlay);
        if !visited.insert((pos, dir)) {
            continue;
        }
        if map[pos.y as usize][pos.x as usize] == '#' {
            continue;
        }
        let cost = point_costs.entry((pos, dir)).or_insert(cost);
        if pos == end {
            return *cost;
        }
        let cost = *cost;
        let next = pos + dir.to_dir();
        queue.push(Reverse(PointCost {
            pos: next,
            dir,
            cost: cost + 1,
        }));
        queue.push(Reverse(PointCost {
            pos,
            dir: dir.rotate_right(),
            cost: cost + 1000,
        }));
        queue.push(Reverse(PointCost {
            pos,
            dir: dir.rotate_left(),
            cost: cost + 1000,
        }));
    }
    panic!("end not found!")
}

#[allow(unused)]
fn print_map(map: &Map, overlay: FxHashMap<Pos, char>) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if let Some(c) = overlay.get(&(Pos::new(x as i32, y as i32))) {
                print!("{c}");
            } else {
                print!("{}", map[y][x]);
            }
        }
        println!();
    }
    println!();
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct PointCost {
    pos: Pos,
    dir: Direction,
    cost: Cost,
}

impl PartialOrd for PointCost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cost.cmp(&other.cost))
    }
}

impl Ord for PointCost {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

type Cost = u64;
type Pos = IVec2;
type Elk = (Pos, Direction);
type End = Pos;
type Map = Vec<Vec<char>>;

fn parse_input(input: &str) -> (Map, Elk, End) {
    let mut start = None;
    let mut end = None;
    let mut map = Map::new();

    for (y, line) in input.lines().enumerate() {
        map.push(vec!['.'; line.len()]);
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' | '.' => map[y][x] = c,
                'S' => {
                    start = Some(Pos::new(x as i32, y as i32));
                    map[y][x] = '.';
                }
                'E' => {
                    end = Some(Pos::new(x as i32, y as i32));
                    map[y][x] = '.';
                }
                other => panic!("unsupported character found: {other}"),
            }
        }
    }

    (map, (start.unwrap(), Direction::Right), end.unwrap())
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn to_dir(self) -> Pos {
        match self {
            Direction::Up => Pos::new(0, -1),
            Direction::Right => Pos::new(1, 0),
            Direction::Down => Pos::new(0, 1),
            Direction::Left => Pos::new(-1, 0),
        }
    }

    fn rotate_left(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }

    fn rotate_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
