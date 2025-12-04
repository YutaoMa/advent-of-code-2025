pub fn run() {
    let input = include_str!("../data/day4_real.txt");
    println!("Day 4 Part 1: {}", part1(input));
    println!("Day 4 Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let grid = parse_grid(input);
    candidates(&grid).count()
}

fn part2(input: &str) -> usize {
    let mut grid = parse_grid(input);
    let mut total = 0;

    loop {
        let to_remove: Vec<_> = candidates(&grid).collect();

        if to_remove.is_empty() {
            break;
        }

        total += to_remove.len();
        for (r, c) in to_remove {
            grid[r][c] = '.';
        }
    }

    total
}

/// Returns an iterator over coordinates (r, c) of '@' cells that have < 4 neighbors.
fn candidates(grid: &[Vec<char>]) -> impl Iterator<Item = (usize, usize)> + '_ {
    let rows = grid.len();
    let cols = grid.first().map_or(0, |row| row.len());

    (0..rows)
        .flat_map(move |r| (0..cols).map(move |c| (r, c)))
        .filter(move |&(r, c)| grid[r][c] == '@' && count_neighbors(grid, r, c) < 4)
}

fn count_neighbors(grid: &[Vec<char>], r: usize, c: usize) -> usize {
    let r = r as isize;
    let c = c as isize;
    const DIRS: [(isize, isize); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        ( 0, -1),          ( 0, 1),
        ( 1, -1), ( 1, 0), ( 1, 1),
    ];

    DIRS.iter()
        .filter_map(|&(dr, dc)| get(grid, r + dr, c + dc))
        .filter(|&cell| cell == '@')
        .count()
}

fn get(grid: &[Vec<char>], r: isize, c: isize) -> Option<char> {
    let rows = grid.len() as isize;
    let cols = grid.first()?.len() as isize;

    if r >= 0 && r < rows && c >= 0 && c < cols {
        Some(grid[r as usize][c as usize])
    } else {
        None
    }
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = include_str!("../data/day4_example.txt");
        assert_eq!(part1(input), 13);
    }

    #[test]
    fn test_part2_example() {
        let input = include_str!("../data/day4_example.txt");
        assert_eq!(part2(input), 43);
    }
}
