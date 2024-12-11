advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u32> {
    let mut stones: Vec<u64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap_or(0))
        .collect();

    // Perform one blink
    for i in 0..75 {
        dbg!(i);
        stones = blink(&stones);
    }

    // For demonstration, let's return the number of stones after one blink
    Some(stones.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn blink(stones: &[u64]) -> Vec<u64> {
    let mut new_stones = Vec::new();
    for &stone in stones {
        if stone == 0 {
            new_stones.push(1);
        } else if stone.to_string().len() % 2 == 0 {
            let s = stone.to_string();
            let mid = s.len() / 2;
            let left = s[..mid].parse().unwrap_or(0);
            let right = s[mid..].parse().unwrap_or(0);
            new_stones.push(left);
            new_stones.push(right);
        } else {
            new_stones.push(stone * 2024);
        }
    }
    new_stones
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
        assert_eq!(result, None);
    }
}
