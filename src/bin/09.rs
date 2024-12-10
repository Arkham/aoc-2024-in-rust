advent_of_code::solution!(9);

pub fn parse_disk_map(input: &str) -> Vec<Option<u64>> {
    let mut result = Vec::new();
    let mut file_id = 0;
    let mut chars = input.chars();

    while let Some(file_length_char) = chars.next() {
        let free_space_length = if let Some(free_space_length_char) = chars.next() {
            free_space_length_char.to_digit(10).unwrap_or(0)
        } else {
            0
        };

        let file_length = file_length_char.to_digit(10).unwrap_or(0);

        // Append file blocks
        result.extend(std::iter::repeat(Some(file_id)).take(file_length as usize));
        // Append free space blocks
        result.extend(std::iter::repeat(None).take(free_space_length as usize));

        file_id += 1;
    }

    result
}

pub fn compact_disk_map_naive(mut disk_map: Vec<Option<u64>>) -> Vec<Option<u64>> {
    let n = disk_map.len();

    let mut moved = true;
    let mut latest_insert_pos = 0;

    while moved {
        moved = false;
        for i in (0..n).rev() {
            if disk_map[i].is_some() {
                if let Some(j) = disk_map
                    .iter()
                    .skip(latest_insert_pos)
                    .position(|&c| c.is_none())
                    .map(|pos| pos + latest_insert_pos)
                {
                    if j < i {
                        disk_map[j] = disk_map[i];
                        disk_map[i] = None;
                        moved = true;
                        latest_insert_pos = j;
                        break;
                    }
                }
            }
        }
    }
    disk_map
}

pub fn compact_disk_map(mut disk_map: Vec<Option<u64>>) -> Vec<Option<u64>> {
    let n = disk_map.len();

    // Identify all files with their starting index and size
    let mut files: Vec<(u64, usize, usize)> = Vec::new();

    let mut i = 0;
    while i < n {
        if let Some(file_id) = disk_map[i] {
            let start = i;
            let mut size = 1;
            i += 1;
            while i < n && disk_map[i] == Some(file_id) {
                size += 1;
                i += 1;
            }
            files.push((file_id, start, size));
        } else {
            i += 1;
        }
    }

    // Sort files in decreasing order of file_id
    files.sort_by(|a, b| b.0.cmp(&a.0));

    for (file_id, start, size) in files {
        // Find the leftmost span of free space that can fit the file
        if let Some(target_pos) = find_leftmost_fit(&disk_map, size) {
            // Ensure we only move the file if the target position is to the left of its current position
            if target_pos < start {
                // Move the entire file to the target position
                for j in 0..size {
                    disk_map[target_pos + j] = Some(file_id);
                    disk_map[start + j] = None;
                }
            }
        }
    }

    disk_map
}

// Helper function to find the leftmost span of free space that can fit the file
fn find_leftmost_fit(disk_map: &[Option<u64>], size: usize) -> Option<usize> {
    disk_map
        .windows(size)
        .position(|window| window.iter().all(|&block| block.is_none()))
}

pub fn checksum(disk_map: &[Option<u64>]) -> u64 {
    disk_map
        .iter()
        .enumerate()
        .filter_map(|(pos, &c)| c.map(|id| pos as u64 * id))
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    let disk_map = parse_disk_map(input);
    let compacted_map = compact_disk_map_naive(disk_map);
    Some(checksum(&compacted_map))
}

pub fn part_two(input: &str) -> Option<u64> {
    let disk_map = parse_disk_map(input);
    let compacted_map = compact_disk_map(disk_map);
    Some(checksum(&compacted_map))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
