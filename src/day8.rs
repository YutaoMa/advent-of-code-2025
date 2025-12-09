use std::collections::HashMap;

pub fn run() {
    let input = include_str!("../data/day8_real.txt");
    println!("Day 8 Part 1: {}", part1(input, 1000));
    println!("Day 8 Part 2: {}", part2(input));
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}

fn compute_edges(positions: &[Position]) -> Vec<(i64, Position, Position)> {
    let mut edges = Vec::with_capacity(positions.len() * (positions.len() - 1) / 2);
    for (i, &pos1) in positions.iter().enumerate() {
        for &pos2 in &positions[i + 1..] {
            let dx = pos1.x - pos2.x;
            let dy = pos1.y - pos2.y;
            let dz = pos1.z - pos2.z;
            let dist = dx * dx + dy * dy + dz * dz;
            edges.push((dist, pos1, pos2));
        }
    }
    edges.sort_unstable_by_key(|e| e.0);
    edges
}

fn part1(input: &str, iterations: usize) -> u64 {
    let positions = parse_input(input);
    let mut disjoint_set = DisjointSet::new(&positions);
    let edges = compute_edges(&positions);

    for (_, p1, p2) in edges.into_iter().take(iterations) {
        disjoint_set.union(p1, p2);
    }

    let mut sizes = disjoint_set.sizes();
    sizes.sort_unstable_by(|a, b| b.cmp(a));

    sizes
        .get(0..3)
        .map(|top3| top3.iter().map(|&s| s as u64).product())
        .unwrap_or(0)
}

fn part2(input: &str) -> u64 {
    let positions = parse_input(input);
    let mut disjoint_set = DisjointSet::new(&positions);
    let edges = compute_edges(&positions);

    let mut last_pair = (0, 0);
    let mut components = positions.len();

    for (_, pos1, pos2) in edges {
        if disjoint_set.union(pos1, pos2) {
            last_pair = (pos1.x, pos2.x);
            components -= 1;
            if components == 1 {
                break;
            }
        }
    }

    (last_pair.0.abs() as u64) * (last_pair.1.abs() as u64)
}

struct DisjointSet {
    parent: HashMap<Position, Position>,
    rank: HashMap<Position, usize>,
}

impl DisjointSet {
    fn new(nodes: &[Position]) -> Self {
        let parent = nodes.iter().map(|&p| (p, p)).collect();
        let rank = nodes.iter().map(|&p| (p, 0)).collect();
        Self { parent, rank }
    }

    fn find(&mut self, x: Position) -> Position {
        let p = self.parent[&x];
        if p != x {
            let root = self.find(p);
            self.parent.insert(x, root);
            root
        } else {
            x
        }
    }

    /// Returns true if union was performed (nodes were in different sets)
    fn union(&mut self, x: Position, y: Position) -> bool {
        let x_root = self.find(x);
        let y_root = self.find(y);
        if x_root == y_root {
            return false;
        }

        let x_rank = self.rank[&x_root];
        let y_rank = self.rank[&y_root];

        match x_rank.cmp(&y_rank) {
            std::cmp::Ordering::Less => {
                self.parent.insert(x_root, y_root);
            }
            std::cmp::Ordering::Greater => {
                self.parent.insert(y_root, x_root);
            }
            std::cmp::Ordering::Equal => {
                self.parent.insert(y_root, x_root);
                *self.rank.get_mut(&x_root).unwrap() += 1;
            }
        }
        true
    }

    fn sizes(&mut self) -> Vec<usize> {
        let positions: Vec<Position> = self.parent.keys().copied().collect();
        let mut sizes: HashMap<Position, usize> = HashMap::new();
        for pos in positions {
            let root = self.find(pos);
            *sizes.entry(root).or_insert(0) += 1;
        }
        sizes.into_values().collect()
    }
}

fn parse_input(input: &str) -> Vec<Position> {
    input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split(',').map(|s| s.parse::<i64>().ok());
            Some(Position {
                x: parts.next()??,
                y: parts.next()??,
                z: parts.next()??,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../data/day8_example.txt");
        assert_eq!(part1(input, 10), 40);
    }

    #[test]
    fn test_parse_input() {
        let input = include_str!("../data/day8_example.txt");
        let positions = parse_input(input);
        assert_eq!(positions.len(), 20);
        assert_eq!(positions[0].x, 162);
        assert_eq!(positions[0].y, 817);
        assert_eq!(positions[0].z, 812);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../data/day8_example.txt");
        assert_eq!(part2(input), 25272);
    }
}
