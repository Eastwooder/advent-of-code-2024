use arrayvec::ArrayVec;
use glam::{IVec2, UVec2};
use pathfinding::directed::bfs::bfs;
use roaring::RoaringBitmap;
use rustc_hash::FxHashMap;

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<usize> {
    let (map, end, _) = parse_input::<71, 71, 1024>(input);
    Some(find_exit(map, end))
}

pub fn part_two(input: &str) -> Option<String> {
    let (map, end, rest) = parse_input::<71, 71, 1024>(input);
    Some(find_exit_try(map, end, rest).to_string())
}

fn find_exit(map: Map2, end: Pos) -> usize {
    let map2 = map.clone();
    let succ = successor_bfs(move |pos| map.in_bounds(pos) && map.is_walkable(pos));
    let path = bfs(&Pos::new(0, 0), succ, |&pos| pos == end).unwrap();
    let p: FxHashMap<_, _> = path.iter().map(|&pos| (pos, 'O')).collect();
    map2.print_map(p);
    path.len() - 1
}

fn find_exit_try(mut map: Map2, end: Pos, rest: Vec<Pos>) -> String {
    for r in rest {
        map.set_unwalkable(r);
        let succ = successor_bfs(|pos| map.in_bounds(pos) && map.is_walkable(pos));
        if bfs(&Pos::new(0, 0), succ, |&pos| pos == end).is_none() {
            return format!("{},{}", r.x, r.y);
        }
    }
    panic!("No solution found")
}

fn successor_bfs(is_valid: impl Fn(Pos) -> bool) -> impl Fn(&Pos) -> ArrayVec<Pos, 4> {
    const DIRS: [Pos; 4] = [
        Pos::new(0, -1),
        Pos::new(0, 1),
        Pos::new(-1, 0),
        Pos::new(1, 0),
    ];
    move |&pos: &Pos| -> ArrayVec<Pos, 4> {
        let mut candidates = ArrayVec::new();
        for dir in DIRS {
            let to = pos + dir;
            if is_valid(to) {
                candidates.push(to);
            }
        }
        candidates
    }
}

fn parse_input<const W: u32, const H: u32, const DIGEST: usize>(
    input: &str,
) -> (Map2, Pos, Vec<Pos>) {
    let mut map = Map2 {
        map: RoaringBitmap::new(),
        dimension: UVec2::new(W, H),
    };
    for y in 0..H {
        for x in 0..W {
            map.set_walkable(Pos::new(x as i32, y as i32));
        }
    }
    for pos in stream_input(input).take(DIGEST) {
        map.set_unwalkable(pos);
    }
    (
        map,
        Pos::new(W as i32 - 1, H as i32 - 1),
        stream_input(input).skip(DIGEST).collect(),
    )
}

fn stream_input(input: &str) -> impl Iterator<Item = Pos> + '_ {
    input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(l, r)| Pos::new(l.parse().unwrap(), r.parse().unwrap()))
}

type Pos = IVec2;

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
    fn set_unwalkable(&mut self, pos: Pos) {
        self.map.remove(self.to_idx(pos));
    }

    #[inline]
    fn set_walkable(&mut self, pos: Pos) {
        self.map.insert(self.to_idx(pos));
    }

    #[inline]
    fn is_walkable(&self, pos: Pos) -> bool {
        self.map.contains(self.to_idx(pos))
    }

    #[inline]
    fn in_bounds(&self, pos: Pos) -> bool {
        pos.x >= 0
            && pos.y >= 0
            && pos.x < self.dimension.x as i32
            && pos.y < self.dimension.y as i32
    }

    #[allow(unused)]
    fn print_map(&self, overlay: FxHashMap<Pos, char>) {
        for y in 0..self.dimension.y {
            for x in 0..self.dimension.x {
                let pos = Pos::new(x as i32, y as i32);
                if let Some(c) = overlay.get(&pos) {
                    print!("{}", c);
                } else if self.is_walkable(pos) {
                    print!(".");
                } else {
                    print!("#");
                }
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let (map, end, _) =
            parse_input::<7, 7, 12>(&advent_of_code::template::read_file("examples", DAY));
        let result = find_exit(map, end);
        assert_eq!(result, 22);
    }

    #[test]
    fn test_part_two() {
        let (map, end, stream) =
            parse_input::<7, 7, 12>(&advent_of_code::template::read_file("examples", DAY));
        let result = find_exit_try(map, end, stream);
        assert_eq!(result, "6,1".to_string());
    }

    #[test]
    fn test_cost() {
        let end = Pos::new(6, 6);
        assert_eq!(Pos::new(0, 0).distance_squared(end), 72);
        assert_eq!(Pos::new(1, 0).distance_squared(end), 61);
        assert_eq!(Pos::new(0, 1).distance_squared(end), 61);
        assert_eq!(Pos::new(1, 1).distance_squared(end), 50);
        assert_eq!(Pos::new(1, 2).distance_squared(end), 41);
        assert_eq!(Pos::new(2, 1).distance_squared(end), 41);
        assert_eq!(Pos::new(2, 2).distance_squared(end), 32);
    }
}
