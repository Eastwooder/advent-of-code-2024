use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let (map_size, guard_pos, obstacles) = parse_input(input);
    Some(visit_field(map_size, guard_pos, obstacles))
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map_size, guard_pos, obstacles) = parse_input(input);
    Some(count_obstacles(map_size, guard_pos, obstacles))
}

fn count_obstacles(map_size: MapSize, guard: Guard, obstacles: Obstacles) -> u32 {
    let mut path = HashMap::new();
    let mut guard_walk = Some(guard);
    while let Some(guard) = guard_walk {
        guard_walk = trace(&map_size, guard, &obstacles, &mut path);
    }
    path.par_iter()
        .filter(|(pos, _)| **pos != guard.0)
        .map(|(pos, _)| pos)
        .filter(|pos| simulate_walk_with_obstacle(map_size, guard, &obstacles, **pos))
        .count() as _
}

#[allow(unused)]
fn pos_on_map(input: &str, pos: Pos) -> char {
    for (h_pos, line) in input.lines().enumerate() {
        for (w_pos, ch) in line.chars().enumerate() {
            if (h_pos as u32) == pos.1 && (w_pos as u32) == pos.0 {
                return ch;
            }
        }
    }
    todo!()
}

#[allow(unused)]
fn print_map(input: &str, map_size: MapSize, replacement: Option<(Pos, char)>) {
    for (height, line) in input.lines().enumerate() {
        for (width, chr) in line.chars().enumerate() {
            if let Some((pos, repl)) = replacement {
                if pos == (width as Width, height as Height) {
                    print!("{}", repl);
                } else {
                    print!("{}", chr);
                }
            } else {
                print!("{}", chr);
            }
        }
        println!();
    }
}

fn simulate_walk_with_obstacle(
    map_size: MapSize,
    guard: Guard,
    obstacles: &Obstacles,
    additional_obstacle: Pos,
) -> bool {
    let mut path = HashMap::new();
    let mut guard_walk = Some(guard);
    let mut obstacles = obstacles.clone();
    obstacles.insert(additional_obstacle);
    let mut loop_check = HashSet::new();
    while let Some(guard) = guard_walk {
        if loop_check.contains(&guard) {
            return true;
        }
        loop_check.insert(guard);
        guard_walk = trace(&map_size, guard, &obstacles, &mut path);
    }
    false
}

fn visit_field(map_size: MapSize, guard: Guard, obstacles: Obstacles) -> u32 {
    let mut path = HashMap::new();
    let mut guard_walk = Some(guard);
    while let Some(guard) = guard_walk {
        guard_walk = trace(&map_size, guard, &obstacles, &mut path);
    }
    path.len() as _
}

fn trace(
    map_size: &MapSize,
    guard: Guard,
    obstacles: &Obstacles,
    path: &mut HashMap<Pos, usize>,
) -> Option<Guard> {
    let up_limit = 0;
    let down_limit = map_size.1 - 1;
    let left_limit = 0;
    let right_limit = map_size.0 - 1;
    let mut x = guard.0 .0;
    let mut y = guard.0 .1;
    loop {
        let dir = guard.1;
        *path.entry((x, y)).or_insert(0) += 1;
        match (dir, x, y) {
            (Direction::Up, _, up) if up == up_limit => {
                return None;
            }
            (Direction::Down, _, down) if down == down_limit => {
                return None;
            }
            (Direction::Left, left, _) if left == left_limit => {
                return None;
            }
            (Direction::Right, right, _) if right == right_limit => {
                return None;
            }
            _ => (),
        }
        if y == 0 || y == map_size.1 - 1 || x == 0 || x == map_size.0 - 1 {
            return None;
        }
        let (new_x, new_y) = guard.1.offset_calc((x, y));
        if obstacles.contains(&(new_x, new_y)) {
            return Some(((x, y), guard.1.turn()));
        }
        x = new_x;
        y = new_y;
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
    fn turn(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn offset_calc(self, (x, y): Pos) -> Pos {
        match self {
            Direction::Up => (x, y.saturating_sub(1)),
            Direction::Down => (x, y.saturating_add(1)),
            Direction::Left => (x.saturating_sub(1), y),
            Direction::Right => (x.saturating_add(1), y),
        }
    }
}

type Guard = (Pos, Direction);
type Width = u32;
type Height = u32;
type MapSize = (Width, Height);
type Pos = (Width, Height);
type Obstacles = HashSet<Pos>;

fn parse_input(input: &str) -> (MapSize, Guard, Obstacles) {
    let width = input
        .find(char::is_whitespace)
        .expect("expected there to be at least one line")
        .try_into()
        .unwrap();
    let mut height = 0;
    let mut guard_pos = None;
    let mut obstacles = HashSet::new();
    for line in input.lines() {
        for (idx, field) in line.chars().enumerate() {
            match field {
                '.' => continue,
                '#' => {
                    obstacles.insert((idx as _, height));
                }
                '^' => {
                    let _ = guard_pos.insert((idx as _, height));
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
