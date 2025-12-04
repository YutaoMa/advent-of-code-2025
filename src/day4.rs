pub fn run() {
    let input = include_str!("../data/day4_real.txt");
    println!("Day 4 Part 1: {}", part1(input));
    // println!("Day 4 Part 2: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    let grid = parse_grid(input);

    let mut total = 0u64;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '@' && count_rolls(&grid, i, j) < 4 {
                total += 1;
            }
        }
    }
    total
}

fn count_rolls(grid: &[Vec<char>], i: usize, j: usize) -> u64 {
    let dirs = [
        (-1, -1), (-1, 0), (-1, 1),
        ( 0, -1),          ( 0, 1),
        ( 1, -1), ( 1, 0), ( 1, 1),
    ];

    let rows = grid.len() as isize;
    let cols = if rows > 0 { grid[0].len() as isize } else { 0 };

    let mut count = 0u64;
    for (di, dj) in dirs.iter() {
        let ni = i as isize + di;
        let nj = j as isize + dj;
        if ni >= 0 && nj >= 0 && ni < rows && nj < cols {
            if grid[ni as usize][nj as usize] == '@' {
                count += 1;
            }
        }
    }
    count
}

fn part2(input: &str) -> u64 {
    0
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
}