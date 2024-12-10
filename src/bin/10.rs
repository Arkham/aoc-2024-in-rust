advent_of_code::solution!(10);

use nom::{
    character::complete::{line_ending, one_of, space0},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::preceded,
    IResult,
};

use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum Cell {
    Height(u8),
    Empty,
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<Cell>>> {
    separated_list1(line_ending, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<Cell>> {
    preceded(space0, many1(parse_cell))(input)
}

fn parse_cell(input: &str) -> IResult<&str, Cell> {
    map(one_of("0123456789."), |c| match c {
        '.' => Cell::Empty,
        d => Cell::Height(d.to_digit(10).unwrap() as u8),
    })(input)
}

#[derive(Debug)]
struct Trail {
    path: Vec<(usize, usize)>,
}

fn find_hiking_trails(map: &Vec<Vec<Cell>>) -> Vec<Trail> {
    let mut trails = Vec::new();
    let rows = map.len();
    let cols = if rows > 0 { map[0].len() } else { 0 };

    for y in 0..rows {
        for x in 0..cols {
            if let Cell::Height(0) = map[y][x] {
                let mut path = Vec::new();
                let mut visited = HashSet::new();
                dfs(x, y, map, &mut path, &mut trails, &mut visited);
            }
        }
    }

    trails
}

fn dfs(
    x: usize,
    y: usize,
    map: &Vec<Vec<Cell>>,
    path: &mut Vec<(usize, usize)>,
    trails: &mut Vec<Trail>,
    visited: &mut HashSet<(usize, usize)>,
) {
    let rows = map.len();
    let cols = if rows > 0 { map[0].len() } else { 0 };

    let current_height = match map[y][x] {
        Cell::Height(h) => h,
        Cell::Empty => return,
    };

    path.push((x, y));
    visited.insert((x, y));

    if current_height == 9 {
        trails.push(Trail { path: path.clone() });
        path.pop();
        visited.remove(&(x, y));
        return;
    }

    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for &(dx, dy) in &directions {
        let new_x = x as isize + dx;
        let new_y = y as isize + dy;

        if new_x >= 0 && new_x < cols as isize && new_y >= 0 && new_y < rows as isize {
            let new_x = new_x as usize;
            let new_y = new_y as usize;
            if !visited.contains(&(new_x, new_y)) {
                if let Cell::Height(next_h) = map[new_y][new_x] {
                    if next_h == current_height + 1 {
                        dfs(new_x, new_y, map, path, trails, visited);
                    }
                }
            }
        }
    }

    path.pop();
    visited.remove(&(x, y));
}

#[allow(dead_code)]
fn print_map(map: &[Vec<Cell>]) {
    for line in map {
        for cell in line {
            match cell {
                Cell::Height(h) => print!("{}", h),
                Cell::Empty => print!("."),
            }
        }
        println!();
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, map) = parse_input(input.trim()).unwrap();
    let trails = find_hiking_trails(&map);

    let mut unique_trails = Vec::new();
    let mut seen = HashSet::new();
    for trail in trails {
        let start = trail.path.first().unwrap();
        let end = trail.path.last().unwrap();
        if seen.insert((*start, *end)) {
            unique_trails.push(trail);
        }
    }

    // for trail in &unique_trails {
    //     println!("{:?}", trail.path);
    // }
    // println!("Total hiking trails found: {}", unique_trails.len());
    Some(unique_trails.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, map) = parse_input(input.trim()).unwrap();
    let trails = find_hiking_trails(&map);
    // for trail in &trails {
    //     println!("{:?}", trail.path);
    // }
    // println!("Total hiking trails found: {}", trails.len());
    Some(trails.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let result = part_one(
            "
            ...0...
            ...1...
            ...2...
            6543456
            7.....7
            8.....8
            9.....9
            ",
        );
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_two() {
        let result = part_one(
            "
            ..90..9
            ...1.98
            ...2..7
            6543456
            765.987
            876....
            987....
            ",
        );
        assert_eq!(result, Some(4));
    }

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
