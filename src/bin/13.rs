advent_of_code::solution!(13);

use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy)]
struct Move {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Machine {
    button_a: Move,
    button_b: Move,
    prize: Point,
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    let (input, (_, x, _, y)) = tuple((
        tag("X"),
        preceded(tag("+"), complete::i64),
        tag(", Y"),
        preceded(tag("+"), complete::i64),
    ))(input)?;

    Ok((input, Move { x, y }))
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    let (input, (_, x, _, y)) =
        tuple((tag("X="), complete::i64, tag(", Y="), complete::i64))(input)?;

    Ok((input, Point { x, y }))
}

fn parse_machine(input: &str) -> IResult<&str, Machine> {
    let (input, (_, button_a, _, _, button_b, _, _, prize, _)) = tuple((
        tag("Button A: "),
        parse_move,
        line_ending,
        tag("Button B: "),
        parse_move,
        line_ending,
        tag("Prize: "),
        parse_point,
        line_ending,
    ))(input)?;

    Ok((
        input,
        Machine {
            button_a,
            button_b,
            prize,
        },
    ))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Machine>> {
    separated_list1(line_ending, parse_machine)(input)
}

fn solve_machine(machine: &Machine) -> Option<(u64, u64)> {
    // We have two equations:
    // a1 * x + b1 * y = c1
    // a2 * x + b2 * y = c2
    let a1 = machine.button_a.x;
    let b1 = machine.button_b.x;
    let c1 = machine.prize.x;

    let a2 = machine.button_a.y;
    let b2 = machine.button_b.y;
    let c2 = machine.prize.y;

    // The solution is given by Cramer's rule:
    // denominator = a1 * b2 - a2 * b1
    // x = (c1 * b2 - c2 * b1) / denominator
    // y = (a1 * c2 - a2 * c1) / denominator
    let denominator = a1 * b2 - a2 * b1;
    if denominator == 0 {
        return None;
    }

    let x_numerator = c1 * b2 - c2 * b1;
    let y_numerator = a1 * c2 - a2 * c1;
    if x_numerator % denominator != 0 || y_numerator % denominator != 0 {
        return None;
    }

    let x = x_numerator / denominator;
    let y = y_numerator / denominator;
    if x < 0 || y < 0 {
        return None;
    }

    Some((x as u64, y as u64))
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, machines) = parse_input(input).ok()?;
    Some(
        machines
            .iter()
            .filter_map(solve_machine)
            .map(|(a, b)| a * 3 + b)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, machines) = parse_input(input).ok()?;
    let updated_machines: Vec<Machine> = machines
        .iter()
        .map(|machine| Machine {
            button_a: machine.button_a,
            button_b: machine.button_b,
            prize: Point {
                x: machine.prize.x + 10000000000000,
                y: machine.prize.y + 10000000000000,
            },
        })
        .collect();

    Some(
        updated_machines
            .iter()
            .filter_map(solve_machine)
            .map(|(a, b)| a * 3 + b)
            .sum(),
    )
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
        assert_eq!(result, Some(875318608908));
    }
}
