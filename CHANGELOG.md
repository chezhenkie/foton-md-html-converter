# Changelog

All notable changes to this project are documented in this file. The format follows Keep a Changelog and the project uses semantic versioning.

## [1.6.0]

### Added

- A back button at the far left of the toolbar, greyed out when the tab has no earlier page.

### Changed

- Table rows share one background: the zebra striping on the page fill is gone, so a table reads as one card surface.

### Fixed

- Browser Back and a reload keep the user's mode and both theme names: a user change rewrites the current history entry instead of leaving the URL without a query.
- The back button greys out on a local page with no earlier entry in Firefox, where each `file://` page is a separate origin for the Navigation API. Local links carry a `via` marker so a page opened from another page stays active.

## [1.5.0]

### Added

- URL theme state: a `?mode=dark|light&dark=<name>&light=<name>` query opens the page in that mode and theme, suppresses OS following, and is carried into every local `.html` link. The names travel, not the colors; each page resolves them through its own table.
- Ten demo screenshots in `docs/`, one per theme and one of the print state, captured from the current page.
- Both release binaries sit in `dist/`.

### Changed

- Rust code and build inputs live under `src/`: `src/build.rs`, `src/icon.ico`, and `src/target` for build output through `[build] target-dir`. The README, expanded manual and CI artifact paths follow.

### Fixed

- The repository About text and the release notes described three palettes and named the wrong build commit; they now name the nine themes and 53dbc66.

## [1.4.0]

First GitHub release.

### Added

- Nine six-value themes, five dark (github, harbor, solarized, oledblack, ayu) and four light (vercel, tokyonight, cursor, orng), in two keyboard-navigable menus.
- A sans/serif font switch for body and headings.
- A print state that aliases a fixed white palette on screen and in the print stylesheet.
- Brightness and saturation sliders, seven steps each, applied to the theme colours.
- The first `#` heading in a document titles the page and becomes the header, and is removed from the body; the file name stays the tab fallback.
- The page follows the OS colour preference on load until a manual mode or theme pick.

### Changed

- The toolbar is rebuilt from the simple-diagram template with a font switch added, replacing the three palettes.
- Light themes share one near-white paper; the accent, the card tint and the serif italic lede carry the theme.
- Tables, table headers, callouts and code blocks use the card surface, with zebra rows on the page fill.
