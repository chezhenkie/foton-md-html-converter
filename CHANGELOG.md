# Changelog

## Unreleased

### Removed

- The entire ASCII diagram feature: `src/parser/ascii.rs` (ASCII-to-DOM
  converter) and `src/js/diagrams.rs` (edge-drawing runtime) were deleted
- The ` ```svg ` fence special case; ` ```svg ` blocks are now rendered as
  plain escaped code blocks like any other language
- `.svg-diagram` and `.diagram-*` CSS rules, the `dg-dash` keyframes, and the
  `--edge` / `--node-bg` palette variables
- Test pages `testpage-htmlconverter-edge-cases.html` and
  `testpage-htmlconverter-box-detection.html`, and `diagram-design-specsheet.md`

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
