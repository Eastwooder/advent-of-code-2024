use std::fmt::Write;
use std::ops::AddAssign;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    Some(find_xmas(input))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(find_mas_in_x(input))
}

fn as_matrix(input: &str) -> Box<[Box<[char]>]> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_xmas(input: &str) -> u32 {
    let matrix = as_matrix(input);
    let mut count = 0;
    let cols = matrix[0].len();
    let rows = matrix.len();
    let horizontal = String::with_capacity(4);
    let vertical = String::with_capacity(4);
    let diagonal_tl2br = String::with_capacity(4);
    let diagonal_bl2tr = String::with_capacity(4);
    let mut net = [horizontal, vertical, diagonal_tl2br, diagonal_bl2tr];
    for col in 0..cols {
        for row in 0..rows {
            fill_net(&matrix, &mut net, (col, row));
            count.add_assign(
                net.iter()
                    .filter(|&s| matches!(s.as_str(), "XMAS" | "SAMX"))
                    .count() as u32,
            );
            net.iter_mut().for_each(|s| s.clear());
        }
    }
    count
}

fn fill_net(matrix: &[Box<[char]>], net: &mut [String; 4], (col, row): (usize, usize)) {
    let cols = matrix[col].len();
    let rows = matrix.len();
    if col + 3 < cols {
        write!(
            &mut net[0],
            "{}{}{}{}",
            matrix[row][col],
            matrix[row][col + 1],
            matrix[row][col + 2],
            matrix[row][col + 3]
        )
        .unwrap();
    }
    if row + 3 < rows {
        write!(
            &mut net[1],
            "{}{}{}{}",
            matrix[row][col],
            matrix[row + 1][col],
            matrix[row + 2][col],
            matrix[row + 3][col]
        )
        .unwrap();
    }
    if col + 3 < cols && row + 3 < rows {
        write!(
            &mut net[2],
            "{}{}{}{}",
            matrix[row][col],
            matrix[row + 1][col + 1],
            matrix[row + 2][col + 2],
            matrix[row + 3][col + 3]
        )
        .unwrap();
    }
    if col >= 3 && row + 3 < rows {
        write!(
            &mut net[3],
            "{}{}{}{}",
            matrix[row][col],
            matrix[row + 1][col - 1],
            matrix[row + 2][col - 2],
            matrix[row + 3][col - 3]
        )
        .unwrap();
    }
}

fn find_mas_in_x(input: &str) -> u32 {
    let matrix = as_matrix(input);
    let mut count = 0;
    let cols = matrix[0].len();
    let rows = matrix.len();
    let diagonal_1 = String::with_capacity(4);
    let diagonal_2 = String::with_capacity(4);
    let mut net = [diagonal_1, diagonal_2];
    for col in 0..cols - 2 {
        for row in 0..rows - 2 {
            fill_net_x(&matrix, &mut net, (col, row));
            if matches!(net[0].as_str(), "MAS" | "SAM") && matches!(net[1].as_str(), "MAS" | "SAM")
            {
                count += 1;
            }
            net.iter_mut().for_each(|s| s.clear())
        }
    }
    count
}

fn fill_net_x(matrix: &[Box<[char]>], net: &mut [String; 2], (col, row): (usize, usize)) {
    write!(
        &mut net[0],
        "{}{}{}",
        matrix[row][col],
        matrix[row + 1][col + 1],
        matrix[row + 2][col + 2]
    )
    .unwrap();
    write!(
        &mut net[1],
        "{}{}{}",
        matrix[row][col + 2],
        matrix[row + 1][col + 1],
        matrix[row + 2][col]
    )
    .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
