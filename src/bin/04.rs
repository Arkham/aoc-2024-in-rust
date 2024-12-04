advent_of_code::solution!(4);

// Function to check if a word exists starting from (i, j) in a given direction
fn check_word(grid: &[Vec<char>], i: isize, j: isize, di: isize, dj: isize, word: &str) -> bool {
    let word_len = word.len();
    for k in 0..word_len {
        let ni = i + k as isize * di;
        let nj = j + k as isize * dj;
        if ni < 0 || nj < 0 || ni >= grid.len() as isize || nj >= grid[0].len() as isize {
            return false;
        }
        if grid[ni as usize][nj as usize] != word.chars().nth(k).unwrap() {
            return false;
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let word = "XMAS";
    let mut count = 0;

    // Iterate over each cell in the grid
    for (i, row) in grid.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            // Check all 8 possible directions for both "XMAS" and "SAMX"
            let directions = [
                (0, 1),   // Horizontal right
                (1, 0),   // Vertical down
                (1, 1),   // Diagonal down-right
                (1, -1),  // Diagonal down-left
                (0, -1),  // Horizontal left
                (-1, 0),  // Vertical up
                (-1, -1), // Diagonal up-left
                (-1, 1),  // Diagonal up-right
            ];

            for &(di, dj) in &directions {
                let i = i as isize;
                let j = j as isize;
                if check_word(&grid, i, j, di, dj, word) {
                    count += 1;
                }
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut count = 0;

    // Function to check if a specific pattern exists at a given starting point
    fn matches_pattern(grid: &[Vec<char>], i: usize, j: usize) -> bool {
        let patterns = [
            ('M', 'S', 'A', 'M', 'S'),
            ('S', 'M', 'A', 'S', 'M'),
            ('M', 'M', 'A', 'S', 'S'),
            ('S', 'S', 'A', 'M', 'M'),
        ];

        for &(top_left, top_right, center, bottom_left, bottom_right) in &patterns {
            if grid[i][j] == top_left
                && grid[i][j + 2] == top_right
                && grid[i + 1][j + 1] == center
                && grid[i + 2][j] == bottom_left
                && grid[i + 2][j + 2] == bottom_right
            {
                return true;
            }
        }
        false
    }

    // Iterate over each possible starting point for a 3x3 subgrid
    for i in 0..grid.len() - 2 {
        for j in 0..grid[i].len() - 2 {
            if matches_pattern(&grid, i, j) {
                count += 1;
            }
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
