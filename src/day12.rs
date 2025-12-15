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

#[derive(Clone, Copy, Debug)]
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
        let mut nodes = Vec::with_capacity(1 + num_cols);
        nodes.push(Node {
            left: 0,
            right: 0,
            up: 0,
            down: 0,
            col: 0,
        });

        let col_size = vec![0usize; 1 + num_cols];
        let mut col_primary = vec![false; 1 + num_cols];

        for c in 0..num_cols {
            let idx = 1 + c;
            let primary = c < primary_cols;
            col_primary[idx] = primary;

            nodes.push(Node {
                left: if idx == 1 { 0 } else { idx - 1 },
                right: if idx == num_cols { 0 } else { idx + 1 },
                up: idx,
                down: idx,
                col: idx,
            });
        }
        if num_cols > 0 {
            nodes[0].right = 1;
            nodes[0].left = num_cols;
            nodes[1].left = 0;
            nodes[num_cols].right = 0;
        } else {
            nodes[0].left = 0;
            nodes[0].right = 0;
        }

        DLX {
            nodes,
            col_size,
            col_primary,
            root: 0,
        }
    }

    fn add_row(&mut self, cols: &[usize]) {
        if cols.is_empty() {
            return;
        }

        let mut row_nodes: Vec<usize> = Vec::with_capacity(cols.len());
        for &col_hdr in cols {
            let node_idx = self.nodes.len();
            let up = self.nodes[col_hdr].up;
            let down = col_hdr;

            self.nodes.push(Node {
                left: node_idx,
                right: node_idx,
                up,
                down,
                col: col_hdr,
            });

            self.nodes[up].down = node_idx;
            self.nodes[down].up = node_idx;

            self.col_size[col_hdr] += 1;

            row_nodes.push(node_idx);
        }

        let n = row_nodes.len();
        for i in 0..n {
            let a = row_nodes[i];
            let b = row_nodes[(i + 1) % n];
            let p = row_nodes[(i + n - 1) % n];
            self.nodes[a].right = b;
            self.nodes[a].left = p;
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

    fn any_primary_left(&self) -> bool {
        let mut c = self.nodes[self.root].right;
        while c != self.root {
            if self.col_primary[c] {
                return true;
            }
            c = self.nodes[c].right;
        }
        false
    }

    fn choose_primary_column(&self) -> Option<usize> {
        let mut best: Option<(usize, usize)> = None;
        let mut c = self.nodes[self.root].right;
        while c != self.root {
            if self.col_primary[c] {
                let sz = self.col_size[c];
                match best {
                    None => best = Some((sz, c)),
                    Some((best_sz, _)) if sz < best_sz => best = Some((sz, c)),
                    _ => {}
                }
            }
            c = self.nodes[c].right;
        }
        best.map(|(_, c)| c)
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
    let t = line.trim();
    if t.ends_with(':') && !t.contains('x') {
        let num = &t[..t.len() - 1];
        if !num.is_empty() && num.chars().all(|ch| ch.is_ascii_digit()) {
            return Some(num.parse().ok()?);
        }
    }
    None
}

fn is_region_line(line: &str) -> bool {
    let t = line.trim();
    if let Some(colon) = t.find(':') {
        let prefix = &t[..colon];
        if let Some(xpos) = prefix.find('x') {
            let (a, b) = (&prefix[..xpos], &prefix[xpos + 1..]);
            return !a.is_empty()
                && !b.is_empty()
                && a.chars().all(|c| c.is_ascii_digit())
                && b.chars().all(|c| c.is_ascii_digit());
        }
    }
    false
}

fn parse_input(input: &str) -> (Vec<Shape>, Vec<Region>) {
    let lines: Vec<&str> = input.lines().collect();
    let mut i = 0usize;

    let mut shapes_by_id: Vec<Option<Shape>> = Vec::new();

    while i < lines.len() {
        let line = lines[i].trim_end();
        if line.trim().is_empty() {
            i += 1;
            continue;
        }
        if is_region_line(line) {
            break;
        }
        if let Some(id) = is_shape_header(line) {
            i += 1;
            let mut grid: Vec<String> = Vec::new();
            while i < lines.len() {
                let l = lines[i].trim_end();
                if l.trim().is_empty() {
                    break;
                }
                if is_shape_header(l).is_some() || is_region_line(l) {
                    break;
                }
                grid.push(l.to_string());
                i += 1;
            }

            let mut cells = Vec::new();
            for (y, row) in grid.iter().enumerate() {
                for (x, ch) in row.chars().enumerate() {
                    if ch == '#' {
                        cells.push((x as i32, y as i32));
                    }
                }
            }

            if shapes_by_id.len() <= id {
                shapes_by_id.resize_with(id + 1, || None);
            }
            shapes_by_id[id] = Some(Shape { id, cells });
        } else {
            i += 1;
        }
        while i < lines.len() && lines[i].trim().is_empty() {
            i += 1;
        }
    }

    let shapes: Vec<Shape> = shapes_by_id.into_iter().filter_map(|x| x).collect();

    let mut regions = Vec::new();
    while i < lines.len() {
        let t = lines[i].trim();
        i += 1;
        if t.is_empty() {
            continue;
        }
        if !is_region_line(t) {
            continue;
        }
        let colon = t.find(':').unwrap();
        let prefix = &t[..colon];
        let rest = t[colon + 1..].trim();
        let xpos = prefix.find('x').unwrap();
        let w: usize = prefix[..xpos].parse().unwrap();
        let h: usize = prefix[xpos + 1..].parse().unwrap();
        let counts: Vec<usize> = if rest.is_empty() {
            Vec::new()
        } else {
            rest.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        };
        regions.push(Region { w, h, counts });
    }

    (shapes, regions)
}

fn orientations(cells: &[(i32, i32)]) -> Vec<Vec<(i32, i32)>> {
    fn rot(p: (i32, i32), k: i32) -> (i32, i32) {
        let (x, y) = p;
        match k.rem_euclid(4) {
            0 => (x, y),
            1 => (y, -x),
            2 => (-x, -y),
            3 => (-y, x),
            _ => unreachable!(),
        }
    }
    fn reflect(p: (i32, i32)) -> (i32, i32) {
        (-p.0, p.1)
    }
    fn normalize(mut pts: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
        let min_x = pts.iter().map(|p| p.0).min().unwrap_or(0);
        let min_y = pts.iter().map(|p| p.1).min().unwrap_or(0);
        for p in &mut pts {
            p.0 -= min_x;
            p.1 -= min_y;
        }
        pts.sort_unstable();
        pts
    }

    let mut seen = HashSet::<Vec<(i32, i32)>>::new();
    let mut outs = Vec::new();

    for &flip in &[false, true] {
        for k in 0..4 {
            let mut pts: Vec<(i32, i32)> = cells
                .iter()
                .copied()
                .map(|p| {
                    let p = if flip { reflect(p) } else { p };
                    rot(p, k)
                })
                .collect();
            pts = normalize(pts);

            if seen.insert(pts.clone()) {
                outs.push(pts);
            }
        }
    }
    outs
}

fn area(cells: &[(i32, i32)]) -> usize {
    cells.len()
}

fn can_pack_region(
    shapes: &[Shape],
    precomputed_orientations: &[Vec<Vec<(i32, i32)>>],
    region: &Region,
) -> bool {
    let w = region.w;
    let h = region.h;

    let mut counts = vec![0usize; shapes.len()];
    for (i, &c) in region.counts.iter().enumerate() {
        if i < counts.len() {
            counts[i] = c;
        } else {
            return false;
        }
    }

    let mut total_area = 0usize;
    for sh in shapes {
        total_area += counts[sh.id] * area(&sh.cells);
    }
    if total_area > w * h {
        return false;
    }

    let mut copy_cols: Vec<Vec<usize>> = vec![Vec::new(); shapes.len()];
    let mut primary_cols = 0usize;
    for sh in shapes {
        let c = counts[sh.id];
        copy_cols[sh.id] = (0..c).map(|k| primary_cols + k).collect();
        primary_cols += c;
    }

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

        for &logical_piece_col in copy_cols[sid].iter() {
            let piece_hdr = hdr(logical_piece_col);

            let mut any_row_for_copy = false;

            for o in olist {
                let max_x = o.iter().map(|p| p.0).max().unwrap_or(0) as usize;
                let max_y = o.iter().map(|p| p.1).max().unwrap_or(0) as usize;

                if max_x >= w || max_y >= h {
                    continue;
                }

                let tx_max = w - (max_x + 1);
                let ty_max = h - (max_y + 1);

                for ty in 0..=ty_max {
                    for tx in 0..=tx_max {
                        let mut cols: Vec<usize> = Vec::with_capacity(1 + o.len());
                        cols.push(piece_hdr);

                        for &(dx, dy) in o {
                            let x = (tx as i32 + dx) as usize;
                            let y = (ty as i32 + dy) as usize;
                            let logical_cell_col = primary_cols + (y * w + x);
                            cols.push(hdr(logical_cell_col));
                        }

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
