# HTML Style CSS Specsheet

Three palettes (Regular, Monochrome, Solarized), compact toolbar, OS-native fonts. Toolbar: right-aligned, fixed 24px.

---

## 1. Regular Palette

Warm paper, amber accents. Light in `:root`; dark via ThemeEngine.

| Variable | Light | Dark | Use |
|---|---|---|---|
| `--paper` | `#f4f3ef` | `#0a0c10` | page background |
| `--ink` | `#2b2b2a` | `#d0ccc0` | primary text |
| `--ink-soft` | `#575756` | `#9a9688` | secondary text |
| `--espresso` | `#383837` | `#a8b4c8` | headings, table header |
| `--accent` | `#b08a2e` | `#4a6d8c` | accents, header rule |
| `--accent-deep` | `#8a6a24` | `#5a7d9c` | emphasis |
| `--bronze` | `#7a746b` | `#7a8a9c` | kicker, palette labels |
| `--blue` | `#4a6d8c` | `#4a6d8c` | accent |
| `--brick` | `#a84a3a` | `#8c6a6a` | accent |
| `--line` | `#d9d8d3` | `#1a2030` | hairlines, borders |
| `--card` | `#fafaf8` | `#141820` | surfaces, callouts |
| `--zebra` | `#f7f7f4` | `#1c2230` | even table rows |
| `--code-bg` | `#eceae4` | `#1a2030` | inline code |
| `--pre-bg` | `#343433` | `#0a0c10` | code blocks |
| `--shadow-color` | `rgba(45,45,45,0.06)` | `rgba(0,0,0,0.4)` | subtle shadows |
| `--pre-ink` | `#f2f2ef` | `#d0ccc0` | code block text |

> Dark: blueish background from poly-canvas. Text warm for readability. Headers keep blue tones.

---

## 2. Monochrome Palette

Neutral grays. Brightness adjusts all. Saturation disabled (already grayscale).

| Variable | Light | Dark |
|---|---|---|
| `--paper` | `#f5f5f5` | `#1a1a1a` |
| `--ink` | `#333333` | `#cccccc` |
| `--ink-soft` | `#666666` | `#999999` |
| `--espresso` | `#222222` | `#dddddd` |
| `--accent` | `#555555` | `#aaaaaa` |
| `--accent-deep` | `#222222` | `#e0e0e0` |
| `--bronze` | `#777777` | `#888888` |
| `--blue` | `#555555` | `#aaaaaa` |
| `--brick` | `#555555` | `#aaaaaa` |
| `--line` | `#dddddd` | `#444444` |
| `--card` | `#fafafa` | `#222222` |
| `--zebra` | `#f0f0f0` | `#2a2a2a` |
| `--code-bg` | `#e8e8e8` | `#333333` |
| `--pre-bg` | `#333333` | `#111111` |
| `--shadow-color` | `rgba(0,0,0,0.08)` | `rgba(0,0,0,0.3)` |
| `--pre-ink` | `#f0f0f0` | `#cccccc` |

> `--accent-deep`: darker than `--ink` in light mode, lighter in dark mode.

---

## 3. Solarized Palette

From [Solarized](https://github.com/altercation/solarized) by Ethan Schoonover.

| Variable | Light | Dark | Solarized |
|---|---|---|---|
| `--paper` | `#fdf6e3` | `#002b36` | base3/base03 |
| `--ink` | `#657b83` | `#839496` | base00/base0 |
| `--ink-soft` | `#93a1a1` | `#586e75` | base1/base01 |
| `--espresso` | `#586e75` | `#93a1a1` | base01/base1 |
| `--accent` | `#b58900` | `#b58900` | yellow |
| `--accent-deep` | `#cb4b16` | `#cb4b16` | orange |
| `--bronze` | `#93a1a1` | `#586e75` | base1/base01 |
| `--blue` | `#268bd2` | `#268bd2` | |
| `--cyan` | `#2aa198` | `#2aa198` | |
| `--green` | `#859900` | `#859900` | |
| `--red` | `#dc322f` | `#dc322f` | |
| `--magenta` | `#d33682` | `#d33682` | |
| `--violet` | `#6c71c4` | `#6c71c4` | |
| `--line` | `#eee8d5` | `#073642` | base2/base02 |
| `--card` | `#eee8d5` | `#073642` | base2/base02 |
| `--zebra` | `#eee8d5` | `#073642` | base2/base02 |
| `--code-bg` | `#eee8d5` | `#073642` | base2/base02 |
| `--pre-bg` | `#eee8d5` | `#073642` | base2/base02 |
| `--shadow-color` | `rgba(0,0,0,0.05)` | `rgba(0,0,0,0.3)` | |
| `--pre-ink` | `#586e75` | `#93a1a1` | base01/base1 |

> Inversion: Light `base3`/`base00`/`base01`/`base1` maps to Dark `base03`/`base0`/`base1`/`base01`.

---

## 4. Theme Engine

`ThemeEngine.apply(palette, mode, brightness, saturation, font)` sets CSS custom properties on `:root` and `data-theme`/`data-palette`/`data-font` on `<html>`.

- **Brightness:** 8% per step, range -3..+3. `factor = 1 + step * 0.08; v = clamp(round(v * factor), 0, 255)`
- **Saturation:** 15% per step, range -3..+3. Via HSL: `newS = clamp(s * (1 + step * 0.15), 0, 1)`. Disabled in mono.

---

## 5. Toolbar

Fixed 24px, right-aligned, opaque `var(--paper)` background.

| Control | Type | Behavior |
|---|---|---|
| Mode toggle | 44x20px switch | Light/dark. Thumb slides 16px. Moon/sun SVG icons. |
| Font toggle | 44x20px switch | Serif/sans-serif. Thumb slides 16px. I/line SVG icons. |
| Palette selector | Button group | Regular / Mono / Solarized. Active highlighted. |
| Brightness slider | Range input | -3..+3, step 1. Value displayed. |
| Saturation slider | Range input | -3..+3, step 1. Disabled in mono. |

Font toggle: ON = `var(--font-ui)`, OFF = `var(--font-serif)`. Italic `em` always serif. Code blocks always monospace. Default font per palette: Regular=Mono=sans, Solarized=serif. Palette selection resets font.

---

## 6. OS-Aware Font Stacks

`data-os` stamped on `<html>` via inline `<head>` script. Three properties per OS:

| OS | `--font-ui` | `--font-serif` | `--font-mono` |
|---|---|---|---|
| **windows** | `"Segoe UI Variable Text", "Segoe UI", sans-serif` | `Georgia, "Times New Roman", serif` | `"Cascadia Mono", "Cascadia Code", Consolas, monospace` |
| **mac** | `-apple-system, "SF Pro Text", "Helvetica Neue", sans-serif` | `"New York", "Times New Roman", Times, serif` | `"SF Mono", Menlo, Monaco, monospace` |
| **ios** | `-apple-system, "SF Pro Text", "Helvetica Neue", sans-serif` | `"New York", Georgia, "Times New Roman", serif` | `"SF Mono", Menlo, monospace` |
| **android** | `Roboto, "Noto Sans", sans-serif` | `"Noto Serif", serif` | `"Roboto Mono", "Droid Sans Mono", "Noto Sans Mono", monospace` |
| **linux** | `Ubuntu, "Cantarell", "Noto Sans", sans-serif` | `"Liberation Serif", "Noto Serif", serif` | `"Ubuntu Mono", "DejaVu Sans Mono", monospace` |
| **unknown** | `"Segoe UI", Helvetica, Arial, sans-serif` | `Georgia, "Times New Roman", serif` | `"Cascadia Code", Consolas, monospace` |

---

## 7. The Converter

`foton-md-html-converter-x64` reads `.md`/`.txt`, writes self-contained HTML.

```bash
foton-md-html-converter-x64 [--target <path>] [--kicker <text>] [PATH...]
```

| Argument | Default | Meaning |
|---|---|---|
| `--target <path>` | cwd | Single `.md`/`.txt` file or directory. |
| `--kicker <text>` | empty | Small uppercase label above title. |
| `PATH...` | none | Bare paths (drag-and-drop). Mutually exclusive with `--target`. |

**Output rules:** HTML next to source (no output-directory). `--target` file=folder=nonexistent/wrong-ext rules apply. Drag-and-drop: items processed in order; bad items skipped; exit 1 if any failed. Self-contained: all CSS/JS inline, three palettes, toolbar, OS-aware fonts, system preference detection, responsive (620px).

**Markdown:** headings (h2 auto-numbered), bold/italic/code inline, links, fenced code blocks, tables (header+zebra), lists, callout blockquotes, horizontal rules (consumed), YAML front matter (skipped).

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
        <line x1="3" y1="3" x2="11" y2="3"/><line x1="7" y1="3" x2="7" y2="11"/><line x1="3" y1="11" x2="11" y2="11"/>
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
      <rect x="1" y="1" width="10" height="10"/><polygon points="1,1 11,1 1,11" fill="#888"/>
    </svg>
    <input type="range" id="i-bri" min="-3" max="3" step="1" value="0">
    <span class="v" id="v-bri">0</span>
  </div>
  <div class="s"></div>
  <div class="g" id="g-sat">
    <svg class="ico-slider" viewBox="0 0 12 12" fill="#888">
      <circle cx="6" cy="3" r="1.8"/><circle cx="3.5" cy="7.5" r="1.8"/><circle cx="8.5" cy="7.5" r="1.8"/>
    </svg>
    <input type="range" id="i-sat" min="-3" max="3" step="1" value="0">
    <span class="v" id="v-sat">0</span>
  </div>
</div>
```

## Appendix B. Toolbar CSS (verbatim)

```css
#tb{position:fixed;top:0;left:0;right:0;height:24px;z-index:10;display:flex;align-items:center;gap:5px;padding:0 8px;justify-content:flex-end;background:var(--paper);transition:background .3s;border-bottom:1px solid rgba(255,255,255,0.06);font:10px/1 var(--font-ui);color:#888;user-select:none}
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
.pal-btn{height:18px;padding:0 5px;background:transparent;color:inherit;border:none;border-right:1px solid rgba(255,255,255,0.15);cursor:pointer;font:8px/1 var(--font-ui);text-transform:uppercase;letter-spacing:0.4px;transition:background .2s ease,color .2s ease}
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

```js
(function(){
  var prefersDark=window.matchMedia&&window.matchMedia('(prefers-color-scheme: dark)').matches;
  var initialMode=prefersDark?'dark':'light';
  MenuBar.init({mode:initialMode,palette:'regular',onChange:function(opts){
    ThemeEngine.apply(opts.palette,opts.mode,opts.brightness,opts.saturation,opts.font);
  }});
  var initial=MenuBar.getValues();
  ThemeEngine.apply(initial.palette,initial.mode,initial.brightness,initial.saturation,initial.font);
})();
```

## Appendix D. Page chrome & content CSS

```css
header{border-top:6px solid var(--accent);padding-top:28px;margin-bottom:8px}
.kicker{font-family:var(--font-ui);font-size:12px;letter-spacing:.22em;text-transform:uppercase;color:var(--bronze);margin:0 0 14px}
h1{font-size:38px;font-weight:400;line-height:1.2;color:var(--espresso)}
.lede{font-size:17px;color:var(--ink-soft);font-style:italic}
h2{font-size:26px;font-weight:400;color:var(--espresso);margin:56px 0 6px}
h2 .num{font-family:var(--font-ui);font-size:14px;color:var(--accent-deep);vertical-align:super;margin-right:8px}
.rule{height:1px;background:var(--line);margin:0 0 22px}
table{width:100%;border-collapse:collapse;margin:20px 0 28px;background:var(--card);border:1px solid var(--line)}
th{font-family:var(--font-ui);font-size:12.5px;letter-spacing:.04em;text-transform:uppercase;color:var(--card);background:var(--espresso);border-bottom:2px solid var(--accent)}
td{font-size:14.5px;border-bottom:1px solid var(--line)}
tr:nth-child(even) td{background:var(--zebra)}
.callout{border-left:4px solid var(--accent);background:var(--card);padding:16px 20px;margin:22px 0;font-style:italic;color:var(--ink-soft)}
code{font-family:var(--font-mono);font-size:13px;background:var(--code-bg);color:var(--espresso);padding:1px 6px;border-radius:3px}
pre{background:var(--pre-bg);color:var(--pre-ink);padding:16px 20px;border-radius:4px;overflow-x:auto;font-family:var(--font-mono);font-size:13px}
pre code{background:none;color:inherit;padding:0}
```
