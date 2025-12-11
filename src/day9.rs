use std::collections::HashMap;

pub fn run() {
    let input = include_str!("../data/day9_real.txt");
    println!("Day 9 Part 1: {}", part1(input));
    println!("Day 9 Part 2: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    let coords = parse_coords(input);
    max_rectangle_area(&coords, |_, _| true)
}

fn part2(input: &str) -> u64 {
    let coords = parse_coords(input);
    let enclosed = build_enclosed_grid(&coords);
    max_rectangle_area(&coords, |c1, c2| enclosed.is_filled(c1, c2))
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

fn max_rectangle_area(
    coords: &[(i64, i64)],
    is_valid: impl Fn((i64, i64), (i64, i64)) -> bool,
) -> u64 {
    coords
        .iter()
        .flat_map(|&a| coords.iter().map(move |&b| (a, b)))
        .filter(|&(a, b)| a != b && is_valid(a, b))
        .map(|(a, b)| ((a.0 - b.0).abs() + 1) * ((a.1 - b.1).abs() + 1))
        .max()
        .unwrap_or(0) as u64
}

struct EnclosedGrid {
    prefix: Vec<Vec<i64>>,
    x_to_idx: HashMap<i64, usize>,
    y_to_idx: HashMap<i64, usize>,
}

impl EnclosedGrid {
    fn is_filled(&self, c1: (i64, i64), c2: (i64, i64)) -> bool {
        let (x1, x2) = (self.x_to_idx[&c1.0], self.x_to_idx[&c2.0]);
        let (y1, y2) = (self.y_to_idx[&c1.1], self.y_to_idx[&c2.1]);
        let (x1, x2) = (x1.min(x2), x1.max(x2));
        let (y1, y2) = (y1.min(y2), y1.max(y2));

        let sum = self.prefix[x2 + 1][y2 + 1] - self.prefix[x1][y2 + 1] - self.prefix[x2 + 1][y1]
            + self.prefix[x1][y1];
        let area = ((x2 - x1 + 1) * (y2 - y1 + 1)) as i64;
        sum == area
    }
}

fn build_enclosed_grid(coords: &[(i64, i64)]) -> EnclosedGrid {
    let (xs, ys) = compress_coordinates(coords);
    let x_to_idx: HashMap<i64, usize> = xs.iter().enumerate().map(|(i, &x)| (x, i + 1)).collect();
    let y_to_idx: HashMap<i64, usize> = ys.iter().enumerate().map(|(i, &y)| (y, i + 1)).collect();

    let (n, m) = (xs.len() + 2, ys.len() + 2);
    let mut perimeter = vec![vec![false; m]; n];

    for (&(x1, y1), &(x2, y2)) in coords.iter().zip(coords.iter().cycle().skip(1)) {
        let (ix1, iy1) = (x_to_idx[&x1], y_to_idx[&y1]);
        let (ix2, iy2) = (x_to_idx[&x2], y_to_idx[&y2]);

        for ix in ix1.min(ix2)..=ix1.max(ix2) {
            for iy in iy1.min(iy2)..=iy1.max(iy2) {
                perimeter[ix][iy] = true;
            }
        }
    }

    let outside = flood_fill_outside(&perimeter, n, m);

    let mut prefix = vec![vec![0i64; m + 1]; n + 1];
    for i in 0..n {
        for j in 0..m {
            let enclosed = !outside[i][j];
            prefix[i + 1][j + 1] =
                prefix[i][j + 1] + prefix[i + 1][j] - prefix[i][j] + enclosed as i64;
        }
    }

    EnclosedGrid {
        prefix,
        x_to_idx,
        y_to_idx,
    }
}

fn compress_coordinates(coords: &[(i64, i64)]) -> (Vec<i64>, Vec<i64>) {
    let mut xs: Vec<i64> = coords.iter().map(|c| c.0).collect();
    let mut ys: Vec<i64> = coords.iter().map(|c| c.1).collect();
    xs.sort_unstable();
    xs.dedup();
    ys.sort_unstable();
    ys.dedup();
    (xs, ys)
}

fn flood_fill_outside(perimeter: &[Vec<bool>], n: usize, m: usize) -> Vec<Vec<bool>> {
    let mut outside = vec![vec![false; m]; n];
    let mut stack = vec![(0, 0)];

    while let Some((x, y)) = stack.pop() {
        if outside[x][y] || perimeter[x][y] {
            continue;
        }
        outside[x][y] = true;

        if x > 0 {
            stack.push((x - 1, y));
        }
        if y > 0 {
            stack.push((x, y - 1));
        }
        if x + 1 < n {
            stack.push((x + 1, y));
        }
        if y + 1 < m {
            stack.push((x, y + 1));
        }
    }

    outside
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
