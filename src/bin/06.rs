use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rustc_hash::FxHashSet;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let (map_size, guard_pos, obstacles) = parse_input(input);
    Some(visit_field(map_size, guard_pos, obstacles))
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map_size, guard_pos, obstacles) = parse_input(input);
    Some(count_obstacles(map_size, guard_pos, obstacles))
}

type HashSet<T> = FxHashSet<T>;
type Guard = (Pos2, Direction);
type Width = u32;
type Height = u32;
type MapSize = (Width, Height);
type Pos = (Width, Height);
type Pos2 = u32;
type Obstacles = HashSet<Pos2>;

fn visit_field(map_size: MapSize, guard: Guard, obstacles: Obstacles) -> u32 {
    let mut path = HashSet::default();
    let mut guard_walk = Some(guard);
    while let Some(guard) = guard_walk {
        guard_walk = trace_simple(&map_size, guard, &obstacles, &mut path);
    }
    path.len() as _
}

fn trace_simple(
    map_size: &MapSize,
    guard: Guard,
    obstacles: &Obstacles,
    path: &mut HashSet<Pos>,
) -> Option<Guard> {
    let up_limit = 0;
    let down_limit = map_size.1 - 1;
    let left_limit = 0;
    let right_limit = map_size.0 - 1;
    let (mut x, mut y) = map_size.to_coord(guard.0);
    loop {
        let dir = guard.1;
        path.insert((x, y));
        match (dir, x, y) {
            (Direction::Up, _, up) if up == up_limit => return None,
            (Direction::Down, _, down) if down == down_limit => return None,
            (Direction::Left, left, _) if left == left_limit => return None,
            (Direction::Right, right, _) if right == right_limit => return None,
            (_, width, _) if width == 0 || width == map_size.0 - 1 => return None,
            (_, _, height) if height == 0 || height == map_size.1 - 1 => return None,
            _ => (),
        }
        let (new_x, new_y) = guard.1.offset_calc((x, y));
        if obstacles.contains(&map_size.to_pos((new_x, new_y))) {
            return Some((map_size.to_pos((x, y)), guard.1.turn()));
        }
        x = new_x;
        y = new_y;
    }
}

trait MapSizeExt {
    fn to_pos(&self, pos: Pos) -> Pos2;
    fn to_coord(&self, pos: Pos2) -> Pos;
}
impl MapSizeExt for MapSize {
    #[inline]
    fn to_pos(&self, pos: Pos) -> Pos2 {
        assert!(self.0 >= pos.0);
        assert!(self.1 >= pos.1);
        let (x, y) = pos;
        x + (y * self.0)
    }

    #[inline]
    fn to_coord(&self, pos: Pos2) -> Pos {
        (pos % self.0, pos / self.0)
    }
}

#[inline]
fn count_obstacles(map_size: MapSize, guard_start: Guard, obstacles: Obstacles) -> u32 {
    let mut path = HashSet::with_capacity_and_hasher(obstacles.len() / 8, Default::default());
    let mut guard_walk = Some(guard_start);

    while let Some(guard) = guard_walk {
        guard_walk = trace(&map_size, guard, &obstacles, guard.0, &mut path);
    }

    path.par_iter()
        .filter(|&&pos| pos != guard_start.0)
        .filter(|&&pos| simulate_walk_with_obstacle(map_size, guard_start, &obstacles, pos))
        .count() as u32
}

#[inline]
fn simulate_walk_with_obstacle(
    map_size: MapSize,
    guard_start: Guard,
    obstacles: &Obstacles,
    additional_obstacle: Pos2,
) -> bool {
    let mut guard_walk = Some(guard_start);
    let mut loop_check = HashSet::with_capacity_and_hasher(obstacles.len() / 8, Default::default());
    while let Some(guard) = guard_walk {
        if loop_check.contains(&guard) {
            return true;
        }
        loop_check.insert(guard);
        guard_walk = trace_no_track(&map_size, guard, obstacles, additional_obstacle);
    }
    false
}

#[inline]
fn trace(
    map_size: &MapSize,
    guard: Guard,
    obstacles: &Obstacles,
    additional_obstacle: Pos2,
    path: &mut HashSet<Pos2>,
) -> Option<Guard> {
    let up_limit = 0;
    let down_limit = map_size.1 - 1;
    let left_limit = 0;
    let right_limit = map_size.0 - 1;

    let (mut curr_x, mut curr_y) = map_size.to_coord(guard.0);
    let curr_dir = guard.1;

    loop {
        path.insert(map_size.to_pos((curr_x, curr_y)));

        match (curr_dir, curr_x, curr_y) {
            (Direction::Up, _, up) if up == up_limit => return None,
            (Direction::Down, _, down) if down == down_limit => return None,
            (Direction::Left, left, _) if left == left_limit => return None,
            (Direction::Right, right, _) if right == right_limit => return None,
            _ => (),
        }

        let (new_x, new_y) = curr_dir.offset_calc((curr_x, curr_y));
        let new_pos = map_size.to_pos((new_x, new_y));
        if additional_obstacle == new_pos || obstacles.contains(&new_pos) {
            return Some((map_size.to_pos((curr_x, curr_y)), curr_dir.turn()));
        }
        (curr_x, curr_y) = (new_x, new_y);
    }
}

#[inline]
fn trace_no_track(
    map_size: &MapSize,
    guard: Guard,
    obstacles: &Obstacles,
    additional_obstacle: Pos2,
) -> Option<Guard> {
    let up_limit = 0;
    let down_limit = map_size.1 - 1;
    let left_limit = 0;
    let right_limit = map_size.0 - 1;

    let (mut curr_x, mut curr_y) = map_size.to_coord(guard.0);
    let curr_dir = guard.1;

    loop {
        match (curr_dir, curr_x, curr_y) {
            (Direction::Up, _, up) if up == up_limit => return None,
            (Direction::Down, _, down) if down == down_limit => return None,
            (Direction::Left, left, _) if left == left_limit => return None,
            (Direction::Right, right, _) if right == right_limit => return None,
            _ => (),
        }

        let (new_x, new_y) = curr_dir.offset_calc((curr_x, curr_y));
        let new_pos = map_size.to_pos((new_x, new_y));
        if additional_obstacle == new_pos || obstacles.contains(&new_pos) {
            return Some((map_size.to_pos((curr_x, curr_y)), curr_dir.turn()));
        }
        (curr_x, curr_y) = (new_x, new_y);
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    #[inline]
    fn turn(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    #[inline]
    fn offset_calc(self, (x, y): Pos) -> Pos {
        match self {
            Direction::Up => (x, y.saturating_sub(1)),
            Direction::Down => (x, y.saturating_add(1)),
            Direction::Left => (x.saturating_sub(1), y),
            Direction::Right => (x.saturating_add(1), y),
        }
    }
}

fn parse_input(input: &str) -> (MapSize, Guard, Obstacles) {
    let width = input
        .find(char::is_whitespace)
        .expect("expected there to be at least one line")
        .try_into()
        .unwrap();
    let mut height = 0;
    let mut guard_pos = None;
    let mut obstacles = HashSet::default();
    for line in input.lines() {
        for (idx, field) in line.chars().enumerate() {
            match field {
                '.' => continue,
                '#' => {
                    obstacles.insert((width, height).to_pos((idx as _, height)));
                }
                '^' => {
                    let _ = guard_pos.insert((width, height).to_pos((idx as _, height)));
                }
                other => panic!("unexpected field {other}"),
            };
        }
        height += 1;
    }
    let guard_pos = guard_pos.expect("expected guard to be present");
    ((width, height as _), (guard_pos, Direction::Up), obstacles)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
