advent_of_code::solution!(11);

use memoize::memoize;

pub fn part_one(input: &str) -> Option<u64> {
    let stones: Vec<u64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap_or(0))
        .collect();

    let total = stones
        .iter()
        .map(|&stone| count_descendants(stone, 25))
        .sum();
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let stones: Vec<u64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap_or(0))
        .collect();

    let total = stones
        .iter()
        .map(|&stone| count_descendants(stone, 75))
        .sum();
    Some(total)
}

#[allow(unused)]
fn blink(stones: &[u64]) -> Vec<u64> {
    let mut new_stones = Vec::with_capacity(stones.len() * 2);
    for &stone in stones {
        if stone == 0 {
            new_stones.push(1);
        } else {
            let num_digits = count_digits(stone);
            if num_digits % 2 == 0 {
                let mid = num_digits / 2;
                let divisor = 10_u64.pow(mid as u32);
                let left = stone / divisor;
                let right = stone % divisor;
                new_stones.push(left);
                new_stones.push(right);
            } else {
                new_stones.push(stone * 2024);
            }
        }
    }
    new_stones
}

fn count_digits(num: u64) -> usize {
    if num == 0 {
        return 1;
    }
    (num as f64).log10().floor() as usize + 1
}

#[memoize]
fn count_descendants(stone: u64, blinks: u32) -> u64 {
    if blinks == 0 {
        return 1;
    }

    if stone == 0 {
        count_descendants(1, blinks - 1)
    } else {
        let num_digits = count_digits(stone);
        if num_digits % 2 == 0 {
            let mid = num_digits / 2;
            let divisor = 10_u64.pow(mid as u32);
            let left = stone / divisor;
            let right = stone % divisor;
            count_descendants(left, blinks - 1) + count_descendants(right, blinks - 1)
        } else {
            count_descendants(stone * 2024, blinks - 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
