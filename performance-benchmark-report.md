# foton-md-html-converter-x64.exe Performance Report

**Date:** 2026-08-26
**Executable:** foton-md-html-converter-x64.exe
**Platform:** Windows (binary cross-compiled from WSL via MinGW)
**Methodology:** Synthetic markdown files generated per configuration; conversion run once per configuration; source and output files removed after each run.

## Root cause of previous slowness (fixed)

The Rust parser was recompiling every regular expression on every call to `inline()` and on every call to `md_to_html()`. `inline()` runs once per paragraph/line/table-cell/heading/blockquote, so a single file triggered thousands of regex compilations.

Python's `re` module caches compiled patterns automatically, so the original Python script paid the compilation cost only once. Rust's `regex` crate does not cache by default.

Fix: moved all regexes to `std::sync::LazyLock` statics so they compile once per process. Also replaced the three-pass `escape_html` with a single-pass loop.

## Raw Results (after fix)

| File Size | Count | Source Total | Conv Time | Files/sec | Per-file |
|-----------|-------|--------------|-----------|-----------|----------|
| small (5120 B) | 100 | 0.50 MB | 0.249 s | 401.6 | 2.49 ms |
| small (5120 B) | 1000 | 5.03 MB | 1.136 s | 880.2 | 1.14 ms |
| medium (51200 B) | 100 | 4.94 MB | 0.463 s | 216.0 | 4.63 ms |
| large (512000 B) | 10 | 4.85 MB | 0.086 s | 116.3 | 8.60 ms |

## Comparison with previous report

| File Size | Count | Before (ms/file) | After (ms/file) | Speedup |
|-----------|-------|------------------|-----------------|---------|
| small | 100 | 18.18 | 2.49 | 7.3x |
| small | 1000 | 16.86 | 1.14 | 14.8x |
| medium | 100 | 150.15 | 4.63 | 32.4x |
| large | 10 | 1469.58 | 8.60 | 170.9x |

## Observations

- Regex compilation dominated runtime; removing it brought per-file times down to single-digit milliseconds.
- Throughput now scales linearly with input size.
- Large files no longer exhibit the disproportionate slowdown seen before.
