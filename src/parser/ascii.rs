#![allow(clippy::needless_range_loop)]
use std::collections::HashSet;

const CHARW: f64 = 9.0;
const CHARH: f64 = 18.0;

#[derive(Clone, Copy)]
struct Box_ {
    r0: usize,
    c0: usize,
    r1: usize,
    c1: usize,
}

#[derive(Clone, Copy)]
struct Line {
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
    arrow: bool,
}

#[derive(Clone)]
struct Label {
    x: f64,
    y: f64,
    text: String,
}

fn inside_any_box(boxes: &[Box_], r: usize, c: usize) -> bool {
    boxes
        .iter()
        .any(|b| r > b.r0 && r < b.r1 && c > b.c0 && c < b.c1)
}

fn cell_x(c: usize) -> f64 {
    c as f64 * CHARW
}

fn cell_y(r: usize) -> f64 {
    r as f64 * CHARH
}

fn normalize(input: &str) -> Vec<Vec<char>> {
    let lines: Vec<&str> = input.split('\n').collect();
    let width = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        let mut row: Vec<char> = chars.clone();
        row.resize(width, ' ');
        grid.push(row);
    }
    // Map unicode box-drawing chars to ASCII equivalents
    for row in grid.iter_mut() {
        for ch in row.iter_mut() {
            *ch = match *ch {
                '─' | '━' | '-' => '-',
                '│' | '┃' | '|' => '|',
                '┌' | '┐' | '└' | '┘' | '├' | '┤' | '┬' | '┴' | '┼' | '╞' | '╡' | '╥' | '╨'
                | '╫' | '+' => '+',
                '→' | '➜' => '>',
                '←' | '➤' => '<',
                '↑' => '^',
                '↓' => 'v',
                _ => *ch,
            };
        }
    }
    grid
}

fn is_box_edge_row(grid: &[Vec<char>], r: usize, c0: usize, c1: usize) -> bool {
    if grid[r][c0] != '+' || grid[r][c1] != '+' {
        return false;
    }
    for c in (c0 + 1)..c1 {
        if grid[r][c] != '-' {
            return false;
        }
    }
    true
}

fn is_box_side_col(grid: &[Vec<char>], c: usize, r0: usize, r1: usize) -> bool {
    grid[r0][c] == '+'
        && grid[r1][c] == '+'
        && (r1 - r0) >= 1
        && (r0 + 1..r1).all(|r| grid[r][c] == '|')
}

fn detect_boxes(grid: &[Vec<char>]) -> Vec<Box_> {
    let rows = grid.len();
    if rows == 0 {
        return Vec::new();
    }
    let cols = grid[0].len();
    let mut boxes: Vec<Box_> = Vec::new();
    let mut taken: HashSet<(usize, usize, usize, usize)> = HashSet::new();
    for r0 in 0..rows {
        for c0 in 0..cols {
            if grid[r0][c0] != '+' {
                continue;
            }
            // find top-right corner
            let mut c1 = c0 + 1;
            while c1 < cols && (grid[r0][c1] == '-' || grid[r0][c1] == '+') {
                c1 += 1;
            }
            c1 -= 1;
            if c1 <= c0 || grid[r0][c1] != '+' {
                continue;
            }
            // top edge valid
            if !is_box_edge_row(grid, r0, c0, c1) {
                continue;
            }
            // find bottom edge by descending
            let mut r1 = r0 + 1;
            let mut found = None;
            while r1 < rows {
                if grid[r1][c0] == '+' && grid[r1][c1] == '+' && is_box_edge_row(grid, r1, c0, c1) {
                    found = Some(r1);
                    break;
                }
                if grid[r1][c0] != '|' || grid[r1][c1] != '|' {
                    break;
                }
                r1 += 1;
            }
            let r1 = match found {
                Some(r) => r,
                None => continue,
            };
            if !is_box_side_col(grid, c0, r0, r1) || !is_box_side_col(grid, c1, r0, r1) {
                continue;
            }
            if r1 - r0 < 1 || c1 - c0 < 1 {
                continue;
            }
            if taken.insert((r0, c0, r1, c1)) {
                boxes.push(Box_ { r0, c0, r1, c1 });
            }
        }
    }
    boxes
}

fn inside_text(grid: &[Vec<char>], b: &Box_) -> String {
    let mut parts: Vec<String> = Vec::new();
    for r in (b.r0 + 1)..b.r1 {
        let mut s = String::new();
        for c in (b.c0 + 1)..b.c1 {
            let ch = grid[r][c];
            if ch == '|' || ch == '-' || ch == '+' || ch == '=' {
                continue;
            }
            s.push(ch);
        }
        let t = s.trim();
        if !t.is_empty() {
            parts.push(t.to_string());
        }
    }
    parts.join("\n")
}

pub fn ascii_to_svg(input: &str) -> String {
    let grid = normalize(input);
    let boxes = detect_boxes(&grid);

    let rows = grid.len();
    let cols = if rows == 0 { 0 } else { grid[0].len() };
    let width = ((cols as f64) * CHARW).max(50.0);
    let height = ((rows as f64) * CHARH).max(50.0);

    let mut lines: Vec<Line> = Vec::new();
    let mut labels: Vec<Label> = Vec::new();

    // Determine occupied box-border cells
    let mut border: HashSet<(usize, usize)> = HashSet::new();
    for b in &boxes {
        for c in b.c0..=b.c1 {
            border.insert((b.r0, c));
            border.insert((b.r1, c));
        }
        for r in b.r0..=b.r1 {
            border.insert((r, b.c0));
            border.insert((r, b.c1));
        }
    }

    // Connectors: horizontal runs of '-' or '->'
    for r in 0..rows {
        let mut c = 0;
        while c < cols {
            if border.contains(&(r, c)) || inside_any_box(&boxes, r, c) {
                c += 1;
                continue;
            }
            let ch = grid[r][c];
            if ch == '-' || ch == '>' || ch == '<' {
                // find run end
                let start = c;
                let mut arrow_at_end = false;
                let mut c2 = c;
                while c2 < cols {
                    let g = grid[r][c2];
                    if border.contains(&(r, c2)) || inside_any_box(&boxes, r, c2) {
                        break;
                    }
                    if g == '-' {
                        c2 += 1;
                    } else if g == '>' {
                        arrow_at_end = true;
                        c2 += 1;
                        break;
                    } else {
                        break;
                    }
                }
                let end = c2 - 1;
                if end >= start {
                    let x0 = cell_x(start);
                    let x1 = cell_x(end) + CHARW;
                    let y = cell_y(r) + CHARH / 2.0;
                    lines.push(Line {
                        x0,
                        y0: y,
                        x1,
                        y1: y,
                        arrow: arrow_at_end,
                    });
                }
                c = c2;
            } else {
                c += 1;
            }
        }
    }

    // Connectors: vertical runs of '|'
    for c in 0..cols {
        let mut r = 0;
        while r < rows {
            if border.contains(&(r, c)) || inside_any_box(&boxes, r, c) {
                r += 1;
                continue;
            }
            let ch = grid[r][c];
            if ch == '|' || ch == 'v' || ch == '^' {
                let start = r;
                let mut arrow_at_end = false;
                let mut r2 = r;
                while r2 < rows {
                    let g = grid[r2][c];
                    if border.contains(&(r2, c)) || inside_any_box(&boxes, r2, c) {
                        break;
                    }
                    if g == '|' {
                        r2 += 1;
                    } else if g == 'v' {
                        arrow_at_end = true;
                        r2 += 1;
                        break;
                    } else {
                        break;
                    }
                }
                let end = r2 - 1;
                if end >= start {
                    let x = cell_x(c) + CHARW / 2.0;
                    let y0 = cell_y(start);
                    let y1 = cell_y(end) + CHARH;
                    lines.push(Line {
                        x0: x,
                        y0,
                        x1: x,
                        y1,
                        arrow: arrow_at_end,
                    });
                }
                r = r2;
            } else {
                r += 1;
            }
        }
    }

    // Connectors: diagonal runs of '/' and '\'
    let mut diag_used: HashSet<(usize, usize)> = HashSet::new();
    for r in 0..rows {
        for c in 0..cols {
            if border.contains(&(r, c))
                || inside_any_box(&boxes, r, c)
                || diag_used.contains(&(r, c))
            {
                continue;
            }
            let ch = grid[r][c];
            if ch != '/' && ch != '\\' {
                continue;
            }
            let (step_r, step_c): (isize, isize) = if ch == '/' {
                (1, -1) // '/' continues to larger r, smaller c
            } else {
                (1, 1) // '\\' continues to larger r, larger c
            };
            // Walk backward from (r,c) to find the start of the chain.
            let mut cells: Vec<(usize, usize)> = Vec::new();
            let mut rr = r as isize;
            let mut cc = c as isize;
            loop {
                rr -= step_r;
                cc -= step_c;
                if rr < 0 || cc < 0 || rr >= rows as isize || cc >= cols as isize {
                    break;
                }
                let ur = rr as usize;
                let uc = cc as usize;
                if border.contains(&(ur, uc)) || grid[ur][uc] != ch {
                    break;
                }
                cells.push((ur, uc));
            }
            cells.reverse();
            cells.push((r, c));
            // Walk forward to find the end of the chain.
            let mut rr = r as isize;
            let mut cc = c as isize;
            loop {
                rr += step_r;
                cc += step_c;
                if rr < 0 || cc < 0 || rr >= rows as isize || cc >= cols as isize {
                    break;
                }
                let ur = rr as usize;
                let uc = cc as usize;
                if border.contains(&(ur, uc)) || grid[ur][uc] != ch {
                    break;
                }
                cells.push((ur, uc));
            }
            for &(ur, uc) in &cells {
                diag_used.insert((ur, uc));
            }
            let (r0, c0) = cells[0];
            let (r1, c1) = *cells.last().unwrap();
            if ch == '/' {
                let x0 = cell_x(c0);
                let y0 = cell_y(r0) + CHARH;
                let x1 = cell_x(c1) + CHARW;
                let y1 = cell_y(r1);
                lines.push(Line {
                    x0,
                    y0,
                    x1,
                    y1,
                    arrow: false,
                });
            } else {
                let x0 = cell_x(c0);
                let y0 = cell_y(r0);
                let x1 = cell_x(c1) + CHARW;
                let y1 = cell_y(r1) + CHARH;
                lines.push(Line {
                    x0,
                    y0,
                    x1,
                    y1,
                    arrow: false,
                });
            }
        }
    }

    // Labels inside boxes
    for b in &boxes {
        let text = inside_text(&grid, b);
        if !text.is_empty() {
            let cx = cell_x(b.c0) + ((b.c1 - b.c0 + 1) as f64) * CHARW / 2.0;
            let cy = cell_y(b.r0) + ((b.r1 - b.r0 + 1) as f64) * CHARH / 2.0;
            labels.push(Label { x: cx, y: cy, text });
        }
    }

    // Free-standing labels (text not in boxes, not on connectors)
    for r in 0..rows {
        let mut c = 0;
        while c < cols {
            if border.contains(&(r, c)) || inside_any_box(&boxes, r, c) {
                c += 1;
                continue;
            }
            let ch = grid[r][c];
            if ch == ' '
                || ch == '-'
                || ch == '|'
                || ch == '+'
                || ch == '>'
                || ch == '<'
                || ch == '/'
                || ch == '\\'
            {
                c += 1;
                continue;
            }
            // collect text run
            let start = c;
            let mut end = c;
            while end < cols {
                let g = grid[r][end];
                if border.contains(&(r, end))
                    || inside_any_box(&boxes, r, end)
                    || g == ' '
                    || g == '-'
                    || g == '|'
                    || g == '+'
                    || g == '>'
                    || g == '<'
                    || g == '/'
                    || g == '\\'
                {
                    break;
                }
                end += 1;
            }
            let text: String = grid[r][start..end].iter().collect();
            let t = text.trim();
            if !t.is_empty() {
                let x = cell_x(start) + ((end - start) as f64) * CHARW / 2.0;
                let y = cell_y(r) + CHARH / 2.0;
                labels.push(Label {
                    x,
                    y,
                    text: t.to_string(),
                });
            }
            c = end;
        }
    }

    let mut out = String::new();
    out.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {:.0} {:.0}\" font-family=\"sans-serif\"><defs><marker id=\"arrow\" viewBox=\"0 0 10 10\" refX=\"9\" refY=\"5\" markerWidth=\"6\" markerHeight=\"6\" orient=\"auto-start-reverse\"><path d=\"M 0 0 L 10 5 L 0 10 z\" fill=\"var(--accent)\"/></marker></defs>",
        width, height
    ));
    out.push_str("\n  <rect width=\"100%\" height=\"100%\" fill=\"transparent\"/>");
    for b in &boxes {
        let x = cell_x(b.c0);
        let y = cell_y(b.r0);
        let w = (b.c1 - b.c0 + 1) as f64 * CHARW;
        let h = (b.r1 - b.r0 + 1) as f64 * CHARH;
        out.push_str(&format!(
            "\n  <rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" rx=\"6\" fill=\"var(--card)\" stroke=\"var(--accent)\" stroke-width=\"1.5\"/>",
            x, y, w, h
        ));
    }
    for l in &lines {
        if l.arrow {
            out.push_str(&format!(
                "\n  <line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"var(--accent)\" stroke-width=\"1.5\" marker-end=\"url(#arrow)\"/>",
                l.x0, l.y0, l.x1, l.y1
            ));
        } else {
            out.push_str(&format!(
                "\n  <line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"var(--ink-soft)\" stroke-width=\"1.5\"/>",
                l.x0, l.y0, l.x1, l.y1
            ));
        }
    }
    for lb in &labels {
        let sub: Vec<&str> = lb.text.split('\n').collect();
        let n = sub.len() as f64;
        let lineh = CHARH;
        let first_y = lb.y - ((n - 1.0) * lineh) / 2.0;
        for (i, line) in sub.iter().enumerate() {
            let y = first_y + (i as f64) * lineh + CHARH * 0.6;
            out.push_str(&format!(
                "\n  <text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" fill=\"var(--ink)\" font-size=\"12\">{}</text>",
                lb.x, y, escape_text(line)
            ));
        }
    }
    out.push_str("\n</svg>");
    out
}

fn escape_text(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_box_and_arrow() {
        let input = "+------+      +------+\n|  A   | ---> |  B   |\n+------+      +------+";
        let svg = ascii_to_svg(input);
        assert!(svg.contains("<svg"));
        assert!(svg.contains("<rect"));
        assert!(svg.contains("</svg>"));
        assert!(svg.contains("A"));
        assert!(svg.contains("B"));
        assert!(svg.contains("marker-end"));
    }

    #[test]
    fn unicode_boxes_normalized() {
        let input = "┌──────┐\n│  Hi  │\n└──────┘";
        let svg = ascii_to_svg(input);
        assert!(svg.contains("<rect"));
        assert!(svg.contains("Hi"));
    }

    #[test]
    fn no_crash_on_empty() {
        assert!(ascii_to_svg("").contains("<svg"));
    }
}
