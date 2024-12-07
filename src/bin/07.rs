advent_of_code::solution!(7);

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use rayon::prelude::*;

fn parse_line(input: &str) -> IResult<&str, (u64, Vec<u64>)> {
    let (input, (test_value, numbers)) = separated_pair(
        map_res(digit1, str::parse),
        tag(": "),
        separated_list1(space1, map_res(digit1, str::parse)),
    )(input)?;
    Ok((input, (test_value, numbers)))
}

#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
enum Operation {
    Add,
    Mul,
    Cat,
}

use Operation::{Add, Cat, Mul};

fn evaluate_expression(numbers: &[u64], operators: &[Operation]) -> u64 {
    let mut result = numbers[0];
    for (i, &op) in operators.iter().enumerate() {
        match op {
            Add => result += numbers[i + 1],
            Mul => result *= numbers[i + 1],
            Cat => {
                // Use arithmetic to concatenate numbers
                let mut concat_result = result;
                let mut next_number = numbers[i + 1];
                while next_number > 0 {
                    concat_result *= 10;
                    next_number /= 10;
                }
                result = concat_result + numbers[i + 1];
            }
        }
    }
    result
}

fn find_matching_expression(
    test_value: u64,
    numbers: &[u64],
    operators: &[Operation],
) -> Option<Vec<Operation>> {
    let n = numbers.len();
    let mut current_operators = vec![operators[0]; n - 1];
    loop {
        if evaluate_expression(numbers, &current_operators) == test_value {
            return Some(current_operators);
        }
        // Generate the next combination of operators
        let mut i = 0;
        while i < current_operators.len() {
            let op_index = operators
                .iter()
                .position(|&op| op == current_operators[i])
                .unwrap();
            if op_index + 1 < operators.len() {
                current_operators[i] = operators[op_index + 1];
                break;
            } else {
                current_operators[i] = operators[0];
                i += 1;
            }
        }
        if i == current_operators.len() {
            break;
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u64> {
    let parsed_lines: Vec<(u64, Vec<u64>)> = input
        .lines()
        .map(|line| parse_line(line).unwrap().1)
        .collect();

    let result = parsed_lines
        .par_iter()
        .filter_map(|(test_value, numbers)| {
            if find_matching_expression(*test_value, numbers, &[Add, Mul]).is_some() {
                Some(test_value)
            } else {
                None
            }
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let parsed_lines: Vec<(u64, Vec<u64>)> = input
        .lines()
        .map(|line| parse_line(line).unwrap().1)
        .collect();

    let result = parsed_lines
        .par_iter()
        .filter_map(|(test_value, numbers)| {
            if find_matching_expression(*test_value, numbers, &[Add, Mul, Cat]).is_some() {
                Some(test_value)
            } else {
                None
            }
        })
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
