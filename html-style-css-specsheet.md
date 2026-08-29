# HTML Style CSS Specsheet

A copy-paste recipe for pages with three palettes (Regular, Monochrome, Solarized), a compact toolbar (mode toggle, font toggle, palette selector, brightness/saturation sliders), and OS-native fonts.

**Toolbar layout (right-aligned, fixed 24px):** Mode toggle / Font toggle / Palette buttons (Regular | Mono | Solarized) / Brightness slider / Saturation slider.

---

## 1. What This Recipe Gives You

Everything needed to build a page with three complete color palettes, each with light and dark mode, a font toggle (serif vs sans-serif), and brightness/saturation adjustment:

| Included |
|---|
| Regular + Monochrome + Solarized palettes (CSS custom properties) |
| ThemeEngine: color calculation + CSS variable application |
| Compact toolbar: mode toggle, font toggle, palette selector, sliders |
| Brightness slider: -3 to +3 (8% per step) |
| Saturation slider: -3 to +3 (15% per step, disabled in mono) |
| Page chrome: header, kicker, h1, lede |
| Content styles: headings, tables, callouts, code |
| SVG diagram wrapper (.svg-diagram) for ASCII-to-DOM/SVG-edge rendering |
| OS detection + native font stacks (including Linux) |
| Responsive layout (mobile breakpoint at 620px) |
| System preference detection (prefers-color-scheme) |

---

## 2. The Regular Palette

The default palette: warm paper, amber accents, serif body text. Light theme in `:root`; dark theme applied by ThemeEngine. All colors are CSS custom properties set dynamically on `:root`.

### Light mode

| Variable | Value | Use |
|---|---|---|
| `--paper` | `#f4f3ef` | page background |
| `--ink` | `#2b2b2a` | primary text |
| `--ink-soft` | `#575756` | secondary text |
| `--espresso` | `#383837` | headings, table header |
| `--accent` | `#b08a2e` | accents, header rule |
| `--accent-deep` | `#8a6a24` | emphasis |
| `--bronze` | `#7a746b` | kicker, palette labels |
| `--blue` | `#4a6d8c` | accent |
| `--brick` | `#a84a3a` | accent |
| `--line` | `#d9d8d3` | hairlines, borders |
| `--card` | `#fafaf8` | surfaces, callouts |
| `--zebra` | `#f7f7f4` | even table rows |
| `--code-bg` | `#eceae4` | inline code |
| `--pre-bg` | `#343433` | code blocks |
| `--shadow-color` | `rgba(45,45,45,0.06)` | subtle shadows |
| `--pre-ink` | `#f2f2ef` | code block text |

### Dark mode

| Variable | Value |
|---|---|
| `--paper` | `#0a0c10` |
| `--ink` | `#d0ccc0` |
| `--ink-soft` | `#9a9688` |
| `--espresso` | `#a8b4c8` |
| `--accent` | `#4a6d8c` |
| `--accent-deep` | `#5a7d9c` |
| `--bronze` | `#7a8a9c` |
| `--blue` | `#4a6d8c` |
| `--brick` | `#8c6a6a` |
| `--line` | `#1a2030` |
| `--card` | `#141820` |
| `--zebra` | `#1c2230` |
| `--code-bg` | `#1a2030` |
| `--pre-bg` | `#0a0c10` |
| `--shadow-color` | `rgba(0,0,0,0.4)` |
| `--pre-ink` | `#d0ccc0` |

> Dark palette: blueish background from poly-canvas. Text is warm (less blue) for readability. Headers keep blue tones.

---

## 3. The Monochrome Palette

Neutral grays. Brightness slider adjusts lightness of all elements. Saturation slider has no effect (already grayscale).

### Light mode

| Variable | Value |
|---|---|
| `--paper` | `#f5f5f5` |
| `--ink` | `#333333` |
| `--ink-soft` | `#666666` |
| `--espresso` | `#222222` |
| `--accent` | `#555555` |
| `--accent-deep` | `#222222` |
| `--bronze` | `#777777` |
| `--blue` | `#555555` |
| `--brick` | `#555555` |
| `--line` | `#dddddd` |
| `--card` | `#fafafa` |
| `--zebra` | `#f0f0f0` |
| `--code-bg` | `#e8e8e8` |
| `--pre-bg` | `#333333` |
| `--shadow-color` | `rgba(0,0,0,0.08)` |
| `--pre-ink` | `#f0f0f0` |

### Dark mode

| Variable | Value |
|---|---|
| `--paper` | `#1a1a1a` |
| `--ink` | `#cccccc` |
| `--ink-soft` | `#999999` |
| `--espresso` | `#dddddd` |
| `--accent` | `#aaaaaa` |
| `--accent-deep` | `#e0e0e0` |
| `--bronze` | `#888888` |
| `--blue` | `#aaaaaa` |
| `--brick` | `#aaaaaa` |
| `--line` | `#444444` |
| `--card` | `#222222` |
| `--zebra` | `#2a2a2a` |
| `--code-bg` | `#333333` |
| `--pre-bg` | `#111111` |
| `--shadow-color` | `rgba(0,0,0,0.3)` |
| `--pre-ink` | `#cccccc` |

> Monochrome italics: `--accent-deep` is darker than `--ink` in light mode, lighter in dark mode, for more contrast with regular text.

---

## 4. The Solarized Palette

Drawn from the [Solarized](https://github.com/altercation/solarized) palette by Ethan Schoonover. Both light and dark modes available.

### Light mode

| Variable | Value | Solarized name |
|---|---|---|
| `--paper` | `#fdf6e3` | base3 |
| `--ink` | `#657b83` | base00 |
| `--ink-soft` | `#93a1a1` | base1 |
| `--espresso` | `#586e75` | base01 |
| `--accent` | `#b58900` | yellow |
| `--accent-deep` | `#cb4b16` | orange |
| `--bronze` | `#93a1a1` | base1 |
| `--blue` | `#268bd2` | |
| `--cyan` | `#2aa198` | |
| `--green` | `#859900` | |
| `--red` | `#dc322f` | |
| `--magenta` | `#d33682` | |
| `--violet` | `#6c71c4` | |
| `--line` | `#eee8d5` | base2 |
| `--card` | `#eee8d5` | base2 |
| `--zebra` | `#eee8d5` | base2 |
| `--code-bg` | `#eee8d5` | base2 |
| `--pre-bg` | `#eee8d5` | base2 |
| `--shadow-color` | `rgba(0,0,0,0.05)` | subtle shadows |
| `--pre-ink` | `#586e75` | base01 |

### Dark mode

| Variable | Value | Solarized name |
|---|---|---|
| `--paper` | `#002b36` | base03 |
| `--ink` | `#839496` | base0 |
| `--ink-soft` | `#586e75` | base01 |
| `--espresso` | `#93a1a1` | base1 |
| `--accent` | `#b58900` | yellow |
| `--accent-deep` | `#cb4b16` | orange |
| `--bronze` | `#586e75` | base01 |
| `--blue` | `#268bd2` | |
| `--cyan` | `#2aa198` | |
| `--green` | `#859900` | |
| `--red` | `#dc322f` | |
| `--magenta` | `#d33682` | |
| `--violet` | `#6c71c4` | |
| `--line` | `#073642` | base02 |
| `--card` | `#073642` | base02 |
| `--zebra` | `#073642` | base02 |
| `--code-bg` | `#073642` | base02 |
| `--pre-bg` | `#073642` | base02 |
| `--shadow-color` | `rgba(0,0,0,0.3)` | subtle shadows |
| `--pre-ink` | `#93a1a1` | base1 |

> Solarized inversion rule: Light/dark are exact inverses. Light: `base3` bg, `base00` text, `base01` headings, `base1` bronze. Dark: `base03` bg, `base0` text, `base1` headings, `base01` bronze.

---

## 5. Theme Engine

Colors are applied dynamically by `ThemeEngine.apply(palette, mode, brightness, saturation, font)`. The engine looks up base colors for the palette + mode, adjusts each color by brightness and saturation, then sets CSS custom properties on `:root`. It also sets `data-theme`, `data-palette`, and `data-font` attributes on `<html>`.

### Brightness adjustment

Each step = 8% brightness change. Range: -3 to +3.

```javascript
const factor = 1 + step * 0.08;
const adjusted = rgb.map(v => Math.max(0, Math.min(255, Math.round(v * factor))));
```

### Saturation adjustment

Each step = 15% saturation change. Range: -3 to +3. Converted via HSL.

```javascript
const [h, s, l] = rgbToHsl(r, g, b);
const newS = Math.max(0, Math.min(1, s * (1 + step * 0.15)));
const [newR, newG, newB] = hslToRgb(h, newS, l);
```

---

## 6. The Toolbar

Compact 24px fixed toolbar at the top of the page. Opaque `var(--paper)` background (no translucency, no blur).

### Controls

| Control | Type | Behavior |
|---|---|---|
| Mode toggle | 44x20px switch | Light/dark mode. Thumb slides 16px. Moon/sun SVG icons. |
| Font toggle | 44x20px switch | Serif/sans-serif. Thumb slides 16px. I/line SVG icons. |
| Palette selector | Button group | Regular / Mono / Solarized. Active button highlighted. |
| Brightness slider | Range input | -3 to +3, step 1. Value displayed. |
| Saturation slider | Range input | -3 to +3, step 1. Disabled in mono mode. |

### Font toggle behavior

- **ON (sans-serif):** Body and headings use `var(--font-ui)`
- **OFF (serif):** Body and headings use `var(--font-serif)`
- **Italic text (`em`):** Always stays serif with `font-family: var(--font-serif)`
- **Code blocks:** Always monospace (`var(--font-mono)`)

### Default font per palette

- Regular -> sans-serif
- Monochrome -> sans-serif
- Solarized -> serif

When a palette is selected, the font resets to that palette's default. The user can still toggle manually afterward.

---

## 7. OS-Aware Native Font Stacks

### 7.1 OS detection

A one-liner stamps `data-os` onto `<html>` so the CSS can pick font stacks native to that OS; no external fonts, no downloads, no CDN. Run it in the `<head>`, right after the stylesheet.

```js
(function(){
  var ua=navigator.userAgent;
  var os='unknown';
  if(/Android/i.test(ua))os='android';
  else if(/iPhone|iPad|iPod/i.test(ua))os='ios';
  else if(/Windows/i.test(ua))os='windows';
  else if(/Macintosh|Mac OS X/i.test(ua))os='mac';
  else if(/Linux/i.test(ua))os='linux';
  document.documentElement.dataset.os=os;
})();
```

### 7.2 The font sets

Three custom properties -- `--font-serif` (body), `--font-ui` (sans UI), `--font-mono` (code) -- defined per OS. The defaults live in `:root`; each `[data-os="..."]` block overrides only these three.

| OS | `--font-ui` | `--font-serif` | `--font-mono` |
|---|---|---|---|
| **windows** | `"Segoe UI Variable Text", "Segoe UI", sans-serif` | `Georgia, "Times New Roman", serif` | `"Cascadia Mono", "Cascadia Code", Consolas, monospace` |
| **mac** | `-apple-system, "SF Pro Text", "Helvetica Neue", sans-serif` | `"New York", "Times New Roman", Times, serif` | `"SF Mono", Menlo, Monaco, monospace` |
| **ios** | `-apple-system, "SF Pro Text", "Helvetica Neue", sans-serif` | `"New York", Georgia, "Times New Roman", serif` | `"SF Mono", Menlo, monospace` |
| **android** | `Roboto, "Noto Sans", sans-serif` | `"Noto Serif", serif` | `"Roboto Mono", "Droid Sans Mono", "Noto Sans Mono", monospace` |
| **linux** | `Ubuntu, "Cantarell", "Noto Sans", sans-serif` | `"Liberation Serif", "Noto Serif", serif` | `"Ubuntu Mono", "DejaVu Sans Mono", monospace` |
| **unknown** (default) | `"Segoe UI", Helvetica, Arial, sans-serif` | `Georgia, "Times New Roman", serif` | `"Cascadia Code", Consolas, monospace` |

---

## 8. The Converter

`foton-md-html-converter-x64` reads Markdown (`.md`) or plain text (`.txt`) and writes a styled, self-contained HTML page for each source, embedding the toolbar and theme engine from this spec. v6 (old 3-toggle system) and the Python v7 predecessor are obsolete.

### 8.1 Usage

```bash
foton-md-html-converter-x64 [--target <path>] [--kicker <text>]
```

All arguments are optional.

| Argument | Default | Meaning |
|---|---|---|
| `--target <path>` | *(current directory)* | A single `.md`/`.txt` file or a directory. Trailing slashes ignored. When absent, every `.md`/`.txt` in cwd is converted. |
| `--kicker <text>` | *(empty)* | Small uppercase label above the title on each page. Omitted if empty. |

### 8.2 Same-folder output rule

Every generated `.html` is written in the same directory as its source. There is no output-directory concept -- the script cannot be told to write elsewhere. Point it at a file or a folder, and the HTML appears next to the sources.

### 8.3 `--target` path rules

- **File** (e.g. `C:\docs\notes.md`) -> only that file is converted; its HTML lands next to it.
- **Folder** (e.g. `C:\docs\` or `c:\docs`) -> every `.md`/`.txt` inside is converted.
- **Nonexistent path** -> error and exit, nothing converted.
- **File that is not `.md`/`.txt`** -> error and exit.

### 8.4 Output features

- **Self-contained** -- each HTML includes all CSS and JS inline; no external dependencies.
- **Three palettes** -- Regular (default), Monochrome, and Solarized.
- **Toolbar** -- mode toggle, font toggle, palette selector, brightness/saturation sliders.
- **ThemeEngine** -- dynamic color adjustment via brightness and saturation sliders.
- **OS-aware fonts** -- Segoe UI on Windows, SF Pro on macOS/iOS, Roboto on Android, Ubuntu on Linux.
- **System preference** -- detects `prefers-color-scheme` on load.
- **Responsive** -- adapts below 620px viewport.

### 8.5 Markdown features

| Syntax | Output |
|---|---|
| `# ... ######` | Headings (h2 gets auto-numbered section counters) |
| `**bold**` / `*italic*` / `***both***` | `<strong>`, `<em>`, combined |
| `` `code` `` | Inline `<code>` |
| ` ``` lang ... ``` ` | Fenced code block with optional language class |
| `[text](url)` | Anchor link |
| `\| ... \| ... \|` | Table with header row and zebra striping |
| `- item` / `1. item` | Unordered / ordered list |
| `> quote` | Callout blockquote |
| `---` / `***` / `___` | Horizontal rule (consumed, not rendered) |
| `--- ... ---` at file start | YAML front matter (skipped silently) |
| ```` ```svg ... ``` ```` | ASCII diagram converted to DOM nodes + runtime SVG edges; if content starts with `<svg`, passed through raw |

### 8.6 Examples

```bash
# Convert every .md and .txt in the current directory
foton-md-html-converter-x64

# Convert a single file (HTML lands next to it)
foton-md-html-converter-x64 --target C:\docs\notes.md
foton-md-html-converter-x64 --target C:\docs\notes.txt

# Convert a whole folder
foton-md-html-converter-x64 --target C:\docs\
foton-md-html-converter-x64 --target c:\docs

# With a kicker label on every page
foton-md-html-converter-x64 --target C:\docs\ --kicker "Product Docs"
```

---

## Appendix A. Full toolbar markup (verbatim)

The complete `#tb` div with all controls and their exact SVG icon paths.

```html
<div id="tb">
  <div class="g">
    <div class="toggle" id="i-mode" tabindex="0" role="switch" aria-checked="false" aria-label="Toggle dark/light mode">
      <span class="toggle-thumb"></span>
      <svg class="ico-l" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#888" stroke-width="2" stroke-linecap="round">
        <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>
      </svg>
      <svg class="ico-r" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#888" stroke-width="2" stroke-linecap="round">
        <circle cx="12" cy="12" r="5"/>
        <line x1="12" y1="1" x2="12" y2="3"/><line x1="12" y1="21" x2="12" y2="23"/>
        <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"/><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"/>
        <line x1="1" y1="12" x2="3" y2="12"/><line x1="21" y1="12" x2="23" y2="12"/>
        <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"/><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"/>
      </svg>
    </div>
  </div>
  <div class="g">
    <div class="toggle on" id="i-font" tabindex="0" role="switch" aria-checked="true" aria-label="Toggle serif/sans-serif">
      <span class="toggle-thumb"></span>
      <svg class="ico-l" width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="#888" stroke-width="1.2">
        <line x1="3" y1="3" x2="11" y2="3"/>
        <line x1="7" y1="3" x2="7" y2="11"/>
        <line x1="3" y1="11" x2="11" y2="11"/>
      </svg>
      <svg class="ico-r" width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="#888" stroke-width="1.2">
        <line x1="7" y1="3" x2="7" y2="11"/>
      </svg>
    </div>
  </div>
  <div class="s"></div>
  <div class="g">
    <div class="pal-ctrl" id="i-palette">
      <button class="pal-btn active" data-palette="regular">Regular</button>
      <button class="pal-btn" data-palette="monochrome">Mono</button>
      <button class="pal-btn" data-palette="solarized">Solarized</button>
    </div>
  </div>
  <div class="s"></div>
  <div class="g">
    <svg class="ico-slider" viewBox="0 0 12 12" fill="none" stroke="#888" stroke-width="1">
      <rect x="1" y="1" width="10" height="10"/>
      <polygon points="1,1 11,1 1,11" fill="#888"/>
    </svg>
    <input type="range" id="i-bri" min="-3" max="3" step="1" value="0">
    <span class="v" id="v-bri">0</span>
  </div>
  <div class="s"></div>
  <div class="g" id="g-sat">
    <svg class="ico-slider" viewBox="0 0 12 12" fill="#888">
      <circle cx="6" cy="3" r="1.8"/>
      <circle cx="3.5" cy="7.5" r="1.8"/>
      <circle cx="8.5" cy="7.5" r="1.8"/>
    </svg>
    <input type="range" id="i-sat" min="-3" max="3" step="1" value="0">
    <span class="v" id="v-sat">0</span>
  </div>
</div>
```

## Appendix B. Full toolbar CSS (verbatim)

```css
#tb{
  position:fixed;top:0;left:0;right:0;height:24px;z-index:10;
  display:flex;align-items:center;gap:5px;padding:0 8px;justify-content:flex-end;
  background:var(--paper);transition:background .3s;
  border-bottom:1px solid rgba(255,255,255,0.06);
  font:10px/1 var(--font-ui);color:#888;
  user-select:none;
}
#tb.lt{border-bottom-color:rgba(0,0,0,0.08);color:#666}
#tb .g{display:flex;align-items:center;gap:3px}
#tb .s{width:1px;height:12px;background:rgba(255,255,255,0.08)}
#tb.lt .s{background:rgba(0,0,0,0.1)}
.toggle{position:relative;width:44px;height:20px;cursor:pointer;background:transparent}
.toggle-thumb{position:absolute;top:2px;left:6px;width:16px;height:16px;border-radius:2px;background:#2e2e2e;border:1px solid #777;transition:transform .3s cubic-bezier(.4,0,.2,1),background .3s ease}
#tb.lt .toggle-thumb{background:#cbcbcb;border-color:#888}
.toggle.on .toggle-thumb{transform:translateX(16px)}
.toggle svg{position:absolute;top:50%;transform:translateY(-50%);transition:opacity .3s ease;pointer-events:none;width:12px;height:12px}
.toggle .ico-l{left:8px;opacity:1}
.toggle .ico-r{right:8px;opacity:0.5}
.toggle.on .ico-l{opacity:0.5}
.toggle.on .ico-r{opacity:1}
.pal-ctrl{display:flex;border:1px solid rgba(255,255,255,0.15);border-radius:2px;overflow:hidden}
#tb.lt .pal-ctrl{border-color:rgba(0,0,0,0.15)}
.pal-btn{
  height:18px;padding:0 5px;background:transparent;color:inherit;
  border:none;border-right:1px solid rgba(255,255,255,0.15);cursor:pointer;
  font:8px/1 var(--font-ui);text-transform:uppercase;
  letter-spacing:0.4px;transition:background .2s ease,color .2s ease;
}
.pal-btn:last-child{border-right:none}
.pal-btn.active{background:#2e2e2e;color:#ccc}
#tb.lt .pal-btn.active{background:#cbcbcb;color:#333}
.ico-slider{width:10px;height:10px;flex-shrink:0}
#tb input[type=range]{-webkit-appearance:none;appearance:none;width:48px;height:3px;background:#4e4e4e;border-radius:2px;outline:none;cursor:pointer;border:1px solid rgba(255,255,255,0.12)}
#tb input[type=range]::-webkit-slider-thumb{-webkit-appearance:none;appearance:none;width:10px;height:10px;background:#2e2e2e;border:1px solid #777;border-radius:2px;cursor:pointer}
#tb input[type=range]::-moz-range-thumb{width:10px;height:10px;background:#2e2e2e;border:none;border-radius:2px;cursor:pointer}
#tb.lt input[type=range]{background:#cbcbcb;border-color:rgba(0,0,0,0.15)}
#tb.lt input[type=range]::-webkit-slider-thumb{background:#cbcbcb}
#tb.lt input[type=range]::-moz-range-thumb{background:#cbcbcb}
#tb .v{min-width:14px;text-align:center;font-variant-numeric:tabular-nums;font-size:10px}
#g-sat.disabled{opacity:0.3;pointer-events:none}
#g-sat.disabled .ico-slider{opacity:0.5}
```

## Appendix C. Init script (verbatim)

System preference detection + MenuBar/ThemeEngine wiring. Runs at end of `<body>`.

```js
(function(){
  var prefersDark=window.matchMedia&&window.matchMedia('(prefers-color-scheme: dark)').matches;
  var initialMode=prefersDark?'dark':'light';
  MenuBar.init({
    mode:initialMode,
    palette:'regular',
    onChange:function(opts){
      ThemeEngine.apply(opts.palette,opts.mode,opts.brightness,opts.saturation,opts.font);
    }
  });
  var initial=MenuBar.getValues();
  ThemeEngine.apply(initial.palette,initial.mode,initial.brightness,initial.saturation,initial.font);
})();
```

## Appendix D. Page chrome & content CSS (readable summary)

```css
header { border-top: 6px solid var(--accent); padding-top: 28px; margin-bottom: 8px; }
.kicker { font-family: var(--font-ui); font-size: 12px; letter-spacing: .22em;
  text-transform: uppercase; color: var(--bronze); margin: 0 0 14px; }
h1 { font-size: 38px; font-weight: 400; line-height: 1.2; color: var(--espresso); }
.lede { font-size: 17px; color: var(--ink-soft); font-style: italic; }
h2 { font-size: 26px; font-weight: 400; color: var(--espresso); margin: 56px 0 6px; }
h2 .num { font-family: var(--font-ui); font-size: 14px; color: var(--accent-deep); vertical-align: super; margin-right: 8px; }
.rule { height: 1px; background: var(--line); margin: 0 0 22px; }

table { width: 100%; border-collapse: collapse; margin: 20px 0 28px;
  background: var(--card); border: 1px solid var(--line); }
th { font-family: var(--font-ui); font-size: 12.5px; letter-spacing: .04em;
  text-transform: uppercase; color: var(--card); background: var(--espresso);
  border-bottom: 2px solid var(--accent); }
td { font-size: 14.5px; border-bottom: 1px solid var(--line); }
tr:nth-child(even) td { background: var(--zebra); }

.callout { border-left: 4px solid var(--accent); background: var(--card);
  padding: 16px 20px; margin: 22px 0; font-style: italic; color: var(--ink-soft); }

.svg-diagram { display: block; text-align: center; margin: 24px 0;
  padding: 20px; background: var(--card); border: 1px solid var(--line); border-radius: 4px; }
.svg-diagram svg { max-width: 100%; height: auto; }

code { font-family: var(--font-mono); font-size: 13px; background: var(--code-bg);
  color: var(--espresso); padding: 1px 6px; border-radius: 3px; }
pre { background: var(--pre-bg); color: var(--pre-ink); padding: 16px 20px;
  border-radius: 4px; overflow-x: auto; font-family: var(--font-mono); font-size: 13px; }
pre code { background: none; color: inherit; padding: 0; }
```
