advent_of_code::solution!(6);

use nom::{
    character::complete::{char, line_ending},
    combinator::map,
    multi::many1,
    sequence::terminated,
    IResult,
};
use rayon::prelude::*;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Clone)]
enum Cell {
    Empty,
    Obstruction,
    Guard,
    GuardVisited(Direction),
}

fn parse_cell(input: &str) -> IResult<&str, Cell> {
    let (input, cell) = nom::branch::alt((
        map(char('.'), |_| Cell::Empty),
        map(char('#'), |_| Cell::Obstruction),
        map(char('^'), |_| Cell::Guard),
    ))(input)?;
    Ok((input, cell))
}

fn parse_row(input: &str) -> IResult<&str, Vec<Cell>> {
    many1(parse_cell)(input)
}

fn parse_map(input: &str) -> IResult<&str, Vec<Vec<Cell>>> {
    many1(terminated(parse_row, line_ending))(input)
}

#[allow(dead_code)]
fn print_map(map: Vec<Vec<Cell>>) {
    for row in map {
        for cell in row {
            match cell {
                Cell::Empty => print!("."),
                Cell::Obstruction => print!("#"),
                Cell::Guard => print!("o"),
                Cell::GuardVisited(Up) => print!("^"),
                Cell::GuardVisited(Right) => print!(">"),
                Cell::GuardVisited(Down) => print!("v"),
                Cell::GuardVisited(Left) => print!("<"),
            }
        }
        println!();
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, mut map) = parse_map(input).unwrap();
    predict_guard_movement(&mut map);
    let mut visited_count = 0;
    for row in &map {
        for cell in row {
            if let Cell::GuardVisited(_) = cell {
                visited_count += 1;
            }
        }
    }
    Some(visited_count)
}
#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
use Direction::{Down, Left, Right, Up};

fn find_guard_position(map: &[Vec<Cell>]) -> Option<(usize, usize)> {
    for (i, row) in map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == Cell::Guard {
                return Some((i, j));
            }
        }
    }
    None
}

fn predict_guard_movement(map: &mut [Vec<Cell>]) -> bool {
    let directions = [
        (Up, (-1, 0)),
        (Right, (0, 1)),
        (Down, (1, 0)),
        (Left, (0, -1)),
    ];
    let mut direction_index = 0;
    let mut visited_positions = HashSet::new();

    if let Some((mut x, mut y)) = find_guard_position(map) {
        loop {
            let (dir, (dx, dy)) = directions[direction_index];
            let (new_x, new_y) = (x as isize + dx, y as isize + dy);

            // Mark the current cell as visited
            visited_positions.insert((x as isize, y as isize, dir));
            map[x][y] = Cell::GuardVisited(dir);

            // Check if the new position is out of bounds
            if new_x < 0
                || new_x >= map.len() as isize
                || new_y < 0
                || new_y >= map[0].len() as isize
            {
                return false;
            }

            // Check for loop detection
            if visited_positions.contains(&(new_x, new_y, dir)) {
                return true;
            }

            match map[new_x as usize][new_y as usize] {
                Cell::Obstruction => {
                    direction_index = (direction_index + 1) % 4; // Turn right 90 degrees
                }
                _ => {
                    x = new_x as usize;
                    y = new_y as usize;
                }
            }
        }
    } else {
        false
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, map) = parse_map(input).unwrap();
    let guard_position = find_guard_position(&map);

    // find all visited positions, excluding the guard's initial position
    let mut visited = map.clone();
    predict_guard_movement(&mut visited);
    let mut visited_positions: HashSet<(usize, usize)> = HashSet::new();
    for (i, row) in visited.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if let Cell::GuardVisited(_) = cell {
                visited_positions.insert((i, j));
            }
        }
    }
    if let Some((guard_x, guard_y)) = guard_position {
        visited_positions.remove(&(guard_x, guard_y));
    }

    // try putting an obstruction everywhere along the path and count loops
    let loop_count: u32 = visited_positions
        .par_iter()
        .filter_map(|&(i, j)| {
            if let Cell::Empty = map[i][j] {
                let mut new_map = map.clone();
                new_map[i][j] = Cell::Obstruction;

                if predict_guard_movement(&mut new_map) {
                    return Some(1);
                }
            }
            None
        })
        .sum();

    Some(loop_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
