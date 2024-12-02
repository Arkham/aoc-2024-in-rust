advent_of_code::solution!(1);

use nom::{
    bytes::complete::take_while, character::complete::space1, combinator::map_res,
    sequence::separated_pair, IResult,
};
use std::collections::HashMap;

fn parse_line(input: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(
        map_res(take_while(|c: char| c.is_ascii_digit()), str::parse),
        space1,
        map_res(take_while(|c: char| c.is_ascii_digit()), str::parse),
    )(input)
}

fn extract_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in input.lines() {
        if let Ok((_, (left_num, right_num))) = parse_line(line) {
            left_list.push(left_num);
            right_list.push(right_num);
        }
    }

    (left_list, right_list)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left_list, mut right_list) = extract_lists(input);

    left_list.sort();
    right_list.sort();

    let total_distance: u32 = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(left, right)| {
            if left > right {
                left - right
            } else {
                right - left
            }
        })
        .sum();

    Some(total_distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left_list, right_list) = extract_lists(input);

    let mut right_list_map = HashMap::new();
    for num in right_list.iter() {
        *right_list_map.entry(num).or_insert(0) += 1;
    }

    let mut count = 0;
    for left_num in left_list.iter() {
        if right_list_map.contains_key(left_num) {
            count += right_list_map[left_num] * left_num;
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
