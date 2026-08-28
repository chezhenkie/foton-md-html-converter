# foton-md-html-converter -- internal logic flow

Version: v1.3.0

This document traces how a source `.md` or `.txt` file becomes a self-contained `.html` page, and records what currently works and what is still rough.

## 1. CLI entry and file discovery

`src/main.rs` uses `clap` to parse two optional arguments: `--target <path>` and `--kicker <text>`. If `--target` points to a directory, `io::files::collect_source_files` scans that directory and returns every file ending in `.md` or `.txt`, sorted alphabetically. If `--target` points to a single file, the extension must be `md` or `txt` or the program exits with an error. If `--target` is omitted, the current working directory is scanned. There is no recursion and no output-directory option: every source file is converted individually and its `.html` sibling is written into the same folder.

## 2. Per-file conversion

`io::convert::convert_file` reads the entire source file as a string, then:

1. Picks a title. A hardcoded `TITLE_MAP` (`title_for`) maps a few known filenames (e.g. `01-containers.md`) to friendly titles; otherwise the file stem is used.
2. Extracts a lede from YAML front matter with `parser::markdown::extract_lede`. Only a `lede: ...` field between `---` fences is recognised.
3. Converts the Markdown body to HTML with `parser::markdown::md_to_html`.
4. Wraps the result in a full page with `html::builder::build_page`.
5. Writes the page to `<source-stem>.html` in the same folder.

## 3. Markdown parser

`src/parser/markdown.rs` is a single-pass line scanner. It skips an initial YAML front-matter block delimited by `---`, then walks the remaining lines and maintains state flags for code blocks, SVG blocks, tables, unordered lists, and ordered lists.

- **Headings** (`#` to `######`) are matched with a regex. `h2` gets an auto-incremented section number and a decorative rule; `h3` and other levels are plain.
- **Paragraphs** are any non-empty, non-special line; their content is passed through the inline parser.
- **Fenced code blocks** start and end with `` ``` ``. A language tag becomes `class="language-<lang>"`. Content is HTML-escaped.
- **SVG fences** are code blocks whose language is exactly `svg`. If the raw content begins with `<svg`, it is pasted verbatim; otherwise it is handed to `parser::ascii::ascii_to_html` and wrapped in `<div class="svg-diagram">`.
- **Tables** are detected when a trimmed line starts with `|`. The first such line becomes the header row; a separator row of dashes/colons is consumed and discarded; subsequent rows become body rows. All cell content is inline-parsed.
- **Blockquotes** (`> ...`) become `<div class="callout"><p>...</p></div>`.
- **Unordered lists** start with `- ` or `* `; ordered lists start with `digits. `. Switching between the two closes the previous list and opens a new one.
- **Horizontal rules** (`---`, `***`, `___`) close any open list/table and produce no output.
- **Unclosed blocks** at EOF are flushed: code blocks become `<pre><code>`, SVG blocks become diagrams.

## 4. Inline parser

`src/parser/inline.rs` first HTML-escapes `&`, `<`, `>`, then applies regex replacements in fixed order: inline code `` `...` ``, bold-italic `***...***`, bold `**...**`, italic `*...*`, and links `[text](url)`. Because escaping happens first, backtick content containing `<` or `&` renders correctly. The regexes are compiled once via `LazyLock`.

## 5. ASCII diagram parser (the tricky part)

`src/parser/ascii.rs` now has three clearly separated layers.

### 5.1 Decode

The input is normalised so Unicode box-drawing characters
(`─`, `│`, `┌`, `┐`, `└`, `┘`, `├`, `┤`, `┬`, `┴`, `┼`, `→`, etc.)
become ASCII equivalents (`-`, `|`, `+`, `/`, `\`, `>`, `<`, `^`, `v`).
Then the grid is scanned twice:

1. Contiguous runs of non-whitespace, non-connector characters become text labels.
2. Connector strokes are collected: horizontal `-` runs ending in `>` or starting with `<`,
   vertical `|` runs ending in `v` or starting with `^`, and diagonal chains of `/` or `\`.

Strokes that look like ASCII box borders are filtered out
(a horizontal stroke with text directly above or below it, or a vertical stroke with text directly to its left or right).

### 5.2 Rebuild logically

Each label becomes a `LogicalNode` with a row (its original grid row) and an x-order within that row.
Each surviving connector becomes a `LogicalEdge` by looking in the natural direction of the stroke:
horizontal strokes connect the nearest label on the left to the nearest label on the right,
vertical strokes connect above to below, and diagonals connect along their axis.
When several connectors share the same source, they are sorted left-to-right by midpoint column,
target labels are sorted left-to-right by centre, and connector i is mapped to target i.
This guarantees that a fan-out such as Hub -> A/B/C produces three distinct edges.

### 5.3 Render graphically

The logical graph is emitted as HTML DOM nodes laid out in flex rows inside `.diagram-graph`.
A small inline data block pushes the node and edge lists into `window.__diagramQueue`.
The global `Diagrams` module then reads each queue entry, computes each node box centre from the live DOM,
and draws cubic-bezier SVG edges between them.
Downward edges use a smooth vertical S-curve; upward edges are drawn as dashed loopback curves with a side bulge.
Because the geometry comes from the rendered DOM, the diagram stays correct across resizes and font changes.

## 6. HTML page assembly

`src/html/builder.rs` constructs the final document. It injects:

- Toolbar HTML from `src/html/toolbar.rs` (a fixed 24px bar with mode toggle, font toggle, palette buttons, brightness slider, saturation slider).
- CSS from `src/css/toolbar.rs`, `src/css/theme.rs`, and `src/css/content.rs`.
- JavaScript from `src/js/os_detect.rs` (sets `data-os` from user-agent), `src/js/theme_engine.rs` (palette/brightness/saturation logic), `src/js/menubar.rs` (toolbar event handling), `src/js/diagrams.rs` (renders ASCII diagrams from decoded logical graphs), and `src/js/init.rs` (initialises everything and respects `prefers-color-scheme`).

The page body is a `<div class="wrap">` containing a `<header>` (kicker, lede, title) and a `<main>` with the converted Markdown body.

## 7. CSS / JS injection

All styles and scripts are embedded as string constants in Rust modules. The release build uses LTO and symbol stripping. There are no external network dependencies, no localStorage, and no output files other than the `.html` sibling.

## 8. Output writing

`io::convert::convert_file` writes the assembled page with `std::fs::write` to `md_file.with_extension("html")`. If writing fails, the program exits with an error message. The tool does not create subdirectories and does not overwrite anything special -- it simply produces a sibling file.

## 9. What works

- CLI discovery of `.md` and `.txt` files in a folder or single-file conversion.
- Full-page output with toolbar, theme engine, OS-aware fonts, and responsive layout.
- Markdown: headings (auto-numbered h2), paragraphs, bold/italic/code/links, fenced code blocks with language class, tables, blockquotes/callouts, unordered/ordered lists, horizontal rules, YAML front matter skipping, lede extraction.
- SVG passthrough when the fence starts with `<svg`.
- ASCII diagram fan-out: one source label can connect to several distinct target labels.
- DOM-based diagram rendering: nodes are laid out in flex rows and SVG edges are drawn from live box centres at runtime.
- Downward cubic-bezier edges and dashed loopback curves for upward edges.
- Unicode box-drawing normalisation.

## 10. What is rough / not handled

- ASCII diagram visual polish: node colors/types, edge spacing, label padding, arrow placement, and routing aesthetics need work.
- No orthogonal L-shaped routing for nodes that are offset both horizontally and vertically; curves can look awkward.
- No decision-diamond or non-rectangular shapes.
- Single-character labels can cause degenerate diagonal lines because the source and target boxes line up too closely.
- Free-standing multi-word labels outside boxes are split per word and may not connect correctly.
- No nested lists.
- Tables: no alignment syntax beyond the required separator row.
- Markdown links and images are not validated; raw URLs are not auto-linked.
