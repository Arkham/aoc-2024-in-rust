advent_of_code::solution!(2);

use nom::{
    character::complete::{digit1, space1},
    combinator::map_res,
    multi::separated_list1,
    IResult,
};

fn parse_line(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(space1, map_res(digit1, str::parse))(input)
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .filter_map(|line| {
            if let Ok((_, parsed_line)) = parse_line(line) {
                Some(parsed_line)
            } else {
                None
            }
        })
        .collect()
}

fn is_valid_line(line: &Vec<u32>) -> bool {
    let mut increasing = true;
    let mut decreasing = true;
    let mut valid = true;

    for window in line.windows(2) {
        let diff = (window[0] as i32 - window[1] as i32).abs();
        if diff < 1 || diff > 3 {
            valid = false;
            break;
        }
        if window[0] < window[1] {
            decreasing = false;
        } else if window[0] > window[1] {
            increasing = false;
        }
    }

    valid && (increasing || decreasing)
}

pub fn part_one(input: &str) -> Option<u32> {
    let report = parse_input(input);
    let mut safe_lines = Vec::new();

    for line in report.iter() {
        if is_valid_line(line) {
            safe_lines.push(line.clone());
        }
    }

    Some(safe_lines.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let report = parse_input(input);
    let mut safe_lines = Vec::new();

    for line in report.iter() {
        if is_valid_line(line) {
            safe_lines.push(line.clone());
        } else {
            for i in 0..line.len() {
                let mut modified_line = line.clone();
                modified_line.remove(i);
                if is_valid_line(&modified_line) {
                    safe_lines.push(line.clone());
                    break;
                }
            }
        }
    }

    Some(safe_lines.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
