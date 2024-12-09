advent_of_code::solution!(8);

use nom::{
    character::complete::{char, one_of},
    combinator::map,
    multi::many1,
    sequence::terminated,
    IResult,
};
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
enum Cell {
    Antenna(char),
    Empty,
}

fn parse_grid(input: &str) -> IResult<&str, Vec<Vec<Cell>>> {
    many1(terminated(
        many1(map(
            one_of("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789."),
            |c| {
                if c == '.' {
                    Cell::Empty
                } else {
                    Cell::Antenna(c)
                }
            },
        )),
        char('\n'),
    ))(input)
}

#[allow(dead_code)]
fn print_grid(grid: &[Vec<Cell>], antinodes: &HashSet<(usize, usize)>) {
    for (i, row) in grid.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if antinodes.contains(&(i, j)) {
                print!("#");
            } else {
                match cell {
                    Cell::Empty => print!("."),
                    Cell::Antenna(c) => print!("{}", c),
                }
            }
        }
        println!();
    }
}

fn calculate_antinodes(
    grid: &[Vec<Cell>],
    antenna_type: char,
    resonant: bool,
) -> Vec<(usize, usize)> {
    let mut antinodes = Vec::new();
    let rows = grid.len();
    let cols = grid[0].len();

    for (i, row) in grid.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if let Cell::Antenna(c) = cell {
                if resonant {
                    antinodes.push((i, j));
                }
                if *c == antenna_type {
                    for (k, other_row) in grid.iter().enumerate() {
                        for (l, other_cell) in other_row.iter().enumerate() {
                            if i != k || j != l {
                                if let Cell::Antenna(other_c) = other_cell {
                                    if *other_c == antenna_type {
                                        let mut antinode_row =
                                            i as isize + (i as isize - k as isize);
                                        let mut antinode_col =
                                            j as isize + (j as isize - l as isize);
                                        while antinode_row >= 0
                                            && antinode_col >= 0
                                            && antinode_row < rows as isize
                                            && antinode_col < cols as isize
                                        {
                                            antinodes.push((
                                                antinode_row as usize,
                                                antinode_col as usize,
                                            ));
                                            if !resonant {
                                                break;
                                            }
                                            antinode_row += i as isize - k as isize;
                                            antinode_col += j as isize - l as isize;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    antinodes
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, grid) = parse_grid(input).unwrap();

    let mut antinodes_set = HashSet::new();
    let antenna_types: HashSet<char> = grid
        .iter()
        .flat_map(|row| row.iter())
        .filter_map(|cell| {
            if let Cell::Antenna(c) = cell {
                Some(*c)
            } else {
                None
            }
        })
        .collect();

    for antenna_type in antenna_types {
        let antinodes = calculate_antinodes(&grid, antenna_type, false);
        for antinode in antinodes {
            antinodes_set.insert(antinode);
        }
    }

    Some(antinodes_set.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, grid) = parse_grid(input).unwrap();

    let mut antinodes_set = HashSet::new();
    let antenna_types: HashSet<char> = grid
        .iter()
        .flat_map(|row| row.iter())
        .filter_map(|cell| {
            if let Cell::Antenna(c) = cell {
                Some(*c)
            } else {
                None
            }
        })
        .collect();

    for antenna_type in antenna_types {
        let antinodes = calculate_antinodes(&grid, antenna_type, true);
        for antinode in antinodes {
            antinodes_set.insert(antinode);
        }
    }

    Some((antinodes_set.len()) as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
