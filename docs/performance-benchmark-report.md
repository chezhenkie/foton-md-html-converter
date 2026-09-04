# foton-md-html-converter-x64.exe Performance Report

**Date:** 2026-09-04
**Executable:** foton-md-html-converter-x64.exe (Windows GNU build)
**Platform:** Windows, local NTFS, binary and test corpus both on the native filesystem
**Method:** PowerShell `System.Diagnostics.Stopwatch`, directory-conversion mode, 1 warm-up run followed by 5 timed runs per configuration, outputs deleted between runs. Min/Avg/Max reported.
**Test corpus:** Synthetic markdown (headings, paragraphs, inline styling, links, ordered and unordered lists, tables, fenced code, callouts) generated to fixed size targets; the identical corpus was converted by both binaries.

This report follows the SPEEDTEST-METHODOLOGY rules: both binaries ran natively on Windows against files on local NTFS, no mount protocols crossed, warm-up runs performed before timing, and min/avg/max reported rather than a single sample.

## History: regex compilation root cause (fixed 2026-08-26)

The parser originally recompiled every regular expression on every call to `inline()` and `md_to_html()`. `inline()` runs once per paragraph, line, table cell, heading, and blockquote, so a single file triggered thousands of regex compilations. Python's `re` module caches compiled patterns automatically, so the original Python script paid the compilation cost only once; Rust's `regex` crate does not cache by default. Moving all regexes to `std::sync::LazyLock` statics and replacing the three-pass `escape_html` with a single-pass loop brought per-file times from double-digit milliseconds down to single digits. See the 2026-08-26 edition of this report for those numbers.

## Structural pass (2026-09-04): old build vs new build

A second pass attacked the remaining structural allocation costs. Three changes were made. First, the HTML page shell (about 21 KB of embedded CSS and JS) is now assembled once per process into constant segments instead of being re-scanned and re-copied by `format!` for every converted file. Second, `md_to_html` writes into a single pre-sized `String` buffer instead of building a `Vec<String>` of per-element `format!` allocations and then duplicating the whole document with a final `join`. Third, the two duplicated code-block flush sites were unified into one helper, and code fence buffers now borrow lines instead of copying them.

Both binaries were built from the same commit base with the identical toolchain and flags; the old binary is HEAD (87a89c0), the new binary additionally contains the structural pass. Results, average of 5 runs:

| File Size | Count | Source Total | Old avg | New avg | Old per-file | New per-file | Speedup |
|-----------|-------|--------------|---------|---------|--------------|--------------|---------|
| small (5.1 KB) | 100 | 0.51 MB | 66.1 ms | 53.8 ms | 0.661 ms | 0.538 ms | 1.23x |
| small (5.1 KB) | 1000 | 5.10 MB | 544.6 ms | 456.6 ms | 0.545 ms | 0.457 ms | 1.19x |
| medium (51.2 KB) | 100 | 4.90 MB | 246.2 ms | 199.9 ms | 2.462 ms | 1.999 ms | 1.23x |
| large (512 KB) | 10 | 4.88 MB | 197.5 ms | 166.3 ms | 19.755 ms | 16.632 ms | 1.19x |

Min/max spread across the 5 timed runs was narrow for both binaries (for example, small-1000: old 530.2-556.0 ms, new 436.3-473.2 ms), so the averages are stable. The speedup is uniform at roughly 1.2x across all four configurations, which is consistent with the changes removing a fixed per-page cost (shell re-copy) and a proportional per-document cost (body double-write): the gain scales with page count and page size rather than with any one file shape.

## Comparison with the 2026-08-26 report

The previous edition reported, for the same size classes, single-run per-file times of 2.49 ms (small x100), 1.14 ms (small x1000), 4.63 ms (medium x100), and 8.60 ms (large x10). This report's corpus differs from the one used then (different generator, denser tables and code fences per kilobyte), and the previous numbers were single samples without warm-up, so the two runs are not directly comparable and no speedup is claimed across them. Within this report both binaries processed the identical corpus under identical conditions, which makes the old-versus-new table above the valid comparison. The structural gain is independent of and additive to the earlier regex fix, which is already contained in both binaries measured here.

## Observations

- The page-shell pre-assembly and the single-buffer parser together cut conversion time by about 20 percent at every scale tested.
- Output bytes are identical between the old and new builds; the gain is purely allocation and copy reduction, verified by a hash comparison of converted pages.
- Throughput continues to scale linearly with input size, with no disproportionate slowdown for large files.
- Process spawn cost is amortized inside directory mode and is negligible per file at all tested scales.
