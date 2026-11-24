# HTML Style CSS Specsheet

Nine themes, one compact toolbar, OS-native fonts, a font switch. Toolbar: right-aligned, fixed 24px.

---

## 1. Themes

A theme is a name, a mode and six values: page background, card fill, ink, muted text, line and accent. Five themes are dark and four are light. Light opens on `vercel`, dark on `github`, and the page follows the OS `prefers-color-scheme` until the reader picks.

| Theme | Mode | `bg` | `card` | `ink` | `muted` | `line` | `accent` |
|---|---|---|---|---|---|---|---|
| github | dark | `#0D1117` | `#010409` | `#C9D1D9` | `#8B949E` | `#6E7681` | `#3B82F6` |
| harbor | dark | `#0F1A2B` | `#212121` | `#C9C9C9` | `#999999` | `#737373` | `#4FD1C5` |
| solarized | dark | `#002B36` | `#073642` | `#9DB0B0` | `#8FA5AA` | `#93A1A1` | `#D96F3C` |
| oledblack | dark | `#000000` | `#1A1A1A` | `#EDEDED` | `#A8A8A8` | `#6E6E6E` | `#D4D4D4` |
| ayu | dark | `#0B0E14` | `#0F131A` | `#BFBDB6` | `#59C2FF` | `#6C7380` | `#E6B450` |
| vercel | light | `#F3F3F1` | `#FFFFFF` | `#20242A` | `#4E545C` | `#C9CCD1` | `#9A6400` |
| tokyonight | light | `#F3F3F1` | `#F0F4FC` | `#232A3D` | `#4B5468` | `#C6CEDF` | `#2F5BB7` |
| cursor | light | `#F3F3F1` | `#F0F7F1` | `#212A24` | `#4C5850` | `#C5D2C7` | `#2A6350` |
| orng | light | `#F3F3F1` | `#FFF6EE` | `#2A2420` | `#5C4F46` | `#DBC8B9` | `#B4471E` |

The six values are the whole theme. No theme changes a size, a weight, a rule or a font.

```js
dark:  github, harbor, solarized, oledblack, ayu
light: vercel, tokyonight, cursor, orng
```

---

## 2. Tokens

`ThemeEngine.apply` writes these custom properties on `:root` and the state attributes on `<html>`.

| Token | Source | Use |
|---|---|---|
| `--page-bg` | `bg` | page background, zebra rows; also the toolbar fill |
| `--page-ink` | `ink` | body text, headings, table header text |
| `--page-muted` | `muted` | lede, kicker, callout text, table header labels |
| `--page-line` | `line` | hairlines, table, code and chip borders |
| `--card` | `card` | table surface, callouts, code blocks |
| `--accent` | `accent` | links, emphasis, section number, markers, header rule |
| `--pick-ink` | black or white | text on an accent fill; chosen by luminance of `accent` |
| `--tb-ink` | `muted` | toolbar text and icons |
| `--tb-line` | `line` | toolbar borders, separators, slider track |
| `--thumb` | mode | switch thumb fill: `#2E2E2E` dark, `#CBCBCB` light |
| `--thumb-line` | mode | switch thumb border: `#777777` dark, `#888888` light |

`--pick-ink` is `#000000` when the relative luminance of `accent` is above 0.179, otherwise `#FFFFFF`.

Print palette, written once on `:root` and used by the toggle and the print stylesheet:

| Token | Value |
|---|---|
| `--print-bg` | `#FFFFFF` |
| `--print-ink` | `#000000` |
| `--print-muted` | `#333333` |
| `--print-line` | `#9A9A9A` |

`html[data-theme="dark"]` mirrors the `github` values and `html[data-theme="light"]` the `vercel` values, so the first paint is already correct. `html[data-print="on"]` forces `color-scheme: light` and leaves `data-theme` untouched.

---

## 3. Theme Engine

`ThemeEngine.apply(state)` takes `{mode, dark, light, font, print, brightness, saturation}`.

- Sets `data-theme`, `data-font`, `data-print`, `data-dark` and `data-light` on `<html>`.
- Adjusts the six theme values, then writes the tokens above as inline properties on `:root`.
- `dark` and `light` are remembered separately; `mode` picks which one is applied.
- In print state the tokens alias the print palette instead of a theme, and the sliders are not applied.
- Unknown theme names fall back to `github` and `vercel`.

**Brightness:** 8 percent per step, range -3..+3. `factor = 1 + step * 0.08; v = clamp(round(v * factor), 0, 255)`

**Saturation:** 15 percent per step, range -3..+3, applied in HSL. `newS = clamp(s * (1 + step * 0.15), 0, 1)`

Brightness runs before saturation. The sliders change the applied values only; the chosen names and the print palette are untouched, and a reload starts both at 0.

---

## 4. Toolbar

Fixed, 24px high, right-aligned, 28px inset, fill `var(--page-bg)`, no bottom divider, 7px gap, 10px UI font.

| Control | Type | Behavior |
|---|---|---|
| Print friendly | Button, printer icon plus label | Forces the print tokens on screen. `aria-pressed` follows. |
| Mode | 44x20px switch | Light or dark. Moon left, sun right. Thumb 16px, travels 17px. `aria-checked` true in light. |
| Font | 44x20px switch | Serif or sans. ON is sans; `aria-checked` true in sans. I/line SVG icons. |
| Dark theme | Fold button, 68px minimum | Opens a menu of the five dark names. |
| Light theme | Fold button, 68px minimum | Opens a menu of the four light names. |
| Brightness | Range input, 48px track | -3..+3, step 1, 10px thumb, readout. |
| Saturation | Range input, 48px track | -3..+3, step 1, 10px thumb, readout. |

Button: 18px high, 1px border, radius 2px, padding 0 6px 3px. Fold caret: 3x4px solid triangle, 4px from the label. Menu: 21px below the button, 2px padding, same border, rows 16px high with 10px text; the checked row takes `--accent` fill and `--pick-ink` text.

Font switch: ON = `var(--font-ui)`, OFF = `var(--font-serif)`, and it drives body text and headings only. Every theme opens sans; a theme pick keeps the current font. The lede, `em` and callouts are serif italic in both positions. Code and table header labels keep their own faces.

Menus: `role="menu"` with `menuitemradio` rows. Picking a name applies it, switches to that mode, closes the menu and returns focus to the button. Opening focuses the checked row; arrows move, Home and End jump, Escape closes, a click outside closes.

A control is a native `<button>` or `<input>`, so Enter and Space work without handlers. Focus ring: 2px `--tb-ink`, 1px offset. `prefers-reduced-motion` stops the 250ms transitions.

Nothing is stored. A reload returns the page to the OS mode, the default theme names, sans and 0/0.

---

## 5. Print

The toggle and `@media print` share the print palette.

- On screen with the toggle on: white page, black ink, `#333333` muted, `#9A9A9A` line, black accent. `data-theme` keeps its value; the mode switch stays where it was.
- Selecting a theme or the mode switch ends the print state.
- Under `@media print` the toolbar is hidden, `.wrap` fills the page with no padding, and the tokens are forced with `!important`, which beats the inline theme properties.
- `break-inside: avoid` on the header, tables, pre blocks, callouts, kicker, lede and rules. Body uses `print-color-adjust: exact`.

---

## 6. OS-Aware Font Stacks

`data-os` is stamped on `<html>` by an inline `<head>` script. Three properties per OS:

| OS | `--font-ui` | `--font-serif` | `--font-mono` |
|---|---|---|---|
| **windows** | `"Segoe UI Variable Text", "Segoe UI", sans-serif` | `Georgia, "Times New Roman", serif` | `"Cascadia Mono", "Cascadia Code", Consolas, monospace` |
| **mac** | `-apple-system, "SF Pro Text", "Helvetica Neue", sans-serif` | `"New York", "Times New Roman", Times, serif` | `"SF Mono", Menlo, Monaco, monospace` |
| **ios** | `-apple-system, "SF Pro Text", "Helvetica Neue", sans-serif` | `"New York", Georgia, "Times New Roman", serif` | `"SF Mono", Menlo, monospace` |
| **android** | `Roboto, "Noto Sans", sans-serif` | `"Noto Serif", serif` | `"Roboto Mono", "Droid Sans Mono", "Noto Sans Mono", monospace` |
| **linux** | `Ubuntu, "Cantarell", "Noto Sans", sans-serif` | `"Liberation Serif", "Noto Serif", serif` | `"Ubuntu Mono", "DejaVu Sans Mono", monospace` |
| **unknown** | `"Segoe UI", Helvetica, Arial, sans-serif` | `Georgia, "Times New Roman", serif` | `"Cascadia Code", Consolas, monospace` |

No webfont is fetched. A missing face falls through the stack to the OS default.

---

## 7. Content

The document uses the six theme values and nothing else.

| Element | Rule |
|---|---|
| Header rule | 6px `--accent` top border |
| Kicker | UI font, 12px, uppercase, 0.22em, `--page-muted` |
| h1 | 38px/400, `--page-ink`; `h1 em` italic `--accent` |
| Lede | 18px serif italic `--accent`; `small` 12.5px UI font |
| h2 | 26px/400 `--page-ink`, 56px top margin; `.num` 14px `--accent` superscript |
| Section rule | 1px `--page-line` |
| h3 | 19px/600 `--page-ink` |
| Emphasis | `em` serif italic `--accent`; `strong` `--accent` 600 |
| Table | `--card` surface, 1px `--page-line` border |
| th | UI font, 12.5px uppercase `--page-muted` on `--card`; 2px `--accent` bottom rule |
| td | 14.5px `--page-ink`, 1px `--page-line` bottom border; even rows `--page-bg` |
| Callout | 4px `--accent` left border, `--card` fill, serif italic `--page-muted` |
| Inline code | Mono 13px, transparent fill, 1px `--page-line` border |
| Code block | Mono 13px, `--card` fill, 1px `--page-line` border |
| Lists | 8px item gap; markers `--accent` |

Breakpoint 620px: 18px side padding, h1 drops to 28px.

---

## 8. The Converter

`foton-md-html-converter-x64` reads `.md`/`.txt`, writes one self-contained page beside each source.

```bash
foton-md-html-converter-x64 [--target <path>] [--kicker <text>] [PATH...]
```

| Argument | Default | Meaning |
|---|---|---|
| `--target <path>` | cwd | Single `.md`/`.txt` file or directory. |
| `--kicker <text>` | empty | Small uppercase label above the body. |
| `PATH...` | none | Bare paths (drag-and-drop). Mutually exclusive with `--target`. |

**Output rules:** HTML next to source (no output-directory). `--target` file=folder=nonexistent/wrong-ext rules apply. Drag-and-drop: items processed in order; bad items skipped; exit 1 if any failed. Self-contained: all CSS/JS inline, nine themes, toolbar, OS-aware fonts, system preference detection, responsive (620px).

**Markdown:** the first `#` heading becomes the page title and is removed from the body; headings (h2 auto-numbered), bold/italic/code inline, links, fenced code blocks, tables (header+zebra), lists, callout blockquotes, horizontal rules (consumed), YAML front matter (skipped; `lede:` becomes the lede).

**Examples:**
```bash
foton-md-html-converter-x64
foton-md-html-converter-x64 --target C:\docs\notes.md
foton-md-html-converter-x64 --target C:\docs\ --kicker "Product Docs"
foton-md-html-converter-x64 "C:\docs\notes.md" "C:\docs\archive"
```

---

## Appendix A. Toolbar markup (verbatim)

```html
<div id="tb" role="group" aria-label="Page controls">
  <button class="tb-btn" id="print" type="button" aria-pressed="false" aria-label="Print friendly" title="Print friendly">
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
      <path d="M6 9V3h12v6"/>
      <path d="M6 18H4a2 2 0 0 1-2-2v-5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v5a2 2 0 0 1-2 2h-2"/>
      <rect x="6" y="14" width="12" height="8" rx="1"/>
    </svg>
    <span>print friendly</span>
  </button>
  <span class="s" aria-hidden="true"></span>
  <button class="toggle on" id="mode" type="button" role="switch" aria-checked="true" aria-label="Light mode" title="Light or dark mode">
    <span class="toggle-thumb" aria-hidden="true"></span>
    <svg class="ico-l" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" aria-hidden="true">
      <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>
    </svg>
    <svg class="ico-r" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" aria-hidden="true">
      <circle cx="12" cy="12" r="5"/>
      <line x1="12" y1="1" x2="12" y2="3"/><line x1="12" y1="21" x2="12" y2="23"/>
      <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"/><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"/>
      <line x1="1" y1="12" x2="3" y2="12"/><line x1="21" y1="12" x2="23" y2="12"/>
      <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"/><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"/>
    </svg>
  </button>
  <button class="toggle on" id="font" type="button" role="switch" aria-checked="true" aria-label="Sans-serif" title="Serif or sans-serif">
    <span class="toggle-thumb" aria-hidden="true"></span>
    <svg class="ico-l" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="1.2" aria-hidden="true">
      <line x1="3" y1="3" x2="11" y2="3"/>
      <line x1="7" y1="3" x2="7" y2="11"/>
      <line x1="3" y1="11" x2="11" y2="11"/>
    </svg>
    <svg class="ico-r" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="1.2" aria-hidden="true">
      <line x1="7" y1="3" x2="7" y2="11"/>
    </svg>
  </button>
  <span class="s" aria-hidden="true"></span>
  <div class="group">
    <button class="tb-btn fold" id="dark-theme" type="button" aria-expanded="false" aria-haspopup="true" aria-controls="dark-menu" title="Dark theme">dark theme</button>
    <div class="menu" id="dark-menu" role="menu" aria-label="Dark theme"></div>
  </div>
  <div class="group">
    <button class="tb-btn fold" id="light-theme" type="button" aria-expanded="false" aria-haspopup="true" aria-controls="light-menu" title="Light theme">light theme</button>
    <div class="menu" id="light-menu" role="menu" aria-label="Light theme"></div>
  </div>
  <span class="s" aria-hidden="true"></span>
  <div class="sl">
    <svg viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1" aria-hidden="true">
      <rect x="1" y="1" width="10" height="10"/>
      <polygon points="1,1 11,1 1,11" fill="currentColor"/>
    </svg>
    <input type="range" id="brightness" min="-3" max="3" step="1" value="0" aria-label="Brightness">
    <span class="v" id="brightness-value" aria-hidden="true">0</span>
  </div>
  <div class="sl">
    <svg viewBox="0 0 12 12" fill="currentColor" aria-hidden="true">
      <circle cx="6" cy="3" r="1.8"/>
      <circle cx="3.5" cy="7.5" r="1.8"/>
      <circle cx="8.5" cy="7.5" r="1.8"/>
    </svg>
    <input type="range" id="saturation" min="-3" max="3" step="1" value="0" aria-label="Saturation">
    <span class="v" id="saturation-value" aria-hidden="true">0</span>
  </div>
</div>
```

## Appendix B. Toolbar CSS (verbatim)

```css
#tb{position:fixed;top:0;left:0;right:0;height:24px;z-index:10;display:flex;align-items:center;justify-content:flex-end;gap:7px;padding:0 28px;background:var(--page-bg);color:var(--tb-ink);font:10px/1 var(--font-ui);user-select:none;transition:background .25s,color .25s}
#tb .s{width:1px;height:12px;background:var(--tb-line)}
#tb .tb-btn{display:inline-flex;align-items:center;justify-content:center;gap:4px;height:18px;padding:0 6px 3px;cursor:pointer;color:inherit;background:transparent;border:1px solid var(--tb-line);border-radius:2px;white-space:nowrap}
#tb .tb-btn svg{width:13px;height:13px}
#tb .tb-btn.on{background:var(--tb-ink);color:var(--page-bg);border-color:var(--tb-ink)}
#tb .tb-btn.fold{min-width:68px}
#tb .tb-btn.fold::after{content:"";width:0;height:0;margin-left:4px;border-left:3px solid transparent;border-right:3px solid transparent;border-top:4px solid currentColor}
#tb .group{position:relative;display:inline-flex}
#tb .menu{display:none;position:absolute;top:21px;right:0;z-index:20;flex-direction:column;min-width:100%;padding:2px;background:var(--page-bg);border:1px solid var(--tb-line);border-radius:2px}
#tb .menu.open{display:flex}
#tb .menu button{height:16px;padding:0 6px;cursor:pointer;text-align:left;font:inherit;color:inherit;background:transparent;border:0;border-radius:1px;white-space:nowrap}
#tb .menu button:hover,#tb .menu button:focus-visible{background:var(--tb-line)}
#tb .menu button[aria-checked="true"]{background:var(--accent);color:var(--pick-ink)}
#tb .toggle{position:relative;width:44px;height:20px;padding:0;cursor:pointer;background:transparent;border:0;color:inherit}
#tb .toggle-thumb{position:absolute;top:2px;left:5px;width:16px;height:16px;border-radius:2px;background:var(--thumb);border:1px solid var(--thumb-line);transition:transform .25s cubic-bezier(.4,0,.2,1),background .25s,border-color .25s}
#tb .toggle.on .toggle-thumb{transform:translateX(17px)}
#tb .toggle svg{position:absolute;top:50%;width:12px;height:12px;transform:translateY(-50%);pointer-events:none;transition:opacity .25s}
#tb .toggle .ico-l{left:7px;opacity:1}
#tb .toggle .ico-r{right:8px;opacity:.45}
#tb .toggle.on .ico-l{opacity:.45}
#tb .toggle.on .ico-r{opacity:1}
#tb .sl{display:inline-flex;align-items:center;gap:4px}
#tb .sl svg{width:10px;height:10px;flex-shrink:0}
#tb .sl input[type=range]{-webkit-appearance:none;appearance:none;width:48px;height:3px;padding:0;margin:0;cursor:pointer;outline:none;background:var(--tb-line);border:1px solid var(--tb-line);border-radius:2px}
#tb .sl input[type=range]::-webkit-slider-thumb{-webkit-appearance:none;appearance:none;width:10px;height:10px;background:var(--page-bg);border:1px solid var(--tb-ink);border-radius:2px;cursor:pointer}
#tb .sl input[type=range]::-moz-range-thumb{width:10px;height:10px;background:var(--page-bg);border:1px solid var(--tb-ink);border-radius:2px;cursor:pointer}
#tb .sl .v{min-width:14px;text-align:center;font-variant-numeric:tabular-nums}
#tb .tb-btn:focus-visible,#tb .toggle:focus-visible,#tb .sl input:focus-visible,#tb .menu button:focus-visible{outline:2px solid var(--tb-ink);outline-offset:1px}
@media(prefers-reduced-motion:reduce){#tb *{transition:none!important}}
```

## Appendix C. Scripts

The head boot script sets the mode and the sans default before paint:

```js
(function(){
  var mq=window.matchMedia&&window.matchMedia('(prefers-color-scheme: dark)');
  var root=document.documentElement;
  root.setAttribute('data-theme',mq&&mq.matches?'dark':'light');
  root.setAttribute('data-font','sans-serif');
  root.setAttribute('data-print','off');
})();
```

The init script follows the OS until the first manual pick:

```js
(function(){
  MenuBar.init({
    onChange:function(state){ThemeEngine.apply(state);}
  });
})();
```

`src/js/theme_engine.rs` holds the theme table and the apply path; `src/js/menubar.rs` holds the controls, menus and sliders. Both are embedded verbatim in every page.

## Appendix D. Content CSS (verbatim)

```css
*{box-sizing:border-box}
.wrap{max-width:990px;margin:0 auto;padding:64px 32px 96px}
header{border-top:6px solid var(--accent);padding-top:28px;margin-bottom:8px;transition:border-color .25s}
.kicker{font-family:var(--font-ui);font-size:12px;letter-spacing:.22em;text-transform:uppercase;color:var(--page-muted);margin:0 0 14px;transition:color .25s}
h1{font-size:38px;font-weight:400;line-height:1.2;color:var(--page-ink);margin:0 0 12px;letter-spacing:-.01em;transition:color .25s}
h1 em{font-style:italic;color:var(--accent);transition:color .25s}
.lede{font-family:var(--font-serif);font-size:18px;line-height:1.5;color:var(--accent);margin:0 0 4px;font-style:italic;transition:color .25s}
.lede small{display:block;font-style:normal;font-family:var(--font-ui);font-size:12.5px;color:var(--page-muted);margin-top:10px;transition:color .25s}
h2{font-size:26px;font-weight:400;color:var(--page-ink);margin:56px 0 6px;letter-spacing:-.01em;transition:color .25s}
h2 .num{font-family:var(--font-ui);font-size:14px;color:var(--accent);vertical-align:super;margin-right:8px;transition:color .25s}
h2+.rule{height:1px;background:var(--page-line);margin:0 0 22px;transition:background .25s}
h3{font-weight:600;color:var(--page-ink);font-size:19px;margin:26px 0 6px;transition:color .25s}
p{margin:0 0 16px}
em{font-family:var(--font-serif);color:var(--accent);transition:color .25s}
strong{color:var(--accent);font-weight:600;transition:color .25s}
table{width:100%;border-collapse:collapse;margin:20px 0 28px;background:var(--card);border:1px solid var(--page-line);transition:background .25s,border-color .25s}
th,td{text-align:left;padding:10px 14px;vertical-align:top}
th{font-family:var(--font-ui);font-size:12.5px;letter-spacing:.04em;text-transform:uppercase;color:var(--page-muted);background:var(--card);border-bottom:2px solid var(--accent);font-weight:600;transition:background .25s,color .25s,border-color .25s}
td{font-size:14.5px;border-bottom:1px solid var(--page-line);color:var(--page-ink);transition:background .25s,border-color .25s,color .25s}
tr:last-child td{border-bottom:none}
tr:nth-child(even) td{background:var(--page-bg)}
.callout{border-left:4px solid var(--accent);background:var(--card);padding:16px 20px;margin:22px 0;font-family:var(--font-serif);font-style:italic;color:var(--page-muted);transition:background .25s,border-color .25s,color .25s}
.callout strong{font-family:var(--font-ui);font-style:normal}
ul,ol{margin:0 0 18px;padding-left:26px}
li{margin:0 0 8px}
li::marker{color:var(--accent)}
code{font-family:var(--font-mono);font-size:13px;background:transparent;color:var(--page-ink);border:1px solid var(--page-line);padding:1px 5px;border-radius:3px;transition:background .25s,color .25s,border-color .25s}
pre{background:var(--card);color:var(--page-ink);border:1px solid var(--page-line);padding:16px 20px;border-radius:4px;overflow-x:auto;margin:18px 0 24px;font-family:var(--font-mono);font-size:13px;line-height:1.6;transition:background .25s,color .25s,border-color .25s}
pre code{background:none;color:inherit;border:0;padding:0}
@media(max-width:620px){.wrap{padding:40px 18px 72px}h1{font-size:28px}}
```
