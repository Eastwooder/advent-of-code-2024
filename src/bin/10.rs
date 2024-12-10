use itertools::Itertools;
use rustc_hash::{FxBuildHasher, FxHashSet};

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let (matrix, start_points) = parse_input(input);
    Some(count_trails(matrix, start_points))
}

pub fn part_two(input: &str) -> Option<u32> {
    let (matrix, start_points) = parse_input(input);
    Some(find_distinct_trails(matrix, start_points))
}

type Row = Vec<u8>;
type Matrix = Vec<Row>;
type Coords = (usize, usize);
type StartPoints = Vec<Coords>;

fn count_trails(matrix: Matrix, start_points: StartPoints) -> u32 {
    let mut sum = 0;
    for s in start_points {
        let positions = find_trail(s, &matrix);
        sum += positions.len() as u32;
    }
    sum
}

fn find_trail(current: Coords, matrix: &Matrix) -> FxHashSet<Coords> {
    let val = matrix[current.0][current.1];
    if val == 9 {
        FxHashSet::from_iter([current])
    } else {
        let next = val + 1;
        let set = find_neighbours(current, matrix, next)
            .iter()
            .flat_map(|&n| find_trail(n, matrix))
            .collect();
        set
    }
}

fn find_neighbours(current: Coords, matrix: &Matrix, target: u8) -> FxHashSet<Coords> {
    let mut neighbours = FxHashSet::with_capacity_and_hasher(9, FxBuildHasher);
    // 0 1 0
    // 1 0 1
    // 0 1 0
    let search_matrix = [
        (current.0.saturating_sub(1), current.1),
        (current.0 + 1, current.1),
        (current.0, current.1.saturating_sub(1)),
        (current.0, current.1 + 1),
    ];
    for (row, col) in search_matrix {
        if row >= matrix.len() || col >= matrix[0].len() {
            continue;
        }
        if matrix[row][col] == target {
            neighbours.insert((row, col));
        }
    }
    neighbours
}

fn parse_input(input: &str) -> (Matrix, StartPoints) {
    let mut matrix = Matrix::new();
    let mut start_points = StartPoints::new();
    for (row, line) in input.lines().enumerate() {
        let mut row_data = Row::new();
        for (col, chr) in line.chars().enumerate() {
            let chr = chr.to_digit(10).unwrap() as _;
            if chr == 0 {
                start_points.push((row, col))
            }
            row_data.push(chr);
        }
        matrix.push(row_data);
    }
    (matrix, start_points)
}

fn find_distinct_trails(matrix: Matrix, start_points: StartPoints) -> u32 {
    let mut sum = 0;
    for s in start_points {
        let trails = track_trail(s, &matrix, &[]);
        sum += trails.len() as u32;
    }
    sum
}

fn track_trail(current: Coords, matrix: &Matrix, head: &[Coords]) -> FxHashSet<Vec<Coords>> {
    let val = matrix[current.0][current.1];
    let mut trail = head.iter().copied().collect_vec();
    trail.push(current);
    if val == 9 {
        FxHashSet::from_iter([trail])
    } else {
        let next = val + 1;
        let set = find_neighbours(current, matrix, next)
            .iter()
            .flat_map(|&n| track_trail(n, matrix, &trail))
            .collect();
        set
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
