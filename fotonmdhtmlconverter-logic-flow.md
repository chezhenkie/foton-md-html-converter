# foton-md-html-converter -- internal logic flow

Version: v1.3.1

This document traces how a source `.md` or `.txt` file becomes a self-contained `.html` page, and records what currently works and what is still rough.

## 1. CLI entry and file discovery

`src/main.rs` uses `clap` to parse two optional arguments: `--target <path>` and `--kicker <text>`. If `--target` points to a directory, `io::files::collect_source_files` scans that directory and returns every file whose extension is `md` or `txt` (case-insensitive), sorted alphabetically. If `--target` points to a single file, the extension must be `md` or `txt` or the program exits with an error. If `--target` is omitted, the current working directory is scanned. Directory read failures abort with an explicit error instead of being reported as "no files found". There is no recursion and no output-directory option: every source file is converted individually and its `.html` sibling is written into the same folder.

## 2. Per-file conversion

`io::convert::convert_file` reads the entire source file as a string, then:

1. Picks a title. A hardcoded `TITLE_MAP` (`title_for`) maps a few known filenames (e.g. `01-containers.md`) to friendly titles; otherwise the file stem is used.
2. Extracts a lede from YAML front matter with `parser::markdown::extract_lede`. Only a `lede: ...` field between `---` fences is recognised.
3. Converts the Markdown body to HTML with `parser::markdown::md_to_html`.
4. Wraps the result in a full page with `html::builder::build_page`.
5. Writes the page to `<source-stem>.html` in the same folder.

## 3. Markdown parser

`src/parser/markdown.rs` is a single-pass line scanner. A leading UTF-8 BOM is stripped, it skips an initial YAML front-matter block delimited by `---`, then walks the remaining lines and maintains state flags for code blocks, tables, unordered lists, and ordered lists. Output is written into a single pre-sized `String` buffer; a `begin_line` helper inserts the element separator, so the document is never assembled twice (the older `Vec<String>` + `join` approach is gone). Code-block flushing is unified in one `flush_code_block` helper used both at fence close and at EOF.

- **Headings** (`#` to `######`) are matched with a regex. `h2` gets an auto-incremented section number and a decorative rule; `h3` and other levels are plain.
- **Paragraphs** are any non-empty, non-special line; their content is passed through the inline parser.
- **Fenced code blocks** start and end with `` ``` ``. A language tag becomes `class="language-<lang>"`. Content is HTML-escaped. All languages, including `svg`, are treated identically.
- **Tables** are detected when a trimmed line starts with `|`. The first such line becomes the header row (unless it consists entirely of separator cells like `|---|---|`, in which case it is skipped); a separator row of dashes/colons is consumed and discarded; subsequent rows become body rows. All cell content is inline-parsed.
- **Blockquotes** (`> ...`) become `<div class="callout"><p>...</p></div>`.
- **Unordered lists** start with `- ` or `* `; ordered lists start with `digits. `. Switching between the two closes the previous list and opens a new one.
- **Horizontal rules** are lines made of 3+ of one of `-`, `*`, `_` (spaces allowed, e.g. `----` or `- - -`); they close any open list/table and produce no output.
- **Unclosed blocks** at EOF are flushed: code blocks become `<pre><code>`.

## 4. Inline parser

`src/parser/inline.rs` first HTML-escapes `&`, `<`, `>`, `"`, `'`, then splits the text into code spans and non-code segments: code spans become `<code>...</code>` verbatim (no further transforms), everything else passes through regex replacements in fixed order: bold-italic `***...***`, bold `**...**`, italic `*...*`, and links `[text](url)`. Because escaping happens first, backtick content containing `<` or `&` renders correctly, and quote escaping prevents attribute injection in link URLs. The regexes are compiled once via `LazyLock`.

## 5. HTML page assembly

`src/html/builder.rs` constructs the final document. The constant part of the shell -- everything from the doctype through the `<style>` block, toolbar markup, and opening `<header>` tag, plus the three trailing script blocks -- is assembled once per process into two `LazyLock<String>` segments plus small `const` literals. `build_page` then splices only the dynamic parts (title, header, body) into a single `String::with_capacity` allocation, so the roughly 21 KB of embedded CSS and JS is never re-copied per converted file. It injects:

- Toolbar HTML from `src/html/toolbar.rs` (a fixed 24px bar with mode toggle, font toggle, palette buttons, brightness slider, saturation slider).
- CSS from `src/css/toolbar.rs`, `src/css/theme.rs`, and `src/css/content.rs`.
- JavaScript from `src/js/os_detect.rs` (sets `data-os` from user-agent), `src/js/theme_engine.rs` (palette/brightness/saturation logic), `src/js/menubar.rs` (toolbar event handling), and `src/js/init.rs` (initialises everything and respects `prefers-color-scheme`).

The page body is a `<div class="wrap">` containing a `<header>` (kicker, lede, title) and a `<main>` with the converted Markdown body.

## 6. CSS / JS injection

All styles and scripts are embedded as string constants in Rust modules. The release build uses LTO and symbol stripping. There are no external network dependencies, no localStorage, and no output files other than the `.html` sibling.

## 7. Output writing

`io::convert::convert_file` writes the assembled page with `std::fs::write` to `md_file.with_extension("html")`. If writing fails, the program exits with an error message. The tool does not create subdirectories and does not overwrite anything special -- it simply produces a sibling file.

## 8. What works

- CLI discovery of `.md` and `.txt` files in a folder or single-file conversion.
- Full-page output with toolbar, theme engine, OS-aware fonts, and responsive layout.
- Markdown: headings (auto-numbered h2), paragraphs, bold/italic/code/links (code spans protected from other inline transforms), fenced code blocks with language class, tables, blockquotes/callouts, unordered/ordered lists, horizontal rules, YAML front matter skipping, BOM tolerance, lede extraction.

## 9. What is rough / not handled

- No nested lists.
- Tables: no alignment syntax beyond the required separator row.
- Markdown links and images are not validated; raw URLs are not auto-linked.
