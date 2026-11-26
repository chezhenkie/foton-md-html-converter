# Markdown to styled HTML

![version 1.6.0](https://img.shields.io/badge/version-1.6.0-blue) ![platforms Windows, Linux](https://img.shields.io/badge/platforms-Windows%20%7C%20Linux-blue) ![license MIT](https://img.shields.io/badge/license-MIT-blue)

`foton-md-html-converter` reads Markdown and plain text files and writes a finished HTML page for each one. The page is a single self-contained file: styles, scripts, toolbar and themes are inside it, so it opens from a local folder or a share without fetching anything. The HTML lands in the same folder as its source. The converter is a native binary with no runtime, package manager or configuration file.

- Input comes from a drop, one `--target` file or folder, or the current folder.
- Nine themes, five dark and four light, with brightness and saturation controls on a fixed toolbar.
- Font stacks follow the host OS, with no webfonts and no external requests.
- The Markdown subset covers headings, emphasis, inline code, links, tables, lists, callouts and fenced code.
- The Windows release build is a silent GUI program; the Linux build prints progress. Both report through exit codes.

## Page

Every generated page carries the same offline toolbar: the back button sits at the far left, the other controls stay at the top right.

| Control | Behavior |
|---|---|
| Back | Follows browser history; greyed out when the tab has no earlier page. |
| Print friendly | White page, black ink, no toolbar in the printout. Selecting a theme or flipping the mode switch ends it. |
| Mode | Light or dark. The page follows the OS `prefers-color-scheme` setting until a mode or theme pick. A URL query `?mode=dark&dark=harbor&light=orng` opens it in that state instead. |
| Font | Serif or sans. Every theme opens sans; a theme change keeps the chosen font. Code and table header labels keep their own faces. |
| Themes | A `dark theme` menu of five names and a `light theme` menu of four. Picking one applies it and switches to that mode; each mode keeps its own pick. |
| Brightness | Seven steps from -3 to +3, 8 percent per step. |
| Saturation | Seven steps from -3 to +3, 15 percent per step. |

Nothing is stored. A user mode or theme change rewrites the current history entry, so browser Back and a reload keep the mode and both theme names. Without a query the page follows the OS mode on every fresh entry. A query carries the mode and the two theme names into every local `.html` link, plus a `via` marker that lets the target page know it has a page behind it; font, sliders and print stay out of it.

Pages are independent. A file converted later can join an existing folder without touching the others; give it a link in the index or in another page, and that link carries the theme query. Pages written before the query support do not read it, so reconvert them once. The query carries names, not colors: each page resolves them through its own theme table, so pages stay in sync by name while a project can retune its palette independently.

## Requirements

The converter is one executable per platform and needs no interpreter, package manager or network access. The page it writes needs only a browser.

| Platform | File | Notes |
|---|---|---|
| Windows x86_64 | `foton-md-html-converter-x64.exe` | GUI subsystem, no console window |
| Linux x86_64 | `foton-md-html-converter-linux-x64` | static musl, runs on any x86_64 distribution |

Building from source needs Rust 1.98.0 and the targets pinned in `rust-toolchain.toml`.

## Install

The two builds are committed in `dist/` and attached to the latest release, and every CI run uploads a build of each target as an artifact. Copy the binary anywhere; there is nothing to install.

```sh
git clone https://github.com/chezhenkie/foton-md-html-converter.git
chmod +x foton-md-html-converter/foton-md-html-converter-linux-x64
```

The `chmod` is for Linux; the Windows file runs as it is.

## Use

| Launch | Scope |
|---|---|
| Drop files or folders onto the exe | every dropped item, in order |
| Double-click the exe | the current folder, which Explorer sets to the folder holding the exe |
| `--target <path>` | one `.md` or `.txt` file, or one folder |
| `PATH...` | bare paths, the same input a drop produces |

A folder is scanned one level deep for `.md` and `.txt`, case-insensitive. A dropped item that is missing, has another extension or is an empty folder counts as a failure and the remaining items still convert.

```text
foton-md-html-converter-x64 [--target <path>] [--kicker <text>] [PATH...]
```

| Argument | Meaning |
|---|---|
| `--target <path>` | One `.md`/`.txt` file or one folder. A bad target exits 1 before anything is converted. |
| `--kicker <text>` | Label above the document body, for example `--kicker "Product Docs"`. Omitted when empty. A drop cannot pass it. |
| `PATH...` | One or more files or folders. Mutually exclusive with `--target`; combining them exits 1. |

```sh
# every .md and .txt in the current folder
foton-md-html-converter-x64

# one file, HTML beside it
foton-md-html-converter-x64 --target C:\docs\notes.md

# one folder with a kicker
foton-md-html-converter-x64 --target C:\docs --kicker "Product Docs"

# mixed files and folders, as produced by a drop
foton-md-html-converter-x64 "C:\docs\notes.md" "C:\docs\archive" "C:\misc\readme.txt"
```

A source keeps its name and gets a `.html` beside it: `notes.md` becomes `notes.html`. An existing page of that name is replaced; the source is never modified.

The page heading and the browser tab take the document's first `#` heading when it opens the document; that heading is removed from the body so it appears once. A document without a leading heading gets no injected heading, and the tab falls back to the file name, where the six filenames `01-containers.md` through `06-client-utilities.md` keep fixed titles. A `lede:` line in YAML front matter becomes the italic line under the kicker, and `--kicker` supplies the label above it.

The Windows exe opens no console and prints nothing in any mode; success is reported by the exit code and by the files it writes. The Linux build prints the folder, one line per converted file and an error line per failure. Both return 0 when every input converted and 1 when any failed.

## Markdown

| Syntax | Result |
|---|---|
| `#` to `######` | Heading; `##` gets an automatic section number |
| `**bold**`, `*italic*`, `***both***` | `<strong>`, `<em>`, combined |
| `` `code` `` | Inline `<code>` |
| Fenced block, optional language tag | Escaped code in `<pre><code>`, language kept as a class |
| `[text](url)` | Link |
| `\| ... \|` | Table; the first row is the header, a separator row is dropped |
| `- item`, `* item`, `1. item` | Unordered or ordered list |
| `> text` | Callout box |
| `---`, `***`, `___` | Horizontal rule, consumed |
| `--- ... ---` at the start | YAML front matter, skipped; `lede:` becomes the lede |

## Files

| Path | Purpose |
|---|---|
| `src/main.rs` | Command line, input modes, exit codes |
| `src/lib.rs` | Module tree and the per-platform output macros |
| `src/parser/` | Block parser and inline formatting |
| `src/html/` | Page assembly and toolbar markup |
| `src/css/`, `src/js/` | Embedded page styles and scripts |
| `src/io/` | Folder scan and per-file conversion |
| `src/build.rs` | Windows version resource and icon |
| `docs/` | Specsheet, manuals and benchmark |
| `.github/workflows/build.yml` | Format, lint, test and both builds |

## Check

Run from the `src` directory:

```sh
cd src
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

The suite has 48 tests covering the parser, escaping, page assembly, toolbar controls, the back button, the theme query and folder scan. CI runs the three commands on Linux and Windows, builds the musl and Windows binaries, and uploads each as an artifact kept for seven days.

## Limits

- Output is always beside the source. There is no output directory.
- One folder level is scanned; subfolders are ignored.
- A paragraph is one source line. Hard-wrapped text becomes a series of paragraphs.
- Nested lists flatten to one level. Images, reference links, footnotes and underscore emphasis are outside the parser.
- Fenced blocks render as escaped code; there is no diagram rendering.
- Settings are per view and reset on reload, except the mode and both theme names: a user change keeps them in the current history entry, and a query carries them to linked pages. Font, sliders and print reset. The page stores nothing.
- Prebuilt files cover x86_64 Windows and Linux. There is no macOS or ARM build.
- A Windows shell does not wait for a GUI program. Scripts start the exe with `Start-Process -Wait` or `start /wait` before reading the exit code.

## License

MIT. See [LICENSE](LICENSE).
