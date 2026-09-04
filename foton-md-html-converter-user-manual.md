# foton-md-html-converter — User Manual

This little tool turns Markdown (`.md`) and plain text (`.txt`) files into
finished HTML pages. Each page is a single self-contained file: the styling,
the color themes, and the toolbar are all inside it. No installation, no
settings files, no internet connection. The generated page always appears in
the same folder as the source file, right next to it.

## Drag and drop (easiest way)

Select one or more `.md` / `.txt` files, or entire folders, in File Explorer
and drop them onto `foton-md-html-converter-x64.exe`.

- Every dropped file gets its own HTML page beside it.
- A dropped folder converts every `.md` / `.txt` file inside it (that folder
  only, no subfolders).
- A bad item (wrong file type, missing file, empty folder) is skipped; the
  rest still convert. No window or text appears — the tool simply runs and
  finishes.
- The drag-and-drop mode cannot pass a kicker label; use the command line if
  you want one.

## Double-click

Double-clicking the exe converts every `.md` and `.txt` file sitting in the
same folder as the exe. Handy for keeping the exe in your documents folder
and dropping fresh notes next to it.

## Command line

The tool is silent: it opens no console window and prints nothing. It does
set an exit code scripts can read — 0 when everything converted, 1 when
anything failed.

    foton-md-html-converter-x64 [--target <path>] [--kicker <text>] [PATH...]

| Argument | What it does |
|---|---|
| `PATH...` | One or more files and/or folders to convert (same as drag-and-drop). |
| `--target <path>` | Convert one file or one folder. Cannot be combined with `PATH...`. |
| `--kicker <text>` | Small label shown above the page title, e.g. `--kicker "Product Docs"`. |

With no arguments at all, the current folder is converted. Examples:

    foton-md-html-converter-x64 --target C:\docs\notes.md
    foton-md-html-converter-x64 --target C:\docs --kicker "Manual"
    foton-md-html-converter-x64 "C:\a.md" "C:\b.txt" "C:\archive"

## The page it produces

Open the generated `.html` file in any browser. It has a fixed toolbar in the
top-right corner:

- **Mode toggle** — switches the page between light and dark. On load the
  page follows your system preference.
- **Font toggle** — switches body and headings between serif and sans-serif;
  italics and code stay unchanged.
- **Palette picker** — Regular (default), Mono, or Solarized.
- **Brightness and saturation sliders** — each has seven steps; saturation is
  disabled in Mono.

These choices are per page view and reset when you reload; the page stores
nothing on your computer.

## Good to know

- Input formats: `.md` and `.txt` only, and only in the folder you point at —
  subfolders are not scanned.
- Markdown support includes headings (numbered sections), bold, italic, code,
  links, tables, lists, quotes, and fenced code blocks.
- Output never overwrites anything except the `.html` file with the same name
  as its source.
