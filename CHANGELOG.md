# Changelog

## v1.3.0 (2026-08-28)

### Changed

- ASCII diagrams now render as DOM nodes + runtime SVG edges instead of a static coordinate-based SVG
- The parser still decodes labels and connectors logically; the graphical layer builds rows of HTML nodes and lets the browser lay them out
- Edges are drawn at runtime by computing box centers from the live DOM, using cubic-bezier curves like `flow-mockup-darkV7.html`
- Loopback edges (source below target) render as dashed curves with a side bulge

### Added

- `src/js/diagrams.rs` -- `Diagrams` module that consumes `window.__diagramQueue` and draws edges
- `.diagram-graph`, `.diagram-edges`, `.diagram-layers`, `.diagram-row`, `.diagram-node`, `.diagram-edge` CSS classes

## v1.2.0 (2026-08-28)

### Changed

- Rewrote ASCII-to-SVG diagram parser with a text-first approach: node labels are detected first, connectors are inferred from strokes (`-`, `|`, `/`, `\`, arrows), and boxes are drawn around the text afterwards
- Connectors are assigned to source/target labels by direction: horizontal strokes connect left-right labels, vertical strokes connect above-below labels, diagonals connect along their axis
- Fan-out routing: multiple connectors from one source map to distinct targets by horizontal ordering, fixing the Hub -> A/B/C misalignment bug
- Box borders from the original ASCII are filtered out so they do not create false edges
- Edge routing snaps to box edges: straight horizontal/vertical lines and straight diagonal lines

### Added

- `diagram-visual-tests/` folder with `.md` and generated `.html` samples for visual inspection

## v1.1.0 (2026-08-28)

### Features

- ASCII-to-SVG diagram rendering: a ` ```svg ` fenced block containing ASCII box-drawing now converts to an inline SVG diagram in the output page
- Diagram detection: boxes (`+`/`-`/`|` borders), text labels, `-->` arrows, horizontal/vertical connectors, and `/ \` diagonal branch lines
- Unicode box-drawing characters (`──│┌┐└┘├┤┬┴┼→`) are normalized to their ASCII equivalents before detection
- Raw SVG passthrough retained: if the ` ```svg ` content starts with `<svg`, it is passed through verbatim (no conversion)
- New `.svg-diagram` CSS wrapper (centered card) for rendered diagrams

## v1.0.0 (2026-08-26)

First release of the Rust port.

### Features

- CLI with `--target` (file or directory) and `--kicker` (optional label above title)
- Converts .md and .txt files to self-contained HTML pages
- Toolbar (24px fixed, right-aligned): mode toggle, font toggle, palette selector, brightness slider, saturation slider
- Three color palettes: Regular, Monochrome, Solarized (each with light + dark mode)
- Dynamic brightness (-3 to +3, 8% per step) and saturation (-3 to +3, 15% per step) sliders
- OS-aware fonts: Segoe UI (Windows), SF Pro Text (macOS/iOS), Roboto (Android), Ubuntu (Linux)
- Markdown parser: headings (h1-h6 with auto-numbered h2), paragraphs, bold/italic/code inline, links, fenced code blocks, tables, blockquotes, ordered/unordered lists, horizontal rules, YAML front matter skipping
- Same-folder output rule: HTML always lands next to its source

### Performance

- Regexes compiled once as statics (std::sync::LazyLock)
- Single-pass escape_html replacing three-pass approach
- Up to 170x faster than previous build on large files

### Build

- Rust edition 2024, toolchain pinned to 1.98.0
- Targets: x86_64-pc-windows-gnu (MinGW cross-linker), x86_64-unknown-linux-musl (static)
- Release profile: LTO + symbol stripping
- Windows exe metadata embedded via winresource (VERSIONINFO in build.rs)
