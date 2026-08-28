# Diagram Design Specsheet (v1.3.1 base)

The complete design spec for the ASCII-diagram-to-HTML rendering pipeline used by
`foton-md-html-converter`. This document covers the whole trajectory: the conceptual model,
the current v1.3.0 architecture, every color and dimension value, the rendering rules, the
known limitations, the planned roadmap, and the visual test programme. It is the single
source of truth for the diagram subsystem only. Theme and page-level styling are covered in
`html-style-css-specsheet.md`; the structural pipeline narrative lives in
`fotonmdhtmlconverter-logic-flow.md` (section 5).

This is a design document, not a verbatim implementation dump. Exact current values are given
where they matter (sizes, strokes, colors); aspirational items are clearly marked as "PLANNED"
or "FUTURE". Everything below is referenced against source files so the spec stays traceable.

---

## 1. Purpose and Scope

The converter turns fenced ASCII diagrams (```svg fences) into self-contained HTML. The diagram
subsystem is the only part of the product that generates graphical content from text; the rest of
the product is textual (headings, tables, code). This spec defines how that graphical layer must
look and behave.

In scope:

- ASCII-to-DOM and ASCII-to-SVG conversion rules.
- The three-layer pipeline (decode, logical rebuild, graphical render).
- Node and edge appearance, sizes, colors, and spacing.
- Theme integration (light/dark, three palettes, brightness/saturation).
- Arrow characters as input syntax; loops, fan-out/fan-in, and curve routing; direction shown by
  hover/tap flow animation (no drawn arrowheads).
- Runtime behaviour (resize, re-render, environment detection).
- Responsive behaviour and accessibility.
- The visual test suite and acceptance criteria.

Out of scope:

- Page chrome, toolbar, palettes, fonts (see the HTML style specsheet).
- The markdown parser outside diagram fences.
- Any external diagramming engine. Everything is generated from the raw ASCII text.

---

## 2. The Whole Trajectory (version view)

The diagram subsystem has evolved through three major releases. Each trajectory point is its own
rendering strategy, and each was a deliberate architectural choice.

| Version | Milestone | Rendering strategy | Result |
|---|---|---|---|
| v1.1.0 | commit d93a60d | Static SVG emission | SVG exactness is coordinate-fragile; hand-coded positions |
| v1.2.0 | text-first parser rewrite | Logical decode separated from rendering | Labels first, connectors second, box borders filtered |
| v1.3.0 | commit 6ef411c | DOM layout + runtime SVG edges | Browser positions nodes; JS draws edges from live geometry |
| v1.3.1 | (this commit) | Arrowheads removed; hover/tap flow animation | Direction shown only by lit dash-march on node hover/tap; tap pins |

### 2.1 Why v1.3.0 is the current base

Static SVG could not stay correct because everything depended on hand-computed coordinates that
ignored the true text width. v1.3.0 inverts the problem:

1. Decode the ASCII into nodes and edges (logical, font-agnostic).
2. Emit nodes as real DOM boxes laid out by the browser flexbox.
3. After layout, read each box centre with `getBoundingClientRect()`.
4. Draw cubic-bezier SVG edges between those live centres.

Because geometry comes from the rendered DOM, the diagram stays correct across font changes and
window resizes. This is the property the whole design hinges on.

### 2.2 Design goals

- Correction-proof: geometry is derived from the browser, not guessed.
- Text-first: labels win over strokes, so a border never becomes a false edge.
- Theme-aware: diagram colours come from the same CSS custom properties as the page.
- Self-contained: no network, no external assets, works from a single HTML file.

---

## 3. Input Model: the ASCII Fence

### 3.1 Activation

A fenced code block whose language is exactly `svg` triggers diagram handling:

    ```svg
    +-----+
    | Hub |
    +--+--+
      / | \
     A   B   C
    ```

Raw SVG passthrough: if the raw fence content starts with `<svg`, the content is pasted
verbatim (wrapped in `.svg-diagram`). Otherwise it is handed to
`parser::ascii::ascii_to_html`.

### 3.2 Supported characters

Unicode box-drawing and arrow characters are normalized to ASCII before processing:

| Input Unicode | Normalized |
|---|---|
| `--                                           ` (U+2500 box drawing horizontal) | `-` |
| `|`  (U+2502, U+2503) | `|` |
| `+`-family corners and joints (U+250C, U+2510, U+2514, U+2518, U+251C, U+2524, U+252C, U+2534, U+253C, and heavy variants/arrows) | `+` |
| `right` arrows (U+2192, U+279C, U+27A4) | `>` |
| `left` arrow (U+2190) | `<` |
| up arrow (U+2191) | `^` |
| down arrow (U+2193) | `v` |

Connector set (after normalization): `- | + / \ > < ^ v`.

### 3.3 Three logical inputs

- Text labels: contiguous non-space, non-connector runs.
- Connector strokes: the connector character set.
- Box borders: connector runs that enclose a label (see section 4.3).

---

## 4. The Three-Layer Pipeline (v1.3.0)

Source: `src/parser/ascii.rs`. Layers run in strict order.

### 4.1 Layer 1: Decode

- `normalize`: pad all rows to a common width; map Unicode to ASCII per section 3.2.- `find_labels`: scan every row; a label is a contiguous run of non-space, non-connector
  characters. One label per word per row (see known limitation 11.1).
- `find_connectors`: trace horizontal, vertical, and diagonal connector runs as `Connector`
  structs with direction, cell extent, and an `arrow` flag.

Direction enum values: H (horizontal), V (vertical), Dn (`\`, down-right), Up (`/`, up-right).

### 4.2 Layer 2: Logical rebuild (`build_logical_graph`)

Produces a `LogicalGraph` of `LogicalNode` and `LogicalEdge`.

- Nodes are grouped by the grid row of their label's top edge (`lb.r0`) and stored with an
  `x_order` reflecting their left-to-right column position.
- Each connector becomes an edge unless it is a box border (4.3).
- Directional assignment uses `nearest_in_direction`: an H connector finds the label directly to
  its left and right; a V connector finds the label above and below; diagonals match by quadrant.
- Fan-out: when one source has N connectors, the targets are gathered, sorted by their horizontal
  centre, deduplicated, and assigned one per connector in connector order. This is what fixes the
  "Hub collapses to one box" bug.
- Edges carry `loopback = source_row > target_row` for upward routing (section 7.3).

### 4.3 Box-border filtering (`is_box_border`)

A connector is a box border and is skipped as an edge when it is directly adjacent (above/right/
left/below, within the grid) to a text character. This stops the enclosing `+---+` frame of a box
from being misinterpreted as a fan-out edge.

Authoring note for this spec: box-border detection is deliberately adjacency-based. It is
correct on well-formed frames and deliberately conservative on malformed drawings. Do not widen
it into full rectangle recovery unless the roadmap item in 12.5 is greenlit.

### 4.4 Layer 3: Graphical render (`render_html`)

- Nodes emit as `<div class="diagram-row">` children, one row per distinct grid row of the
  logical nodes; within a row the nodes appear left-to-right.
- Each node is `<div class="diagram-node" data-dg-node="{id}">{text}</div>`.
- A data block defines the graph and is pushed to `window.__diagramQueue`:

    window.__diagramQueue = window.__diagramQueue || [];
    window.__diagramQueue.push({ id, nodes:[...], edges:[...] });

- The inline data is read by the `Diagrams` module at runtime (section 6).

---

## 5. Node Presentation (the boxes)

Source: `src/css/content.rs`.

### 5.1 Node box (.diagram-node)

| Property | Value |
|---|---|
| width | 160px |
| min-height | 64px |
| padding | 12px 14px |
| background | `var(--node-bg, var(--card))` |
| border | 1px solid `var(--line)` |
| border-top | 4px solid `var(--accent)` |
| border-radius | 3px |
| font-family | `var(--font-ui)` |
| font-size | 14px |
| font-weight | 600 |
| color | `var(--espresso)` |
| text-align | center |
| box-shadow | 0 1px 0 `var(--shadow-color)` |
| display | flex, column, justify-content center |
| overflow | hidden |

The 4px accent top edge is the primary identity cue: it reads as a "header strip" even though the
whole box is one node. The 1px `--line` border defines the outer boundary; the `--node-bg`
background separates the box from the `.diagram-graph` card behind it (important in dark mode).

### 5.2 Hover state

| Property | Value |
|---|---|
| box-shadow | 0 3px 10px `var(--shadow-hover)` |
| transform | translateY(-1px) |

Hover lifts the node 1px and strengthens the shadow. This is a pointer-affordance cue, not an
interaction; nodes are not links.

### 5.3 Node background per palette/mode (--node-bg)

| Palette | Mode | --node-bg |
|---|---|---|
| Regular | light | `#efeee8` |
| Regular | dark | `#222a3a` |
| Solarized | light | `#fdf6e3` |
| Solarized | dark | `#0b3f4e` |
| Monochrome | light | `#efefef` |
| Monochrome | dark | `#2e2e2e` |

Rule: `--node-bg` must differ from `--card` in the same palette/mode by enough to read as a
distinct surface in dark mode (the background of the graph card is `--card`; a node that equals
`--card` becomes invisible against it). The chosen dark values are one step lighter than `--card`.

---

## 6. Edge Presentation (the connecting lines)

Source: `src/css/content.rs` and `src/js/diagrams.rs`.

### 6.1 Edge stroke (.diagram-edge)

| Property | Value |
|---|---|
| fill | none |
| stroke | `var(--edge, var(--line))` |
| stroke-width | 2.8 |
| transition | stroke .35s ease |

### 6.2 Edge color per palette/mode (--edge)

| Palette | Mode | --edge |
|---|---|---|
| Regular | light | `#b5b3ab` |
| Regular | dark | `#8b97ab` |
| Solarized | light | `#93a1a1` |
| Solarized | dark | `#839496` |
| Monochrome | light | `#c2c2c2` |
| Monochrome | dark | `#cccccc` |

Rule: in light mode `--edge` is a light grey, clearly lighter than the ink text but still visible
against the card. In dark mode `--edge` is a mid-light value that cannot be allowed to equal the
near-black `--line` border (dark `--line` is effectively invisible against the graph card). The
edge colour is what the whole graph's connectivity reads from, so it must always be a visible
step above the card.

### 6.3 Edge geometry and curves

All edges are cubic bezier paths drawn at runtime between box centres.

- `p0` source point: the bottom centre of the source node `(S.x, S.y + S.h/2)` for a downward edge.
- `p3` target point: the top centre of the target node `(T.x, T.y - T.h/2)`.
- Downward control points: `c1=(p0.x, mid)`, `c2=(p3.x, mid)` where `mid=(p0.y+p3.y)/2`; this makes
  a smooth vertical S-curve.
- Loopback/upward edges: `p0` at source top `(S.x, S.y - S.h/2)`, `p3` at target bottom, with a
  side bulge of 90px so the returning curve bulges left of the nodes instead of overlapping them.
  The bulge keeps ascending paths visually distinct from descending ones.

### 6.4 Direction indication (no arrowheads)

There are no arrowhead markers. Static edges carry no direction cue: every path is a plain
stroke, directed or not. Direction is revealed only by the hover/tap flow animation (section 6.6),
whose dash march runs along the path from source point `p0` to target point `p3`.

Rule: the logical source/target of a reversed connector (`<---`, `^`) is swapped in the parser
(`connector_source_target`), so the path itself always starts at the true source. The renderer
never needs to know which arrowhead character produced the edge.

### 6.5 Loopback rendering

A logical edge with `loopback = true` gains the CSS class `diagram-edge loop` and renders with a
dashed stroke:

| Property | Value |
|---|---|
| stroke-dasharray | 5 4 |

A loopback is any edge whose source node row is strictly above its target node row in grid order
(upward edge). Dashes distinguish "return route" from "forward route".

### 6.6 Hover/tap flow animation

All edges are static by default (solid or loop-dashed, no motion). Interaction follows the
darkV7 mockup: hovering (or tapping) a node animates that node's OUTGOING edges and dims the rest.

| Selector | Meaning |
|---|---|
| `.diagram-edge.lit` | outgoing edge of the active node: `stroke: var(--accent)`, `stroke-width: 2.7`, `stroke-dasharray: 8 5`, `animation: dg-dash .9s linear infinite` |
| `@keyframes dg-dash` | `to { stroke-dashoffset: -26 }` (one full 13px pattern cycle, marching source to target) |
| `.diagram-edge.dim` | every other edge during activation: `opacity: .25` |

Trigger contract (in `renderDiagram`):

1. Each edge group carries `data-from` / `data-to` node ids.
2. `mouseenter` on a node: add `lit` to paths whose group has `data-from` equal to the node id,
   `dim` to all others. `mouseleave`: restore the pinned node's state, or clear all when nothing
   is pinned.
3. `click` on a node toggles a pinned id (same behaviour via touch tap on mobile, where hover
   does not exist). A click on the graph background unpins and clears.
4. No `prefers-reduced-motion` guard: on Windows desktops with system animations disabled the
   guard silently killed the whole feature. The animation is unconditional by decision.

---

## 7. Logical Routing Model

The logical model is the contract between the parser and the runtime renderer. Understanding it
is required before any routing work.

### 7.1 Node identity

Each node has:

- `id`: `{diagram-id}-n{label-index}` (source-unique string, used as a DOM lookup key).
- `row`: the grid row of the label's top edge (drives row grouping).
- `x_order`: left-to-right sequence within its row (drives flex order).
- `text`: the decoded label text.

### 7.2 Edge identity

Each edge has:

- `s` / `t`: source and target node ids.
- `arrow`: whether the edge is directed (it then participates in hover/tap flow animation; no
  arrowhead is drawn).
- `loop`: whether to render as an upward dashed loopback.
- `h` / `d`: whether the source connector was flat-horizontal / diagonal (geometry hints used by
  `drawEdge` to choose routing instead of guessing from pixel deltas).

### 7.3 Direction rules (built into `connector_source_target`)

| Connector dir | Source (from) | Target (to) |
|---|---|---|
| H | nearest label to the left | nearest label to the right |
| V | nearest label above | nearest label below |
| Dn (`\`) | nearest label up-left | nearest label down-right |
| Up (`/`) | nearest label up-right | nearest label down-left |

A connector that resolves to no label on either side produces no edge. A connector that resolves
to the same label for both sides produces no self-edge (skipped).

### 7.4 Fan-out (one source, many targets)

Algorithm in `build_logical_graph`:

1. Group raw edges by source.
2. For each source, sort its connectors by their horizontal midpoint.
3. Collect target labels, sort by horizontal centre, deduplicate.
4. Assign target i to connector i (in connector order).

This guarantees a Hub-style source with three visible connectors reaches three distinct targets
rather than collapsing all three onto the first one found.

---

## 8. Runtime Rendering (the Diagrams module)

Source: `src/js/diagrams.rs`.

### 8.1 Boot sequence

- `window.__diagramQueue` accumulates every diagram's data block at parse time.
- On `DOMContentLoaded` (or immediately if the document is already interactive), `initAll`
  runs `renderDiagram` for each queued entry.
- A 100ms debounced `resize` handler re-runs `initAll` so geometry refreshes on window/font change.

### 8.2 Rendering a diagram

1. Find the `.diagram-graph` element by `data.id`.
2. Locate the `<svg class="diagram-edges">` and the `.diagram-layers` container.
3. Clear previous SVG content (so re-render does not stack duplicate paths).
4. Build a node map from `.diagram-node[data-dg-node]`.
5. For each edge, run `drawEdge` to compute centres and append a `<g>` with one `<path>`.
6. Bind `mouseenter` / `mouseleave` / `click` on every node and a background-click clear on the
   graph, implementing the lit/dim/pin contract of section 6.6.

### 8.3 Coordinate source

`center(graph, el)` computes a node's position in SVG coordinate space:

    x = el.left - graph.left + el.width / 2
    y = el.top  - graph.top  + el.height / 2

Because this uses `getBoundingClientRect()` of both the node and the graph at drawing time, the
SVG overlay stays perfectly aligned to the flex-laid-out boxes.

### 8.4 Re-render hygiene

The SVG children are cleared before each render pass. The node DOM is created once at parse time
and never rebuilt on resize; only the SVG overlay is recomputed. This keeps layout stable and
re-rendering cheap.

---

## 9. Container and Layout

Source: `src/css/content.rs`.

### 9.1 .diagram-graph (the canvas)

| Property | Value |
|---|---|
| position | relative |
| background | `var(--card)` |
| border | 1px solid `var(--line)` |
| border-radius | 4px |
| padding | 30px 24px 34px |
| overflow-x | auto |
| min-height | 120px |

The graph is the positioning context for the absolute SVG overlay.

### 9.2 .diagram-edges (SVG overlay layer)

| Property | Value |
|---|---|
| position | absolute |
| inset | 0 |
| width / height | 100% |
| overflow | visible |
| pointer-events | none |
| z-index | 0 |

Edges must never intercept pointer events (they sit under the nodes, z-index 0).

### 9.3 .diagram-layers (node stacking layer)

| Property | Value |
|---|---|
| position | relative |
| z-index | 1 |
| display | flex, column |
| gap | 38px |

Nodes sit above the SVG overlay (z-index 1) so box hover interaction is never blocked.

### 9.4 .diagram-row

| Property | Value |
|---|---|
| display | flex |
| justify-content | space-evenly |
| align-items | flex-start |
| gap | 46px |
| padding | 0 12px |

Rows distribute their nodes evenly across the canvas width; nodes are top-aligned so ragged
heights grow downward and do not misalign neighbours.

### 9.5 Page wrapper (.svg-diagram)

| Property | Value |
|---|---|
| display | block |
| text-align | center |
| margin | 24px 0 |
| padding | 20px |
| background | `var(--card)` |
| border | 1px solid `var(--line)` |
| border-radius | 4px |

---

## 10. Responsive Behaviour

Mobile breakpoint at 620px (matching the rest of the product). The diagram shrinks but keeps its
structure fan-out/fan-in semantics intact.

| Element | Desktop | Mobile (max 620px) |
|---|---|---|
| .diagram-node width | 160px | 120px |
| .diagram-node min-height | 64px | 54px |
| .diagram-node padding | 12px 14px | 10px |
| .diagram-node font-size | 14px | 12px |
| .diagram-row gap | 46px | 24px |

The enhanced resize handler recomputes edge geometry after the reflow, so edges stay attached at
mobile sizes. The graph container clips horizontally (overflow-x auto) if a diagram is wider than
the viewport.

---

## 11. Current Limitations and Behavioural Rules

These are accepted behaviours of v1.3.0. They must not be "fixed by coincidence" while working on
unrelated polish; each has an explicit roadmap gate.

### 11.1 Multi-word labels split (KNOWN)

`find_labels` treats any space as a word boundary, so a box containing
`Central dispatch gateway` becomes three separate node boxes, not one box with wrapping text.
Space-delimited labels inside a box do not merge. The whole box/coordinate area is fragile and
is gated behind roadmap item 12.5. Do not modify box detection or connector coordinates unless
that item is explicitly greenlit by the user.

### 11.2 Single-character degenerate diagonals

Two single-character nodes aligned closely can collapse a diagonal into a degenerate line because
their box centres are nearly coincident. Visually awkward but not a crash.

### 11.3 No orthogonal L-shaped routing

A node offset both horizontally and vertically gets a diagonal-looking curve rather than an
L-shaped elbow route. Functionally correct, aesthetically approximate.

### 11.4 Shape set is rectangular only

No decision diamonds, no rounded nodes, no ports. All nodes are rectangles.

### 11.5 Curves are always cubic-bezier, downward by default

A node that is both right-of and below a source routes as a downward S-curve even if it also
spans horizontally. No elbow detection.

### 11.6 Border filter is adjacency-based

Border detection relies on `is_box_border` adjacency checks. Well-formed frames work; malformed
or overlapping frames can produce stray edges or missed ones.

### 11.7 Text is not wrapped or truncated by design

Node text is placed in a 160px flex box with `overflow:hidden`. Long single-line text clips at
the node edge. There is deliberately no automatic hyphenation or ellipsis in v1.3.0.

---

## 12. Roadmap (whole trajectory forward)

Ordering is by dependency and risk. Each item is a gate; none is started without explicit user
approval. Items build on the v1.3.0 base already committed in 6ef411c.

### 12.1 Edge spacing and label padding polish (low risk, high value)

- Increase horizontal separation between parallel outgoing edges from a common source.
- Standardize a minimum vertical padding between node bottom and its outgoing curve.
- Keep the incoming curve endpoint just at the target node border so the lit dash march reads as
  "arriving at" the box.

### 12.2 Edge-style differentiation (low risk)

- Distinguish "data flow" vs "control flow" by exposing a modifier class on edges, letting the
  user choose dash, weight, or colour per edge via a future ASCII marker.
- Keep the light-grey/visible-colour rule from 6.2 as the baseline; modifiers only layer on top.

### 12.3 Direction cue refinement (superseded arrowheads)

Arrowheads were removed in v1.3.1; direction now reads from the hover/tap dash march (section 6.6).
Remaining polish ideas, all low priority:

- A persistent (non-hover) subtle flow option for exported/static builds, if ever requested.
- Shorten the lit dash cycle so very long edges animate at a readable per-second rate rather than
  a fixed .9s, if short hops make the march read as too fast.

### 12.4 Node type styling (medium risk)

- Support a compact variant (smaller width) for leaf/debug nodes.
- Add a "decision" node type (diamond approach) behind a dedicated toggle, leaving rectangles the
  default. Diamond geometry interacts with the centre-based edge model, so this is coordinated
  with 12.5.

### 12.5 Multi-word box text + orthogonal routing (HIGH RISK, gate-required)

This is the correct fix for their / the parser's two big gaps, but it touches the exact area that
keeps causing the agent to spiral (box detection, label merging, connector coordinates). It must
be broken into a separate scoped work session with its own plan and tests, and only started on
explicit user instruction:

- Merge space-delimited labels that share an enclosing box frame into a single node whose text
  wraps and pushes the box height naturally.
- Add orthogonal (L-shaped) routing for nodes offset both horizontally and vertically.
- Re-validate every existing fan-out/fan-in/loop test after the merge change.

### 12.6 Edge-case coverage (continuous)

- Loops (source above and behind target).
- Multi-row fan-in (many sources into one sink).
- Right-to-left and upward arrows.
- Unicode-only diagrams (full box-drawing set).
- Malformed boxes (open corner, missing edge) to lock the border-filter behaviour.

---

## 13. Theme Integration Contract

The diagram subsystem reads all its colours from CSS custom properties set by `ThemeEngine`. It
never hard-codes a colour.

Required variables (both must exist in every palette/mode for the diagram to render correctly):

| Variable | Role |
|---|---|
| `--card` | graph canvas + border backing |
| `--line` | node borders and graph frame |
| `--accent` | node top strip + lit edge flow colour |
| `--espresso` | node text |
| `--node-bg` | node interior surface (differ from `--card`, esp. dark) |
| `--edge` | connecting-line stroke (visible step above `--card`, esp. dark) |
| `--shadow-color` / `--shadow-hover` | node depth cues |
| `--font-ui` | node label font |

Brightness and saturation sliders adjust every CSS custom property (including `--edge` and
`--node-bg`) because `ThemeEngine` iterates the whole palette object. No diagram-specific colour
handling is needed beyond declaring the variables.

---

## 14. Test Programme and Acceptance Criteria

### 14.1 Automated test set (Rust, `src/parser/ascii.rs`)

| Test | Pass condition |
|---|---|
| basic_box_and_arrow | diagram-graph, diagram-node, A, B, __diagramQueue present |
| unicode_boxes_normalized | Unicode frame normalizes; Hi present as a node |
| no_crash_on_empty | empty input still yields a graph container |
| hub_fan_out_distinct_targets | exactly 3 edges; Hub, A, B, C all present |
| vertical_chain | exactly 1 edge; A and B present |
| horizontal_chain | exactly 1 edge |
| diagonal_pair | exactly 1 edge; A and B present |

The suite currently passes 43 tests (both parser modules). Any roadmap change that alters node or
edge counts must update these assertions in the same change.

### 14.2 Visual test programme (`diagram-visual-tests/`)

Seven git-ignored source/HTML pairs cover the routing space:

| File | Covers |
|---|---|
| 01-flat-right | single horizontal directed edge, flow left to right |
| 02-flat-left | reversed horizontal connector, flow right to left |
| 03-flat-chain | same-row chain of directed edges |
| 04-flat-fanout | one source, multiple distinct targets |
| 05-flat-tall-short | unequal node heights, flat horizontal routing |
| 06-flat-diagonal-mixed | horizontal + diagonal edges mixed |
| 07-branch-loopback | branch fan-out plus an upward dashed loopback |

Regeneration: run the Windows release exe against the folder after any source change, then
visually confirm in both light and dark mode and in all three palettes. The 620px viewport must
also be checked.

### 14.3 Acceptance criteria (across all palettes/modes)

1. Every node encloses its label with the accent top strip visible.
2. Every recognised connector renders as a visible edge (>= 2.8 stroke) that does not disappear
   against the card in dark mode.
3. Static edges carry no arrowheads; direction is only shown by the hover/tap flow animation.
4. Fan-out reaches all distinct targets; no two edges collapse onto one target.
5. Geometry survives window resize and font-toggle without detached edges.
6. In light mode edges read lighter than ink; in dark mode edges read clearly above the card.
7. Nodes read as distinct surfaces from the graph card in both modes.
8. Hovering or tapping a node lights its outgoing edges (accent, dash march) and dims the rest;
   leaving clears, and tap-pin persists until an unpinned click on the background.

---

## 15. Reference Documents

| Document | Relation |
|---|---|
| `html-style-css-specsheet.md` | page chrome, palettes, fonts (diagram colours declared there) |
| `fotonmdhtmlconverter-logic-flow.md` | structural pipeline narrative, section 5 |
| `md-to-styled-html-converter-script-manual.md` | user-facing behaviour of the converter |
| `docs/flow-mockup-darkV7.html` | visual/logic reference: flex rows + SVG edge overlay |
| `docs/flow-to-html-manual.md` | reference data model and flex-row overlay concept |

The `docs/` reference files are user-copied and git-ignored; they inform the spirit of the diagram
rendering but are not listed for verbatim porting.

---

## 16. Version Record

| Version | Commit | Change |
|---|---|---|
| v1.1.0 | d93a60d | Static SVG emission |
| v1.2.0 | (pre-6ef411c) | text-first parser rewrite; logical decode separated |
| v1.3.0 | 6ef411c | DOM layout + runtime SVG edges; --edge and --node-bg added; edge weight 2.8; lighter light edges; visual test folder updated |
| v1.3.1 | (this commit) | Arrowhead markers removed; direction via hover/tap dash-march flow (section 6.6) with lit/dim/pin; h/d geometry hints on edges; reversed connectors swap source/target; context-aware connector splitting and = normalization in parser; 43 tests; 7 visual test files |

This document is written against the v1.3.1 base. Future versions append a row above and update
the sections they touch; they do not rewrite history.

---
