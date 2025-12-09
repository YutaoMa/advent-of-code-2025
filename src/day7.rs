pub fn run() {
    let input = include_str!("../data/day7_real.txt");
    println!("Day 7 Part 1: {}", part1(input));
    // println!("Day 7 Part 2: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    let grid = parse_grid(input);
    let mut beam_grid: Vec<Vec<bool>> = Vec::new();
    beam_grid.resize(grid.len(), vec![false; grid[0].len()]);
    let mut count = 0;
    for r in 1..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r - 1][c] == 'S' {
                beam_grid[r][c] = true;
            } else if beam_grid[r - 1][c] {
                if grid[r][c] == '^' {
                    count += 1;
                    if c > 0 {
                        beam_grid[r][c - 1] = true;
                    }
                    if c < grid[r].len() - 1 {
                        beam_grid[r][c + 1] = true;
                    }
                } else {
                    beam_grid[r][c] = true;
                }
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
    fn test_part1() {
        let input = include_str!("../data/day7_example.txt");
        assert_eq!(part1(input), 21);
    }
}