use glam::IVec2;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<usize> {
    let robots = parse_input(input).unwrap();
    let sums = walk_robots(&robots, 100, WIDTH, HEIGHT);
    Some(sums.iter().product())
}

pub fn part_two(_: &str) -> Option<u32> {
    None
}

fn walk_robots(robots: &[(Pos, Velocity)], times: usize, width: i32, height: i32) -> [usize; 4] {
    debug_assert!(width % 2 != 0);
    debug_assert!(height % 2 != 0);
    let sep_x = width / 2;
    let sep_y = height / 2;
    let qs = [
        (Pos::new(0, 0), Pos::new(sep_x - 1, sep_y - 1)),
        (Pos::new(sep_x + 1, 0), Pos::new(width - 1, sep_y - 1)),
        (Pos::new(0, sep_y + 1), Pos::new(sep_x - 1, height - 1)),
        (
            Pos::new(sep_x + 1, sep_y + 1),
            Pos::new(width - 1, height - 1),
        ),
    ];
    let mut sums = [0, 0, 0, 0];
    for &robot in robots {
        let robot = walk_robot(robot, times, width, height);
        for (idx, q) in qs.iter().enumerate() {
            if is_in_area(robot, q.0, q.1) {
                sums[idx] += 1;
                break;
            }
        }
    }
    sums
}

fn is_in_area(robot: Pos, top_left: Pos, bot_right: Pos) -> bool {
    robot.x >= top_left.x
        && robot.x <= bot_right.x
        && robot.y >= top_left.y
        && robot.y <= bot_right.y
}

fn walk_robot(robot: (Pos, Velocity), times: usize, width: i32, height: i32) -> Pos {
    let rob = robot.0;
    let vel = robot.1;
    wrap(
        rob + vel * Pos::new(times as i32, times as i32),
        width,
        height,
    )
}

fn wrap(robot: Pos, width: i32, height: i32) -> Pos {
    debug_assert!(width > 0);
    debug_assert!(height > 0);
    robot.rem_euclid(Pos::new(width, height))
}

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

type Pos = IVec2;
type Velocity = IVec2;

fn parse_input(input: &str) -> Option<Vec<(Pos, Velocity)>> {
    let mut robots = vec![];
    for line in input.lines() {
        let (pos, vel) = line.split_once(' ')?;
        let pos = pos.strip_prefix("p=")?;
        let vel = vel.strip_prefix("v=")?;
        let (x, y) = pos.split_once(',')?;
        let x = x.parse().ok()?;
        let y = y.parse().ok()?;
        let pos = Pos::new(x, y);
        let (x, y) = vel.split_once(',')?;
        let x = x.parse().ok()?;
        let y = y.parse().ok()?;
        let vel = Velocity::new(x, y);
        robots.push((pos, vel));
    }
    Some(robots)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_one_run_robot() {
        let width = 11;
        let height = 7;
        let robots = vec![(Pos::new(2, 4), Velocity::new(2, -3))];
        let result = walk_robot(robots[0], 1, width, height);
        assert_eq!(result, Pos::new(4, 1));
        let result = walk_robot(robots[0], 2, width, height);
        assert_eq!(result, Pos::new(6, 5));
        let result = walk_robot(robots[0], 3, width, height);
        assert_eq!(result, Pos::new(8, 2));
        let result = walk_robot(robots[0], 4, width, height);
        assert_eq!(result, Pos::new(10, 6));
        let result = walk_robot(robots[0], 5, width, height);
        assert_eq!(result, Pos::new(1, 3));
    }

    #[test]
    fn test_part_one_quadrants() {
        let robots = vec![
            (Pos::new(0, 4), Velocity::new(3, -3)),
            (Pos::new(6, 3), Velocity::new(-1, -3)),
            (Pos::new(10, 3), Velocity::new(-1, 2)),
            (Pos::new(2, 0), Velocity::new(1, 3)),
            (Pos::new(0, 0), Velocity::new(1, 3)),
            (Pos::new(3, 0), Velocity::new(2, -2)),
            (Pos::new(7, 6), Velocity::new(-1, -3)),
            (Pos::new(3, 0), Velocity::new(-1, -2)),
            (Pos::new(9, 3), Velocity::new(2, 3)),
            (Pos::new(7, 3), Velocity::new(-1, 3)),
            (Pos::new(2, 4), Velocity::new(2, -3)),
            (Pos::new(9, 5), Velocity::new(-3, -3)),
        ];
        // . 0 1 2 3 4 5 6 7 8 9 A
        // 0 1         x 2
        // 1           x
        // 2         1 x         2
        // 3 x x x x x x x x x x x
        // 4 3         x 4
        // 5           x
        // 6         3 x         4
        let width = 11;
        let height = 7;
        let qs = walk_robots(&robots, 100, width, height);
        assert_eq!(qs, [1, 3, 4, 1]);
    }
}
