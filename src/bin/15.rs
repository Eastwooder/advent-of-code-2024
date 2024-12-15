use glam::IVec2;
use rustc_hash::{FxHashMap, FxHashSet};
use std::cell::Cell;
use std::str::FromStr;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    let (map, robot, instructions) = parse_input(input);
    Some(calculate_warehose(map, robot, instructions))
}

pub fn part_two(_: &str) -> Option<u32> {
    None
}

fn calculate_warehose(mut map: Map, mut robot: RobotPos, instructions: Vec<Direction>) -> u32 {
    // print_map(&map, robot);
    for instruction in instructions {
        robot = consume_instruction(instruction, robot, &mut map);
        // print_map(&map, robot);
    }
    gps(&map)
}

fn gps(map: &Map) -> u32 {
    map.iter()
        .filter(|(_, kind)| kind.get() == Kind::Crate)
        .map(|(pos, _)| pos)
        .map(|pos| pos.x as u32 + (pos.y as u32 * 100))
        .sum()
}

fn consume_instruction(instruction: Direction, robot_pos: RobotPos, map: &mut Map) -> RobotPos {
    let (&pos, kind) = map
        .get_key_value(&(robot_pos + instruction.to_dir()))
        .iter()
        .copied()
        .next()
        .unwrap();
    match kind.get() {
        Kind::Wall => robot_pos,
        Kind::Crate => move_crates(robot_pos, pos, map),
        Kind::Air => pos,
    }
}

fn move_crates(robot_pos: RobotPos, target_pos: Pos, map: &mut Map) -> RobotPos {
    let dir: Direction = (robot_pos, target_pos).try_into().unwrap();
    // println!("dir: {dir:?}");
    let neighbours = find_neighbours(target_pos, Kind::Crate, map, dir);
    if let Some(last) = neighbours.last().cloned() {
        match map[&(last + dir.to_dir())].get() {
            Kind::Wall => robot_pos,
            Kind::Crate => panic!("shouldn't have happened"),
            Kind::Air => {
                // println!("neighbours: {neighbours:?}");
                for n in neighbours.iter().rev() {
                    map[n].swap(&map[&(n + dir.to_dir())]);
                }
                target_pos
            }
        }
    } else {
        target_pos
    }
}

fn find_neighbours(point: Pos, kind: Kind, map: &Map, dir: Direction) -> Vec<Pos> {
    let mut visited = FxHashSet::default();
    let mut stack = vec![point];
    let mut result = vec![];
    while let Some(curr) = stack.pop() {
        if map[&curr].get() == kind {
            if visited.insert(curr) {
                result.push(curr);
            }
            if let Some((&next, _)) = map.get_key_value(&(curr + dir.to_dir())) {
                stack.push(next);
            }
        }
    }
    result
}

#[allow(unused)]
fn print_map(map: &Map, robot: RobotPos) {
    let width = map.keys().map(|l| l.x).max().unwrap();
    let height = map.keys().map(|l| l.y).max().unwrap();
    for y in 0..=height {
        for x in 0..=width {
            let cur = Pos::new(x, y);
            if cur == robot {
                print!("@");
            } else {
                let c = match map[&cur].get() {
                    Kind::Wall => '#',
                    Kind::Crate => 'O',
                    Kind::Air => '.',
                };
                print!("{c}");
            }
        }
        println!()
    }
    println!()
}

type Pos = IVec2;
type RobotPos = Pos;
type Map = FxHashMap<Pos, Cell<Kind>>;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Kind {
    Wall,
    Crate,
    Air,
}

impl FromStr for Kind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            Err("Only match one character at a time".to_string())
        } else {
            match s.chars().next().unwrap() {
                'O' => Ok(Kind::Crate),
                '#' => Ok(Kind::Wall),
                '.' => Ok(Kind::Air),
                other => Err(format!("Unknown kind: {}", other)),
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_dir(self) -> Pos {
        match self {
            Direction::Up => Pos::new(0, -1),
            Direction::Down => Pos::new(0, 1),
            Direction::Left => Pos::new(-1, 0),
            Direction::Right => Pos::new(1, 0),
        }
    }
}

impl TryFrom<(RobotPos, Pos)> for Direction {
    type Error = Pos;

    fn try_from((robot_pos, target_pos): (RobotPos, Pos)) -> Result<Self, Self::Error> {
        let dir = target_pos - robot_pos;
        match (dir.x, dir.y) {
            (0, -1) => Ok(Direction::Up),
            (0, 1) => Ok(Direction::Down),
            (-1, 0) => Ok(Direction::Left),
            (1, 0) => Ok(Direction::Right),
            (x, y) => Err(Pos::new(x, y)),
        }
    }
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            Err("Only match one character at a time".to_string())
        } else {
            match s.chars().next().unwrap() {
                '<' => Ok(Direction::Left),
                '^' => Ok(Direction::Up),
                '>' => Ok(Direction::Right),
                'v' => Ok(Direction::Down),
                other => Err(format!("Unknown instruction: {}", other)),
            }
        }
    }
}

fn parse_input(input: &str) -> (Map, RobotPos, Vec<Direction>) {
    let (input_map, input_instructions) = input.split_once("\n\n").unwrap();
    let mut map = Map::default();
    let mut robot_pos = None;
    for (y, line) in input_map.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '@' {
                map.insert(Pos::new(x as _, y as _), Cell::new(Kind::Air));
                robot_pos = Some(Pos::new(x as _, y as _));
            } else {
                let c = c.to_string().parse().unwrap();
                map.insert(Pos::new(x as _, y as _), Cell::new(c));
            }
        }
    }
    let instructions = input_instructions
        .chars()
        .map(|c| c.to_string().parse())
        .filter(Result::is_ok)
        .collect::<Result<_, _>>();
    (map, robot_pos.unwrap(), instructions.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
