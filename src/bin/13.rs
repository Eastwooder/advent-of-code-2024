use glam::IVec2;
use regex::Regex;

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u32> {
    let config = parse_input(input);
    Some(calculate_price(config))
}

pub fn part_two(_: &str) -> Option<u32> {
    None
}

type Pos = IVec2;

#[derive(Copy, Clone, Debug)]
struct Configuration {
    a: Pos,
    b: Pos,
    price: Pos,
}

fn calculate_price(configs: Vec<Configuration>) -> u32 {
    configs
        .iter()
        .map(|config| calculate_config(config, 3, 1))
        .sum()
}

fn calculate_config(config: &Configuration, cost_a: u32, cost_b: u32) -> u32 {
    let perp_a_b = config.a.perp_dot(config.b);
    let perp_a_price = config.a.perp_dot(config.price);
    let perp_price_b = config.price.perp_dot(config.b);
    if perp_a_b == 0 || perp_a_price % perp_a_b != 0 || perp_price_b % perp_a_b != 0 {
        0
    } else {
        let num_press_a = (perp_price_b / perp_a_b) as u32;
        let num_press_b = (perp_a_price / perp_a_b) as u32;
        (num_press_a * cost_a) + (num_press_b * cost_b)
    }
}

fn parse_input(input: &str) -> Vec<Configuration> {
    let mut configs = vec![];
    let regex = Regex::new(r#"Button\s+A:\s+X\+(?<ax>\d+),\s+Y\+(?<ay>\d+)\nButton\s+B:\s+X\+(?<bx>\d+),\s+Y\+(?<by>\d+)\nPrize:\s+X=(?<px>\d+),\s+Y=(?<py>\d+)\n?"#).unwrap();
    for config in regex.captures_iter(input) {
        let ax = config.name("ax").unwrap().as_str().parse().unwrap();
        let ay = config.name("ay").unwrap().as_str().parse().unwrap();
        let bx = config.name("bx").unwrap().as_str().parse().unwrap();
        let by = config.name("by").unwrap().as_str().parse().unwrap();
        let px = config.name("px").unwrap().as_str().parse().unwrap();
        let py = config.name("py").unwrap().as_str().parse().unwrap();
        configs.push(Configuration {
            a: Pos::new(ax, ay),
            b: Pos::new(bx, by),
            price: Pos::new(px, py),
        })
    }
    configs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}