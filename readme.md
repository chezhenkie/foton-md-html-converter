# markdown-to-stylecss-html-translate -- v7 Reference

> Tool: `foton-md-html-converter-x64`
> Converts Markdown/text files into styled, self-contained HTML pages **in the same folder as each source**.

## 1. Overview

A Rust CLI that reads Markdown (`.md`) or plain text (`.txt`) and writes a styled HTML page for each source. Each output page is fully self-contained (all CSS and JS inlined, no external dependencies) and embeds the **toolbar + theme engine** from `html-style-css-specsheet.md`:

- **Three palettes** -- Regular, Monochrome, Solarized
- **Compact toolbar** -- mode toggle, font toggle, palette selector, brightness/saturation sliders
- **ThemeEngine** -- dynamic color adjustment via brightness and saturation sliders
- **OS-aware fonts** (including Linux) and **responsive layout**

Pages open in **light mode + system-preferred font** by default; readers can adjust color, font, brightness, and saturation via the toolbar.

> **v7 is the only current converter.** v6 (old 3-toggle system) and earlier versions are obsolete.

### hard rule: same-folder output
Every generated `.html` file **always** lands in the same directory as its source file. There is **no output-directory concept** in v7 -- no separate or overridable output folder exists, so this rule cannot be violated. Point the script at a file or a folder, and the HTML appears right next to the sources.

## 2. Usage

```bash
foton-md-html-converter-x64 [--target <path>] [--kicker <text>]
```

All arguments are optional.

| Argument | Default | Meaning |
|---|---|---|
| `--target <path>` | *(current directory)* | Path to a single `.md`/`.txt` file **or** a directory to convert. Trailing slashes are ignored. When absent, every `.md` and `.txt` in the current directory is converted. |
| `--kicker <text>` | *(empty)* | Small uppercase label shown above the title on every content page (e.g. "REST API", "Knowledge Base"). Omitted if empty. |

**Output goes into the same folder as the source files.** There are no hardcoded paths and no output-directory argument.

## 3. Flags

| Flag | Type | Meaning |
|---|---|---|
| `--target <path>` | optional | Path to a single `.md`/`.txt` file **or** a directory. A directory (trailing slash optional) converts every `.md` and `.txt` inside it. Default: convert every `.md` and `.txt` in the current working directory. |
| `--kicker <text>` | optional | Kicker text shown above the title on each page. Omitted if empty. |

### `--target` path rules

`--target` accepts either a **file** or a **folder**:

- **File** (e.g. `C:\docs\notes.md`) -> only that one file is converted; its HTML is written into the same folder.
- **Folder** (e.g. `C:\docs\` or `c:\docs`) -> every `.md` and `.txt` in that folder is converted, each HTML written beside its source.
- **Nonexistent path** -> clear error and the script exits without converting anything.
- **Existing file that is not `.md` or `.txt`** -> clear error and the script exits without converting anything.

## 4. Markdown features

| Syntax | Output |
|---|---|
| `# ... ######` | Headings (h2 gets auto-numbered section counters) |
| `**bold**` / `*italic*` / `***both***` | `<strong>`, `<em>`, combined |
| `` `code` `` | Inline `<code>` |
| ```` ```lang ... ``` ```` | Fenced code block with optional language class |
| `[text](url)` | Anchor link |
| `\| ... \| ... \|` | Table with header row and zebra striping |
| `- item` / `1. item` | Unordered / ordered list |
| `> quote` | Callout blockquote |
| `---` / `***` / `___` | Horizontal rule (consumed, not rendered) |
| `--- ... ---` at file start | YAML front matter (skipped silently) |
| ```` ```svg ... ``` ```` | ASCII diagram auto-converted to DOM nodes + runtime SVG edges; if content starts with `<svg`, passed through raw |

## 5. Examples

```bash
# Convert every .md and .txt in the current directory (HTML beside each source)
foton-md-html-converter-x64

# Convert a single file (only it; HTML lands next to it)
foton-md-html-converter-x64 --target C:\docs\notes.md
foton-md-html-converter-x64 --target C:\docs\notes.txt

# Convert a whole folder (HTML beside each source), with or without trailing slash
foton-md-html-converter-x64 --target C:\docs\
foton-md-html-converter-x64 --target c:\docs

# With a kicker label on the generated pages
foton-md-html-converter-x64 --target C:\docs\ --kicker "Product Docs"
```

## 6. Output features

- **Same-folder output** -- each `.html` file is written next to its source file (`.md` or `.txt`); there is no output-directory option.
- **Self-contained** -- each HTML file includes all CSS and JS inline; no external dependencies.
- **Three palettes** -- Regular (default), Monochrome, and Solarized, selectable via the palette buttons.
- **Compact toolbar** -- 24px fixed bar with mode toggle, font toggle, palette selector, brightness/saturation sliders.
- **ThemeEngine** -- dynamic color adjustment: brightness (-3 to +3, 8% per step), saturation (-3 to +3, 15% per step).
- **Font toggle** -- switches body/headings between serif and sans-serif; italics and code stay unchanged.
- **System preference** -- detects `prefers-color-scheme` on load and applies matching mode.
- **OS-aware fonts** -- Segoe UI on Windows, SF Pro on macOS/iOS, Roboto on Android, Ubuntu on Linux; no webfonts/CDN.
- **Responsive** -- adapts to mobile viewports below 620px.
- **SVG diagrams** -- ASCII box-drawing blocks inside a ` ```svg ` fence render as DOM nodes with runtime SVG edges. Text labels become nodes, connector strokes (`-`, `|`, `/`, `\`, `-->`, `^`, `v`) become logical edges, and the browser lays out the rows before JavaScript draws cubic-bezier connectors between box centers. Edges carry no arrowheads; direction is shown by the hover/tap dash-march flow. Raw `<svg>` markup in the fence passes through verbatim.


## 7. Script structure

v7 is organized into four modules:

| Module | Contents |
|---|---|
| Module 1 | Toolbar/styling CSS -- literal from `menubar.css` + `theme.css` |
| Module 2 | ThemeEngine + MenuBar JS -- literal from `theme-engine.js` + `menubar.js` |
| Module 3 | Converter logic -- `md_to_html`, `TITLE_MAP`, `convert_file`, `main` |
| Module 4 | `build_page` -- toolbar markup, OS detection, init script |

## 8. Rust Implementation

v7 is implemented in Rust. Same behavior as the original Python script, modular architecture, standalone binaries.

### Module mapping

| Rust module | Python equivalent | Contents |
|---|---|---|
| `src/css/toolbar.rs` | Module 1 (TOOLBAR_CSS) | Static toolbar CSS |
| `src/css/theme.rs` | Module 1 (THEME_CSS) | Static theme CSS |
| `src/css/content.rs` | Module 1 (CONTENT_CSS) | Static content CSS |
| `src/js/theme_engine.rs` | Module 2 (THEME_ENGINE_JS) | ThemeEngine JS |
| `src/js/menubar.rs` | Module 2 (MENUBAR_JS) | MenuBar JS |
| `src/js/os_detect.rs` | Module 4 (OS_DETECT_JS) | OS detection JS |
| `src/js/init.rs` | Module 4 (INIT_JS) | Bootstrap JS |
| `src/html/toolbar.rs` | Module 4 (TOOLBAR_HTML) | Toolbar markup |
| `src/html/builder.rs` | Module 4 (build_page) | Page assembly |
| `src/parser/markdown.rs` | Module 3 (md_to_html) | Markdown parser |
| `src/parser/inline.rs` | Module 3 (inline, escape_html) | Inline formatting |
| `src/parser/ascii.rs` | Module 3 (ascii_to_html) | ASCII diagram to HTML/DOM rendering |
| `src/io/files.rs` | Module 3 (TITLE_MAP, collect_source_files) | File discovery |
| `src/io/convert.rs` | Module 3 (convert_file) | Conversion loop |
| `src/main.rs` | Module 3 (main) | CLI entry point |

### Build targets

| Target | Command | Binary |
|---|---|---|
| Windows (MSVC) | `cargo build --release` | `target/release/foton-md-html-converter-x64.exe` |
| Linux (musl, static) | `cargo build --release --target x86_64-unknown-linux-musl` | `target/x86_64-unknown-linux-musl/release/foton-md-html-converter-x64` |

### Current status

- Rust port complete and functional
- Both Windows (GNU/MSVC) and Linux (musl static) release binaries build clean
- Conversion tested; output matches the original Python v7 behavior
- v1.3.1: DOM-node + runtime SVG edge diagram rendering with hover/tap
  dash-march direction flow (no arrowheads); see `diagram-design-specsheet.md`

### Windows exe metadata

The Windows `.exe` includes a VERSIONINFO resource that populates the Details tab in File Properties (company, product name, version, copyright, file description). This is a PE format feature -- Linux ELF binaries have no equivalent.

The resource is embedded at build time by `build.rs` using the `winresource` crate. When building the `x86_64-pc-windows-gnu` target, `winresource` calls `x86_64-w64-mingw32-windres` (from the MinGW toolchain in WSL) to compile the resource and link it into the binary. The Linux musl build skips this step.

To update metadata: edit the `set()` calls in `build.rs`, rebuild with `cargo build --release --target x86_64-pc-windows-gnu` inside WSL, and copy the exe from `target/x86_64-pc-windows-gnu/release/` to the project root.

### Linux musl build

The Linux binary targets `x86_64-unknown-linux-musl`, which produces a fully static executable with no runtime dependencies. A glibc-linked binary built on Ubuntu will fail to run on Alpine or older CentOS if the system glibc version is lower than the one used at build time. A musl static binary has no such constraint -- it runs on any x86_64 Linux distribution regardless of installed system libraries. The trade-off is slightly larger binary size and no dynamic linking, which is acceptable for a single-file CLI tool.
