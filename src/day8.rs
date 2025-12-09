use std::collections::HashMap;
use std::hash::Hash;

pub fn run() {
    let input = include_str!("../data/day8_real.txt");
    println!("Day 8 Part 1: {}", part1(input, 1000));
    // println!("Day 8 Part 2: {}", part2(input));
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}

fn part1(input: &str, iterations: usize) -> u64 {
    let positions = parse_input(input);
    let mut disjoint_set = DisjointSet::new(&positions);

    let mut edges = Vec::new();
    for (i, pos1) in positions.iter().enumerate() {
        for (j, pos2) in positions.iter().enumerate() {
            if i >= j {
                continue;
            }
            let dx = pos1.x - pos2.x;
            let dy = pos1.y - pos2.y;
            let dz = pos1.z - pos2.z;
            let dist = dx * dx + dy * dy + dz * dz;
            edges.push((dist, *pos1, *pos2));
        }
    }

    edges.sort_by_key(|e| e.0);

    for (_, p1, p2) in edges.iter().take(iterations) {
        disjoint_set.union(*p1, *p2);
    }

    let mut sizes = disjoint_set.sizes();
    sizes.sort_unstable_by_key(|s| *s);
    sizes.reverse();
    
    if sizes.len() < 3 {
        return 0;
    }

    sizes[0] as u64 * sizes[1] as u64 * sizes[2] as u64
}

struct DisjointSet {
    parent: HashMap<Position, Position>,
    rank: HashMap<Position, usize>,
}

impl DisjointSet {
    fn new(nodes: &[Position]) -> Self {
        let mut parent = HashMap::new();
        let mut rank = HashMap::new();
        for pos in nodes {
            parent.insert(*pos, *pos);
            rank.insert(*pos, 0);
        }
        Self { parent, rank }
    }

    fn find(&mut self, x: Position) -> Position {
        let p = *self.parent.get(&x).unwrap_or(&x);
        if p != x {
            let root = self.find(p);
            self.parent.insert(x, root);
            root
        } else {
            x
        }
    }

    fn union(&mut self, x: Position, y: Position) {
        let x_root = self.find(x);
        let y_root = self.find(y);
        if x_root == y_root {
            return;
        }
        let x_rank = *self.rank.get(&x_root).unwrap_or(&0);
        let y_rank = *self.rank.get(&y_root).unwrap_or(&0);

        if x_rank < y_rank {
            self.parent.insert(x_root, y_root);
        } else if x_rank > y_rank {
            self.parent.insert(y_root, x_root);
        } else {
            self.parent.insert(y_root, x_root);
            self.rank.insert(x_root, x_rank + 1);
        }
    }

    fn connected(&mut self, x: Position, y: Position) -> bool {
        self.find(x) == self.find(y)
    }

    fn sizes(&mut self) -> Vec<usize> {
        let mut sizes = HashMap::new();
        let positions: Vec<Position> = self.parent.keys().copied().collect();
        for pos in positions {
            let root = self.find(pos);
            *sizes.entry(root).or_insert(0) += 1;
        }
        sizes.values().cloned().collect()
    }
}


fn part2(input: &str) -> u64 {
    0
}

fn parse_input(input: &str) -> Vec<Position> {
    input.lines().map(|line| {
        let [x, y, z] = line.split(',').map(|s| s.parse().unwrap()).collect::<Vec<i64>>().try_into().unwrap();
        Position { x, y, z }
    }).collect()
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
}