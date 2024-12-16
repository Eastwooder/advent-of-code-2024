use arrayvec::ArrayVec;
use glam::{IVec2, UVec2};
use pathfinding::prelude::{astar_bag, dijkstra};
use roaring::RoaringBitmap;
use rustc_hash::FxHashSet;

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<Cost> {
    let (map, start, end) = parse_input(input);
    Some(find_target(map, start, end))
}

pub fn part_two(input: &str) -> Option<usize> {
    let (map, start, end) = parse_input(input);
    Some(find_target_route(map, start, end))
}

fn find_target(map: Map2, start: Elk, end: End) -> Cost {
    let succ = successor_fn(move |pos| map.is_walkable(pos));
    if let Some((_, cost)) = dijkstra(&start, succ, |&(pos, _)| pos == end) {
        cost
    } else {
        panic!("unable to find shortest path to target!");
    }
}

fn successor_fn(is_valid: impl Fn(Pos) -> bool) -> impl Fn(&Elk) -> ArrayVec<(Elk, Cost), 3> {
    move |&(pos, dir): &(Pos, Direction)| -> ArrayVec<((Pos, Direction), Cost), 3> {
        let mut candidates = ArrayVec::new();
        let forward = pos + dir.to_dir();
        if is_valid(pos) {
            candidates.push(((forward, dir), 1));
        }
        candidates.push(((pos, dir.rotate_left()), 1000));
        candidates.push(((pos, dir.rotate_right()), 1000));
        candidates
    }
}

fn find_target_route(map: Map2, start: Elk, end: End) -> usize {
    let succ = successor_fn(move |pos| map.is_walkable(pos));
    let mut visited = FxHashSet::default();
    for path in astar_bag(
        &start,
        succ,
        |(pos, _)| pos.dot(end) as _,
        |&(pos, _)| pos == end,
    )
    .unwrap()
    .0
    {
        visited.extend(path.iter().map(|&(pos, _)| pos));
    }
    visited.len()
}

type Cost = u64;
type Pos = IVec2;
type Elk = (Pos, Direction);
type End = Pos;

#[derive(Clone, Debug)]
struct Map2 {
    map: RoaringBitmap,
    dimension: UVec2,
}

impl Map2 {
    #[inline]
    fn to_idx(&self, pos: Pos) -> u32 {
        debug_assert!(self.dimension.x > pos.x as u32);
        debug_assert!(self.dimension.y > pos.y as u32);
        pos.x as u32 + (pos.y as u32 * self.dimension.x)
    }

    #[allow(unused)]
    #[inline]
    fn to_pos(&self, idx: u32) -> Pos {
        debug_assert!(self.dimension.x * self.dimension.y > idx);
        Pos::new(
            (idx % self.dimension.x) as i32,
            (idx / self.dimension.x) as i32,
        )
    }

    #[inline]
    fn set_walkable(&mut self, pos: Pos) {
        self.map.insert(self.to_idx(pos));
    }

    #[inline]
    fn is_walkable(&self, pos: Pos) -> bool {
        self.map.contains(self.to_idx(pos))
    }
}

fn parse_input(input: &str) -> (Map2, Elk, End) {
    let mut start = None;
    let mut end = None;
    let width = input.find('\n').unwrap();
    let height = input.len() / width;
    let mut map2 = Map2 {
        map: RoaringBitmap::default(),
        dimension: UVec2::new(height as u32, width as u32),
    };
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => (),
                '.' => map2.set_walkable(Pos::new(x as i32, y as i32)),
                'S' => {
                    map2.set_walkable(Pos::new(x as i32, y as i32));
                    start = Some(Pos::new(x as i32, y as i32));
                }
                'E' => {
                    map2.set_walkable(Pos::new(x as i32, y as i32));
                    end = Some(Pos::new(x as i32, y as i32));
                }
                other => panic!("unsupported character found: {other}"),
            }
        }
    }
    (map2, (start.unwrap(), Direction::Right), end.unwrap())
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
    use itertools::Itertools;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }

    #[test]
    fn test_map() {
        let mut map = Map2 {
            map: Default::default(),
            dimension: UVec2::new(7, 3),
        };
        assert_eq!(map.to_idx(Pos::new(5, 0)), 5);
        assert_eq!(map.to_idx(Pos::new(5, 1)), 12);
        assert_eq!(map.to_idx(Pos::new(5, 2)), 19);
        assert_eq!(map.to_idx(Pos::new(6, 2)), 20);
        assert_eq!(map.to_pos(5), Pos::new(5, 0));
        assert_eq!(map.to_pos(6), Pos::new(6, 0));
        assert_eq!(map.to_pos(8), Pos::new(1, 1));
        assert_eq!(map.to_pos(12), Pos::new(5, 1));
        assert_eq!(map.to_pos(19), Pos::new(5, 2));
        assert_eq!(map.to_pos(20), Pos::new(6, 2));

        map.set_walkable(Pos::new(5, 0));
        map.set_walkable(Pos::new(5, 1));
        map.set_walkable(Pos::new(5, 2));
        map.set_walkable(Pos::new(2, 1));
        map.set_walkable(Pos::new(3, 1));
        map.set_walkable(Pos::new(4, 1));
        assert_eq!(
            map.map.iter().map(|i| map.to_pos(i)).collect_vec(),
            vec![
                Pos::new(5, 0),
                Pos::new(2, 1),
                Pos::new(3, 1),
                Pos::new(4, 1),
                Pos::new(5, 1),
                Pos::new(5, 2),
            ]
        );
    }
}
