advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut results = Vec::new();

    for cap in re.captures_iter(input) {
        let first: u32 = cap[1].parse().unwrap();
        let second: u32 = cap[2].parse().unwrap();
        results.push((first, second));
    }

    let total_sum: u32 = results.iter().map(|(a, b)| a * b).sum();
    Some(total_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut enabled = true;

    let mut results = Vec::new();
    for cap in re.captures_iter(input) {
        if &cap[0] == "do()" {
            enabled = true;
        } else if &cap[0] == "don't()" {
            enabled = false;
        } else if enabled {
            let first: u32 = cap[1].parse().unwrap();
            let second: u32 = cap[2].parse().unwrap();
            results.push((first, second));
        }
    }

    let total_sum: u32 = results.iter().map(|(a, b)| a * b).sum();
    Some(total_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(result, Some(48));
    }
}
