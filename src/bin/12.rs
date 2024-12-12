advent_of_code::solution!(12);

type Grid = Vec<Vec<char>>;
type Pos = (isize, isize);

const DIRECTIONS: [Pos; 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

const CORNERS: [(Pos, Pos, Pos); 4] = [
    // (adjacent, adjacent, diagonal)
    // X -
    // | D
    ((0, 1), (1, 0), (1, 1)), // bottom right corner
    // - X
    // D |
    ((1, 0), (0, -1), (1, -1)), // bottom left corner
    // | D
    // X -
    ((0, -1), (-1, 0), (-1, -1)), // top right corner
    // D |
    // - X
    ((-1, 0), (0, 1), (-1, 1)), // top left corner
];

struct Puzzle {
    grid: Grid,
}

impl From<&str> for Puzzle {
    fn from(input: &str) -> Self {
        let grid: Grid = input.lines().map(|line| line.chars().collect()).collect();
        Self { grid }
    }
}

impl Puzzle {
    fn find_regions(&self) -> Vec<Vec<Pos>> {
        let rows = self.grid.len();
        let cols = self.grid[0].len();
        let mut visited = vec![vec![false; cols]; rows];
        let mut regions = Vec::new();

        for x in 0..rows {
            for y in 0..cols {
                if visited[x][y] {
                    continue;
                }

                let mut region = Vec::new();
                let mut stack = vec![(x, y)];
                visited[x][y] = true;

                while let Some((cx, cy)) = stack.pop() {
                    region.push((cx as isize, cy as isize));

                    for &(dx, dy) in &DIRECTIONS {
                        let nx = cx as isize + dx;
                        let ny = cy as isize + dy;

                        if nx >= 0 && nx < rows as isize && ny >= 0 && ny < cols as isize {
                            let nx = nx as usize;
                            let ny = ny as usize;

                            if !visited[nx][ny] && self.grid[nx][ny] == self.grid[cx][cy] {
                                visited[nx][ny] = true;
                                stack.push((nx, ny));
                            }
                        }
                    }
                }

                regions.push(region);
            }
        }

        regions
    }

    fn at(&self, x: isize, y: isize) -> Option<char> {
        self.grid.get(x as usize)?.get(y as usize).copied()
    }

    fn neighbor_count(&self, x: isize, y: isize) -> isize {
        let mut result = 4;
        for (dx, dy) in &DIRECTIONS {
            if self.at(x + dx, y + dy) == self.at(x, y) {
                result -= 1;
            }
        }
        result
    }

    fn corner_count(&self, x: isize, y: isize) -> isize {
        let mut result = 0;
        for &((dx0, dy0), (dx1, dy1), (dx2, dy2)) in &CORNERS {
            if self.at(x + dx0, y + dy0) != self.at(x, y)
                && self.at(x + dx1, y + dy1) != self.at(x, y)
            {
                // Both adjacent cells are different:
                // X X
                // O O
                //
                // X in pos (0, 0) has two corners
                result += 1;
            } else if self.at(x + dx0, y + dy0) == self.at(x, y)
                && self.at(x + dx1, y + dy1) == self.at(x, y)
                && self.at(x + dx2, y + dy2) != self.at(x, y)
            {
                // Adjacent cells are the same but diagonal is different:
                // X X
                // X O
                //
                // X in pos (0, 0) has two corners
                result += 1;
            }
        }
        result
    }
}

pub fn part_one(input: &str) -> Option<isize> {
    let puzzle = Puzzle::from(input);
    let mut total = 0;
    for region in puzzle.find_regions() {
        let mut fences = 0;
        for (x, y) in &region {
            fences += puzzle.neighbor_count(*x, *y);
        }
        total += fences * region.len() as isize;
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<isize> {
    let puzzle = Puzzle::from(input);
    let mut total = 0;
    for region in puzzle.find_regions() {
        let mut fences = 0;
        for (x, y) in &region {
            fences += puzzle.corner_count(*x, *y);
        }
        total += fences * region.len() as isize;
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two_simple() {
        let result = part_two("AAAA\nBBCD\nBBCC\nEEEC");
        assert_eq!(result, Some(80));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
