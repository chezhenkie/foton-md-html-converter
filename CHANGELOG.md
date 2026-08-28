# Changelog

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
