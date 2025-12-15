use std::collections::HashSet;

pub fn run() {
    let input = include_str!("../data/day12_real.txt");
    println!("Day 12 Part 1: {}", part1(input));
}

fn part1(input: &str) -> u64 {
    let (shapes, regions) = parse_input(input);

    let precomputed_orientations: Vec<Vec<Vec<(i32, i32)>>> =
        shapes.iter().map(|sh| orientations(&sh.cells)).collect();

    let total_regions = regions.len();
    let mut ok = 0usize;

    for (idx, r) in regions.iter().enumerate() {
        if can_pack_region(&shapes, &precomputed_orientations, r) {
            ok += 1;
        }

        if (idx + 1) % 100 == 0 || idx + 1 == total_regions {
            println!(
                "Processed {}/{} regions ({} packable so far)",
                idx + 1,
                total_regions,
                ok
            );
        }
    }
    ok as u64
}

#[derive(Clone, Debug)]
struct Shape {
    id: usize,
    cells: Vec<(i32, i32)>,
}

#[derive(Clone, Debug)]
struct Region {
    w: usize,
    h: usize,
    counts: Vec<usize>,
}

#[derive(Clone, Copy, Debug, Default)]
struct Node {
    left: usize,
    right: usize,
    up: usize,
    down: usize,
    col: usize,
}

struct DLX {
    nodes: Vec<Node>,
    col_size: Vec<usize>,
    col_primary: Vec<bool>,
    root: usize,
}

impl DLX {
    fn new(num_cols: usize, primary_cols: usize) -> Self {
        let mut nodes = vec![Node::default()];
        nodes.extend((1..=num_cols).map(|idx| Node {
            left: idx - 1,
            right: if idx == num_cols { 0 } else { idx + 1 },
            up: idx,
            down: idx,
            col: idx,
        }));

        if num_cols > 0 {
            nodes[0].right = 1;
            nodes[0].left = num_cols;
        }

        Self {
            nodes,
            col_size: vec![0; 1 + num_cols],
            col_primary: (0..=num_cols).map(|i| i > 0 && i <= primary_cols).collect(),
            root: 0,
        }
    }

    fn add_row(&mut self, cols: &[usize]) {
        if cols.is_empty() {
            return;
        }

        let row_nodes: Vec<_> = cols
            .iter()
            .map(|&col_hdr| {
                let node_idx = self.nodes.len();
                let up = self.nodes[col_hdr].up;

                self.nodes.push(Node {
                    left: node_idx,
                    right: node_idx,
                    up,
                    down: col_hdr,
                    col: col_hdr,
                });

                self.nodes[up].down = node_idx;
                self.nodes[col_hdr].up = node_idx;
                self.col_size[col_hdr] += 1;

                node_idx
            })
            .collect();

        let n = row_nodes.len();
        for (i, &a) in row_nodes.iter().enumerate() {
            self.nodes[a].right = row_nodes[(i + 1) % n];
            self.nodes[a].left = row_nodes[(i + n - 1) % n];
        }
    }

    fn cover(&mut self, col_hdr: usize) {
        let l = self.nodes[col_hdr].left;
        let r = self.nodes[col_hdr].right;
        self.nodes[l].right = r;
        self.nodes[r].left = l;

        let mut i = self.nodes[col_hdr].down;
        while i != col_hdr {
            let mut j = self.nodes[i].right;
            while j != i {
                let up = self.nodes[j].up;
                let down = self.nodes[j].down;
                self.nodes[up].down = down;
                self.nodes[down].up = up;
                self.col_size[self.nodes[j].col] -= 1;
                j = self.nodes[j].right;
            }
            i = self.nodes[i].down;
        }
    }

    fn uncover(&mut self, col_hdr: usize) {
        let mut i = self.nodes[col_hdr].up;
        while i != col_hdr {
            let mut j = self.nodes[i].left;
            while j != i {
                let up = self.nodes[j].up;
                let down = self.nodes[j].down;
                self.nodes[up].down = j;
                self.nodes[down].up = j;
                self.col_size[self.nodes[j].col] += 1;
                j = self.nodes[j].left;
            }
            i = self.nodes[i].up;
        }

        let l = self.nodes[col_hdr].left;
        let r = self.nodes[col_hdr].right;
        self.nodes[l].right = col_hdr;
        self.nodes[r].left = col_hdr;
    }

    fn column_iter(&self) -> impl Iterator<Item = usize> + '_ {
        std::iter::successors(Some(self.nodes[self.root].right), move |&c| {
            (c != self.root).then(|| self.nodes[c].right)
        })
        .take_while(|&c| c != self.root)
    }

    fn any_primary_left(&self) -> bool {
        self.column_iter().any(|c| self.col_primary[c])
    }

    fn choose_primary_column(&self) -> Option<usize> {
        self.column_iter()
            .filter(|&c| self.col_primary[c])
            .min_by_key(|&c| self.col_size[c])
    }

    fn solve_exists(&mut self) -> bool {
        self.search()
    }

    fn search(&mut self) -> bool {
        if !self.any_primary_left() {
            return true;
        }

        let c = match self.choose_primary_column() {
            Some(c) => c,
            None => return true,
        };

        if self.col_size[c] == 0 {
            return false;
        }

        self.cover(c);

        let mut r = self.nodes[c].down;
        while r != c {
            let mut j = self.nodes[r].right;
            while j != r {
                let col_j = self.nodes[j].col;
                self.cover(col_j);
                j = self.nodes[j].right;
            }

            if self.search() {
                return true;
            }

            let mut j2 = self.nodes[r].left;
            while j2 != r {
                let col_j = self.nodes[j2].col;
                self.uncover(col_j);
                j2 = self.nodes[j2].left;
            }

            r = self.nodes[r].down;
        }

        self.uncover(c);
        false
    }
}

fn is_shape_header(line: &str) -> Option<usize> {
    let num = line.trim().strip_suffix(':')?;
    (!num.contains('x') && num.chars().all(|c| c.is_ascii_digit())).then(|| num.parse().ok())?
}

fn is_region_line(line: &str) -> bool {
    line.trim()
        .split_once(':')
        .and_then(|(prefix, _)| prefix.split_once('x'))
        .is_some_and(|(a, b)| {
            !a.is_empty()
                && !b.is_empty()
                && a.chars().all(|c| c.is_ascii_digit())
                && b.chars().all(|c| c.is_ascii_digit())
        })
}

fn parse_input(input: &str) -> (Vec<Shape>, Vec<Region>) {
    let lines: Vec<_> = input.lines().collect();
    let mut idx = 0;
    let mut shapes_by_id: Vec<Option<Shape>> = Vec::new();

    while idx < lines.len() && !is_region_line(lines[idx]) {
        let line = lines[idx].trim();
        if line.is_empty() {
            idx += 1;
            continue;
        }

        if let Some(id) = is_shape_header(line) {
            idx += 1;
            let grid: Vec<_> = lines[idx..]
                .iter()
                .take_while(|l| {
                    let t = l.trim();
                    !t.is_empty() && is_shape_header(t).is_none() && !is_region_line(t)
                })
                .collect();
            idx += grid.len();

            let cells: Vec<_> = grid
                .iter()
                .enumerate()
                .flat_map(|(y, row)| {
                    row.chars()
                        .enumerate()
                        .filter(|&(_, ch)| ch == '#')
                        .map(move |(x, _)| (x as i32, y as i32))
                })
                .collect();

            if shapes_by_id.len() <= id {
                shapes_by_id.resize_with(id + 1, || None);
            }
            shapes_by_id[id] = Some(Shape { id, cells });
        } else {
            idx += 1;
        }

        while idx < lines.len() && lines[idx].trim().is_empty() {
            idx += 1;
        }
    }

    let shapes: Vec<_> = shapes_by_id.into_iter().flatten().collect();

    let regions: Vec<_> = lines[idx..]
        .iter()
        .filter(|l| is_region_line(l))
        .filter_map(|t| {
            let (prefix, rest) = t.trim().split_once(':')?;
            let (w, h) = prefix.split_once('x')?;
            Some(Region {
                w: w.parse().ok()?,
                h: h.parse().ok()?,
                counts: rest
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect(),
            })
        })
        .collect();

    (shapes, regions)
}

fn orientations(cells: &[(i32, i32)]) -> Vec<Vec<(i32, i32)>> {
    fn rot((x, y): (i32, i32), k: i32) -> (i32, i32) {
        match k & 3 {
            0 => (x, y),
            1 => (y, -x),
            2 => (-x, -y),
            _ => (-y, x),
        }
    }

    fn normalize(pts: &mut Vec<(i32, i32)>) {
        let (min_x, min_y) = pts.iter().fold((i32::MAX, i32::MAX), |(mx, my), &(x, y)| {
            (mx.min(x), my.min(y))
        });
        pts.iter_mut().for_each(|(x, y)| {
            *x -= min_x;
            *y -= min_y;
        });
        pts.sort_unstable();
    }

    let mut seen = HashSet::new();
    [false, true]
        .into_iter()
        .flat_map(|flip| (0..4).map(move |k| (flip, k)))
        .filter_map(|(flip, k)| {
            let mut pts: Vec<_> = cells
                .iter()
                .map(|&(x, y)| {
                    let p = if flip { (-x, y) } else { (x, y) };
                    rot(p, k)
                })
                .collect();
            normalize(&mut pts);
            seen.insert(pts.clone()).then_some(pts)
        })
        .collect()
}

fn can_pack_region(
    shapes: &[Shape],
    precomputed_orientations: &[Vec<Vec<(i32, i32)>>],
    region: &Region,
) -> bool {
    let (w, h) = (region.w, region.h);

    if region.counts.len() > shapes.len() {
        return false;
    }

    let mut counts = vec![0usize; shapes.len()];
    counts[..region.counts.len()].copy_from_slice(&region.counts);

    let total_area: usize = shapes.iter().map(|sh| counts[sh.id] * sh.cells.len()).sum();
    if total_area > w * h {
        return false;
    }

    let (copy_cols, primary_cols) = shapes.iter().fold(
        (vec![Vec::new(); shapes.len()], 0usize),
        |(mut cols, offset), sh| {
            let c = counts[sh.id];
            cols[sh.id] = (offset..offset + c).collect();
            (cols, offset + c)
        },
    );

    let num_cols = primary_cols + w * h;

    let mut dlx = DLX::new(num_cols, primary_cols);

    let hdr = |logical_col: usize| -> usize { 1 + logical_col };

    for sh in shapes {
        let sid = sh.id;
        let needed = counts[sid];
        if needed == 0 {
            continue;
        }

        let olist = &precomputed_orientations[sid];
        if olist.is_empty() {
            return false;
        }

        for &logical_piece_col in &copy_cols[sid] {
            let piece_hdr = hdr(logical_piece_col);
            let mut any_row_for_copy = false;

            for o in olist {
                let (max_x, max_y) = o.iter().fold((0, 0), |(mx, my), &(x, y)| {
                    (mx.max(x as usize), my.max(y as usize))
                });

                if max_x >= w || max_y >= h {
                    continue;
                }

                for ty in 0..=(h - max_y - 1) {
                    for tx in 0..=(w - max_x - 1) {
                        let mut cols = vec![piece_hdr];
                        cols.extend(o.iter().map(|&(dx, dy)| {
                            let (x, y) = (tx + dx as usize, ty + dy as usize);
                            hdr(primary_cols + y * w + x)
                        }));
                        dlx.add_row(&cols);
                        any_row_for_copy = true;
                    }
                }
            }

            if !any_row_for_copy {
                return false;
            }
        }
    }

    dlx.solve_exists()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../data/day12_example.txt");
        assert_eq!(part1(input), 2);
    }
}
