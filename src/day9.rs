use std::collections::HashMap;

pub fn run() {
    let input = include_str!("../data/day9_real.txt");
    println!("Day 9 Part 1: {}", part1(input));
    println!("Day 9 Part 2: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    let coords = parse_coords(input);
    find_largest_area(&coords)
}

fn part2(input: &str) -> u64 {
    let coords = parse_coords(input);

    let mut xs: Vec<i64> = coords.iter().map(|c| c.0).collect();
    let mut ys: Vec<i64> = coords.iter().map(|c| c.1).collect();
    xs.sort_unstable();
    xs.dedup();
    ys.sort_unstable();
    ys.dedup();

    let x_to_idx: HashMap<i64, usize> = xs.iter().enumerate().map(|(i, &x)| (x, i + 1)).collect();
    let y_to_idx: HashMap<i64, usize> = ys.iter().enumerate().map(|(i, &y)| (y, i + 1)).collect();

    let n = xs.len() + 2;
    let m = ys.len() + 2;

    let mut grid = vec![vec![false; m]; n];

    for i in 0..coords.len() {
        let (x1, y1) = coords[i];
        let (x2, y2) = coords[(i + 1) % coords.len()];

        let ix1 = x_to_idx[&x1];
        let iy1 = y_to_idx[&y1];
        let ix2 = x_to_idx[&x2];
        let iy2 = y_to_idx[&y2];

        if ix1 == ix2 {
            for iy in iy1.min(iy2)..=iy1.max(iy2) {
                grid[ix1][iy] = true;
            }
        }
        if iy1 == iy2 {
            for ix in ix1.min(ix2)..=ix1.max(ix2) {
                grid[ix][iy1] = true;
            }
        }
    }

    let mut outside = vec![vec![false; m]; n];
    let mut stack = vec![(0usize, 0usize)];

    while let Some((x, y)) = stack.pop() {
        if x >= n || y >= m || outside[x][y] || grid[x][y] {
            continue;
        }
        outside[x][y] = true;

        if x > 0 {
            stack.push((x - 1, y));
        }
        if x + 1 < n {
            stack.push((x + 1, y));
        }
        if y > 0 {
            stack.push((x, y - 1));
        }
        if y + 1 < m {
            stack.push((x, y + 1));
        }
    }

    let mut enclosed = vec![vec![false; m]; n];
    for i in 0..n {
        for j in 0..m {
            enclosed[i][j] = !outside[i][j];
        }
    }

    let mut prefix = vec![vec![0i64; m + 1]; n + 1];
    for i in 0..n {
        for j in 0..m {
            prefix[i + 1][j + 1] =
                prefix[i][j + 1] + prefix[i + 1][j] - prefix[i][j] + enclosed[i][j] as i64;
        }
    }

    let mut max_area = 0i64;
    for coord in &coords {
        for other in &coords {
            if coord == other {
                continue;
            }

            let ix1 = x_to_idx[&coord.0];
            let iy1 = y_to_idx[&coord.1];
            let ix2 = x_to_idx[&other.0];
            let iy2 = y_to_idx[&other.1];

            if is_fully_enclosed(&prefix, ix1, iy1, ix2, iy2) {
                let area = ((coord.0 - other.0).abs() + 1) * ((coord.1 - other.1).abs() + 1);
                max_area = max_area.max(area);
            }
        }
    }

    max_area as u64
}

fn is_fully_enclosed(prefix: &[Vec<i64>], x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
    let (x1, x2) = (x1.min(x2), x1.max(x2));
    let (y1, y2) = (y1.min(y2), y1.max(y2));
    let sum = prefix[x2 + 1][y2 + 1] - prefix[x1][y2 + 1] - prefix[x2 + 1][y1] + prefix[x1][y1];
    let area = ((x2 - x1 + 1) * (y2 - y1 + 1)) as i64;
    sum == area
}

fn parse_coords(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

fn find_largest_area(coords: &[(i64, i64)]) -> u64 {
    let mut max_area = 0;
    for coord in coords {
        for other in coords {
            let area = ((coord.0 - other.0).abs() + 1) * ((coord.1 - other.1).abs() + 1);
            if area > max_area {
                max_area = area;
            }
        }
    }
    max_area as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../data/day9_example.txt");
        assert_eq!(part1(input), 50);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../data/day9_example.txt");
        assert_eq!(part2(input), 24);
    }
}
