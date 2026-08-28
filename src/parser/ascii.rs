#![allow(clippy::needless_range_loop)]
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicUsize, Ordering};

const CHARW: f64 = 9.0;
const CHARH: f64 = 18.0;

static DIAGRAM_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Debug)]
struct Label {
    text: String,
    r0: usize,
    c0: usize,
    r1: usize,
    c1: usize,
}

impl Label {
    fn cx(&self) -> f64 {
        ((self.c0 + self.c1) as f64 + 1.0) * CHARW / 2.0
    }
    fn cy(&self) -> f64 {
        ((self.r0 + self.r1) as f64 + 1.0) * CHARH / 2.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Dir {
    H,  // horizontal
    V,  // vertical
    Dn, // \  (down-right)
    Up, // /  (up-right)
}

#[derive(Clone, Debug)]
struct Connector {
    dir: Dir,
    r0: usize,
    c0: usize,
    r1: usize,
    c1: usize,
    arrow: bool,
}

// ---------------------------------------------------------------------------
// 1. Decode: turn raw ASCII into a grid, labels, and connector strokes.
// ---------------------------------------------------------------------------

fn normalize(input: &str) -> Vec<Vec<char>> {
    let lines: Vec<&str> = input.split('\n').collect();
    let width = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        let mut row = chars.clone();
        row.resize(width, ' ');
        grid.push(row);
    }
    for row in grid.iter_mut() {
        for ch in row.iter_mut() {
            *ch = match *ch {
                '─' | '━' | '−' => '-',
                '│' | '┃' => '|',
                '┌' | '┐' | '└' | '┘' | '├' | '┤' | '┬' | '┴' | '┼' | '╞' | '╡' | '╥' | '╨'
                | '╫' | '+' => '+',
                '→' | '➜' | '➤' => '>',
                '←' => '<',
                '↑' => '^',
                '↓' => 'v',
                _ => *ch,
            };
        }
    }
    grid
}

fn is_connector(ch: char) -> bool {
    matches!(ch, '-' | '|' | '+' | '/' | '\\' | '>' | '<' | '^' | 'v')
}

fn find_labels(grid: &[Vec<char>]) -> Vec<Label> {
    let rows = grid.len();
    if rows == 0 {
        return Vec::new();
    }
    let cols = grid[0].len();
    let mut used = HashSet::new();
    let mut labels = Vec::new();
    for r in 0..rows {
        let mut c = 0;
        while c < cols {
            if used.contains(&(r, c)) {
                c += 1;
                continue;
            }
            let ch = grid[r][c];
            if ch == ' ' || is_connector(ch) {
                c += 1;
                continue;
            }
            let start = c;
            let mut end = c;
            while end < cols {
                let g = grid[r][end];
                if g == ' ' || is_connector(g) {
                    break;
                }
                end += 1;
            }
            let text: String = grid[r][start..end].iter().collect();
            let t = text.trim();
            if !t.is_empty() {
                for cc in start..end {
                    used.insert((r, cc));
                }
                labels.push(Label {
                    text: t.to_string(),
                    r0: r,
                    c0: start,
                    r1: r,
                    c1: end - 1,
                });
            }
            c = end;
        }
    }
    labels
}

fn find_connectors(grid: &[Vec<char>]) -> Vec<Connector> {
    let rows = grid.len();
    if rows == 0 {
        return Vec::new();
    }
    let cols = grid[0].len();
    let mut used: HashSet<(usize, usize)> = HashSet::new();
    let mut out = Vec::new();

    // Horizontal runs: -, possibly ending with > or starting with <
    for r in 0..rows {
        let mut c = 0;
        while c < cols {
            if used.contains(&(r, c)) {
                c += 1;
                continue;
            }
            let ch = grid[r][c];
            if ch == '-' || ch == '<' {
                let start = c;
                let mut arrow = false;
                let mut c2 = c;
                if ch == '<' {
                    c2 += 1;
                    arrow = true;
                }
                while c2 < cols && grid[r][c2] == '-' {
                    c2 += 1;
                }
                if c2 < cols && grid[r][c2] == '>' {
                    arrow = true;
                    c2 += 1;
                }
                let end = c2.saturating_sub(1);
                if end >= start {
                    for cc in start..=end {
                        used.insert((r, cc));
                    }
                    out.push(Connector {
                        dir: Dir::H,
                        r0: r,
                        c0: start,
                        r1: r,
                        c1: end,
                        arrow,
                    });
                }
                c = c2;
            } else {
                c += 1;
            }
        }
    }

    // Vertical runs: |, possibly ending with v or starting with ^
    for c in 0..cols {
        let mut r = 0;
        while r < rows {
            if used.contains(&(r, c)) {
                r += 1;
                continue;
            }
            let ch = grid[r][c];
            if ch == '|' || ch == '^' {
                let start = r;
                let mut arrow = false;
                let mut r2 = r;
                if ch == '^' {
                    r2 += 1;
                    arrow = true;
                }
                while r2 < rows && grid[r2][c] == '|' {
                    r2 += 1;
                }
                if r2 < rows && grid[r2][c] == 'v' {
                    arrow = true;
                    r2 += 1;
                }
                let end = r2.saturating_sub(1);
                if end >= start {
                    for rr in start..=end {
                        used.insert((rr, c));
                    }
                    out.push(Connector {
                        dir: Dir::V,
                        r0: start,
                        c0: c,
                        r1: end,
                        c1: c,
                        arrow,
                    });
                }
                r = r2;
            } else {
                r += 1;
            }
        }
    }

    // Diagonal runs: / and \
    for r in 0..rows {
        for c in 0..cols {
            if used.contains(&(r, c)) {
                continue;
            }
            let ch = grid[r][c];
            if ch != '/' && ch != '\\' {
                continue;
            }
            let (step_r, step_c): (isize, isize) = if ch == '/' { (1, -1) } else { (1, 1) };
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
                if used.contains(&(ur, uc)) || grid[ur][uc] != ch {
                    break;
                }
                cells.push((ur, uc));
            }
            cells.reverse();
            cells.push((r, c));
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
                if used.contains(&(ur, uc)) || grid[ur][uc] != ch {
                    break;
                }
                cells.push((ur, uc));
            }
            for &(ur, uc) in &cells {
                used.insert((ur, uc));
            }
            if !cells.is_empty() {
                let dir = if ch == '/' { Dir::Up } else { Dir::Dn };
                out.push(Connector {
                    dir,
                    r0: cells[0].0,
                    c0: cells[0].1,
                    r1: cells.last().unwrap().0,
                    c1: cells.last().unwrap().1,
                    arrow: false,
                });
            }
        }
    }

    out
}

fn is_box_border(grid: &[Vec<char>], conn: &Connector) -> bool {
    let rows = grid.len();
    let cols = if rows == 0 { 0 } else { grid[0].len() };
    let is_text = |r: usize, c: usize| {
        if r >= rows || c >= cols {
            return false;
        }
        let ch = grid[r][c];
        ch != ' ' && !is_connector(ch)
    };
    match conn.dir {
        Dir::H => {
            let r = conn.r0;
            for c in conn.c0..=conn.c1 {
                if is_text(r.saturating_sub(1), c) || is_text((r + 1).min(rows.saturating_sub(1)), c) {
                    return true;
                }
            }
            false
        }
        Dir::V => {
            let c = conn.c0;
            for r in conn.r0..=conn.r1 {
                if is_text(r, c.saturating_sub(1)) || is_text(r, (c + 1).min(cols.saturating_sub(1))) {
                    return true;
                }
            }
            false
        }
        _ => false,
    }
}

fn nearest_in_direction(labels: &[Label], x: f64, y: f64, dx: f64, dy: f64) -> Option<usize> {
    let mut best = None;
    let mut best_d = f64::INFINITY;
    for (i, lb) in labels.iter().enumerate() {
        let vx = lb.cx() - x;
        let vy = lb.cy() - y;
        let dot = vx * dx + vy * dy;
        if dot <= 0.0 {
            continue;
        }
        let d = vx * vx + vy * vy;
        if d < best_d {
            best_d = d;
            best = Some(i);
        }
    }
    best
}

fn connector_source_target(labels: &[Label], conn: &Connector) -> (Option<usize>, Option<usize>) {
    match conn.dir {
        Dir::H => {
            let y = cell_cy(conn.r0);
            let x0 = cell_x(conn.c0);
            let x1 = cell_x(conn.c1) + CHARW;
            let from = nearest_in_direction(labels, x0, y, -1.0, 0.0);
            let to = nearest_in_direction(labels, x1, y, 1.0, 0.0);
            (from, to)
        }
        Dir::V => {
            let x = cell_cx(conn.c0);
            let y0 = cell_y(conn.r0);
            let y1 = cell_y(conn.r1) + CHARH;
            let from = nearest_in_direction(labels, x, y0, 0.0, -1.0);
            let to = nearest_in_direction(labels, x, y1, 0.0, 1.0);
            (from, to)
        }
        Dir::Dn => {
            let x0 = cell_x(conn.c0);
            let y0 = cell_y(conn.r0);
            let x1 = cell_x(conn.c1) + CHARW;
            let y1 = cell_y(conn.r1) + CHARH;
            let from = nearest_in_direction(labels, x0, y0, -1.0, -1.0);
            let to = nearest_in_direction(labels, x1, y1, 1.0, 1.0);
            (from, to)
        }
        Dir::Up => {
            let x0 = cell_x(conn.c0) + CHARW;
            let y0 = cell_y(conn.r0);
            let x1 = cell_x(conn.c1);
            let y1 = cell_y(conn.r1) + CHARH;
            let from = nearest_in_direction(labels, x0, y0, 1.0, -1.0);
            let to = nearest_in_direction(labels, x1, y1, -1.0, 1.0);
            (from, to)
        }
    }
}

fn cell_x(c: usize) -> f64 {
    c as f64 * CHARW
}
fn cell_y(r: usize) -> f64 {
    r as f64 * CHARH
}
fn cell_cx(c: usize) -> f64 {
    c as f64 * CHARW + CHARW / 2.0
}
fn cell_cy(r: usize) -> f64 {
    r as f64 * CHARH + CHARH / 2.0
}

// ---------------------------------------------------------------------------
// 2. Rebuild logically: labels -> logical nodes, connectors -> logical edges.
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct LogicalNode {
    pub id: String,
    pub row: usize,
    pub x_order: usize,
    pub text: String,
}

#[derive(Clone, Debug)]
pub struct LogicalEdge {
    pub source_id: String,
    pub target_id: String,
    pub arrow: bool,
    pub loopback: bool,
}

#[derive(Clone, Debug)]
pub struct LogicalGraph {
    pub diagram_id: String,
    pub nodes: Vec<LogicalNode>,
    pub edges: Vec<LogicalEdge>,
}

fn build_logical_graph(
    labels: &[Label],
    connectors: &[Connector],
    grid: &[Vec<char>],
    diagram_id: &str,
) -> LogicalGraph {
    let mut nodes: Vec<LogicalNode> = Vec::new();
    let mut rows: HashMap<usize, Vec<(usize, &Label)>> = HashMap::new();

    for (i, lb) in labels.iter().enumerate() {
        rows.entry(lb.r0).or_default().push((i, lb));
    }

    let mut sorted_rows: Vec<usize> = rows.keys().copied().collect();
    sorted_rows.sort_unstable();

    for row_index in sorted_rows {
        let mut row_nodes = rows.remove(&row_index).unwrap();
        row_nodes.sort_by(|a, b| a.1.c0.cmp(&b.1.c0));
        for (x_order, (label_index, lb)) in row_nodes.iter().enumerate() {
            nodes.push(LogicalNode {
                id: format!("{}-n{}", diagram_id, label_index),
                row: row_index,
                x_order,
                text: lb.text.clone(),
            });
        }
    }

    let mut raw_edges: Vec<(usize, usize, usize, bool)> = Vec::new();
    for (ci, conn) in connectors.iter().enumerate() {
        if is_box_border(grid, conn) {
            continue;
        }
        let (from, to) = connector_source_target(labels, conn);
        if let (Some(f), Some(t)) = (from, to) {
            if f != t {
                raw_edges.push((f, t, ci, conn.arrow));
            }
        }
    }

    // Fan-out: per source, ensure distinct targets by connector and target ordering.
    let mut grouped: HashMap<usize, Vec<(usize, usize, bool)>> = HashMap::new();
    for (f, t, ci, arrow) in raw_edges {
        grouped.entry(f).or_default().push((t, ci, arrow));
    }

    let mut edges: Vec<LogicalEdge> = Vec::new();
    for (from, group) in grouped {
        let mut order: Vec<(usize, usize, bool)> = group;
        order.sort_by(|a, b| {
            let ca = &connectors[a.1];
            let cb = &connectors[b.1];
            let ma = (ca.c0 + ca.c1) as f64;
            let mb = (cb.c0 + cb.c1) as f64;
            ma.partial_cmp(&mb).unwrap()
        });

        let mut targets: Vec<usize> = order.iter().map(|x| x.0).collect();
        targets.sort_by(|a, b| labels[*a].cx().partial_cmp(&labels[*b].cx()).unwrap());
        targets.dedup();

        for (i, (_, _ci, arrow)) in order.iter().enumerate() {
            let to = targets.get(i).copied().unwrap_or(order[i].0);
            let source_row = labels[from].r0;
            let target_row = labels[to].r0;
            edges.push(LogicalEdge {
                source_id: format!("{}-n{}", diagram_id, from),
                target_id: format!("{}-n{}", diagram_id, to),
                arrow: *arrow,
                loopback: source_row > target_row,
            });
        }
    }

    LogicalGraph {
        diagram_id: diagram_id.to_string(),
        nodes,
        edges,
    }
}

// ---------------------------------------------------------------------------
// 3. Render graphically: emit DOM rows + data block; browser draws edges.
// ---------------------------------------------------------------------------

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn escape_js_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

fn render_html(graph: &LogicalGraph) -> String {
    let mut rows: HashMap<usize, Vec<&LogicalNode>> = HashMap::new();
    for node in &graph.nodes {
        rows.entry(node.row).or_default().push(node);
    }
    for row in rows.values_mut() {
        row.sort_by(|a, b| a.x_order.cmp(&b.x_order));
    }
    let mut sorted_rows: Vec<usize> = rows.keys().copied().collect();
    sorted_rows.sort_unstable();

    let mut layers = String::new();
    for row in sorted_rows {
        let nodes = rows.get(&row).unwrap();
        let mut node_html = String::new();
        for n in nodes {
            node_html.push_str(&format!(
                "<div class=\"diagram-node\" data-dg-node=\"{}\">{}</div>",
                n.id,
                escape_html(&n.text)
            ));
        }
        layers.push_str(&format!("<div class=\"diagram-row\">{}</div>", node_html));
    }

    let mut nodes_json = String::from("[");
    for (i, n) in graph.nodes.iter().enumerate() {
        if i > 0 {
            nodes_json.push(',');
        }
        nodes_json.push_str(&format!(
            "{{\"id\":\"{}\",\"row\":{},\"x\":{},\"text\":\"{}\"}}",
            n.id,
            n.row,
            n.x_order,
            escape_js_string(&n.text)
        ));
    }
    nodes_json.push(']');

    let mut edges_json = String::from("[");
    for (i, e) in graph.edges.iter().enumerate() {
        if i > 0 {
            edges_json.push(',');
        }
        edges_json.push_str(&format!(
            "{{\"s\":\"{}\",\"t\":\"{}\",\"arrow\":{},\"loop\":{}}}",
            e.source_id,
            e.target_id,
            e.arrow,
            e.loopback
        ));
    }
    edges_json.push(']');

    format!(
        r#"<div class="svg-diagram">
  <div class="diagram-graph" id="{}">
    <svg class="diagram-edges" width="100%" height="100%"></svg>
    <div class="diagram-layers">{}</div>
  </div>
</div>
<script>
(function(){{
  var data = {{"id":"{}","nodes":{},"edges":{}}};
  window.__diagramQueue = window.__diagramQueue || [];
  window.__diagramQueue.push(data);
}})();
</script>"#,
        graph.diagram_id, layers, graph.diagram_id, nodes_json, edges_json
    )
}

pub fn ascii_to_html(input: &str) -> String {
    let grid = normalize(input);
    let labels = find_labels(&grid);
    let connectors = find_connectors(&grid);
    let id = format!("dg-{}", DIAGRAM_COUNTER.fetch_add(1, Ordering::Relaxed));
    let graph = build_logical_graph(&labels, &connectors, &grid, &id);
    render_html(&graph)
}

// ---------------------------------------------------------------------------
// Tests: verify decode and logical structure.
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_box_and_arrow() {
        let input = "+------+      +------+\n|  A   | ---> |  B   |\n+------+      +------+";
        let html = ascii_to_html(input);
        assert!(html.contains("diagram-graph"));
        assert!(html.contains("diagram-node"));
        assert!(html.contains("A"));
        assert!(html.contains("B"));
        assert!(html.contains("__diagramQueue"));
    }

    #[test]
    fn unicode_boxes_normalized() {
        let input = "┌──────┐\n│  Hi  │\n└──────┘";
        let html = ascii_to_html(input);
        assert!(html.contains("diagram-node"));
        assert!(html.contains("Hi"));
    }

    #[test]
    fn no_crash_on_empty() {
        assert!(ascii_to_html("").contains("diagram-graph"));
    }

    #[test]
    fn hub_fan_out_distinct_targets() {
        let input = "+-----+\n| Hub |\n+--+--+\n  / | \\\n A   B   C";
        let html = ascii_to_html(input);
        assert!(html.contains("Hub"));
        assert!(html.contains("A"));
        assert!(html.contains("B"));
        assert!(html.contains("C"));
        let edge_count = html.matches("\"arrow\":").count();
        assert_eq!(edge_count, 3, "expected 3 edges, got {}", edge_count);
    }

    #[test]
    fn vertical_chain() {
        let input = " A \n | \n v \n B ";
        let html = ascii_to_html(input);
        let edge_count = html.matches("\"arrow\":").count();
        assert_eq!(edge_count, 1);
        assert!(html.contains("A"));
        assert!(html.contains("B"));
    }

    #[test]
    fn horizontal_chain() {
        let input = " A  --->  B ";
        let html = ascii_to_html(input);
        let edge_count = html.matches("\"arrow\":").count();
        assert_eq!(edge_count, 1);
    }

    #[test]
    fn diagonal_pair() {
        let input = "A   \n \\   \n   B";
        let html = ascii_to_html(input);
        let edge_count = html.matches("\"arrow\":").count();
        assert_eq!(edge_count, 1);
        assert!(html.contains("A"));
        assert!(html.contains("B"));
    }
}
