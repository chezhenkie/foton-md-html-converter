# Local WSL Toolchain Test Procedure

This document describes how to verify that the local WSL2 Ubuntu toolchain works from any reinitialized local repo. The procedure was validated on `html-md-txt-stylespec-converter-proj`; adapt the path and project name for other repos.

## 1. Prerequisites

All Rust/Go/Node/Python toolchains live inside WSL2 Ubuntu 26.04 at `/home/user1linux/`. The Windows side has no toolchains. Builds and tests must run inside WSL.

- WSL2 distro: `Ubuntu`
- Rust installed at `/home/user1linux/.cargo/`
- Toolchain env file: `/home/user1linux/.cargo/env`
- MinGW cross-linker for Windows GNU target: `x86_64-w64-mingw32-gcc`
- MinGW resource compiler (when VERSIONINFO is used): `x86_64-w64-mingw32-windres`

## 2. Basic WSL sanity check

From the Windows project folder, open a WSL bash shell and source the env:

```powershell
wsl -d Ubuntu -- bash -c 'source /home/user1linux/.cargo/env && which cargo rustc && cargo --version && rustup target list --installed && which x86_64-w64-mingw32-gcc'
```

Expected output includes:

- `/home/user1linux/.cargo/bin/cargo`
- `/home/user1linux/.cargo/bin/rustc`
- `cargo 1.98.0 ...`
- `x86_64-pc-windows-gnu`
- `x86_64-unknown-linux-musl`
- `/usr/bin/x86_64-w64-mingw32-gcc`

## 3. Project-specific toolchain check

If the crate shells out to C++ build tools (for example `transcribe-cpp` or `cpal`), also verify:

```powershell
wsl -d Ubuntu -- bash -c 'which cmake g++'
```

Pure Rust crates (like this converter) do not need cmake/g++.

## 4. Run the standard quality gate

Replace the `/mnt/c/...` path with the actual repo path.

```powershell
wsl -d Ubuntu -- bash -c 'source /home/user1linux/.cargo/env && cd /mnt/c/Users/lap1user/ocdev/html-specsheet-converters-dir/html-md-txt-stylespec-converter-proj && cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test'
```

This single command runs, in order:

1. `cargo fmt --check` — formatting must match rustfmt.
2. `cargo clippy --all-targets -- -D warnings` — clippy must be warning-free.
3. `cargo test` — all unit tests must pass.

If any step fails, fix it before proceeding.

## 5. Build the release targets

### Windows GNU binary

```powershell
wsl -d Ubuntu -- bash -c 'source /home/user1linux/.cargo/env && cd /mnt/c/Users/lap1user/ocdev/html-specsheet-converters-dir/html-md-txt-stylespec-converter-proj && cargo build --release --target x86_64-pc-windows-gnu'
```

Output:

```
target/x86_64-pc-windows-gnu/release/foton-md-html-converter-x64.exe
```

### Linux musl static binary

```powershell
wsl -d Ubuntu -- bash -c 'source /home/user1linux/.cargo/env && cd /mnt/c/Users/lap1user/ocdev/html-specsheet-converters-dir/html-md-txt-stylespec-converter-proj && cargo build --release --target x86_64-unknown-linux-musl'
```

Output:

```
target/x86_64-unknown-linux-musl/release/foton-md-html-converter-x64
```

## 6. Verify the binaries

### Windows exe

From Windows PowerShell, confirm the file exists and run `--help`:

```powershell
& "C:\Users\lap1user\ocdev\html-specsheet-converters-dir\html-md-txt-stylespec-converter-proj\target\x86_64-pc-windows-gnu\release\foton-md-html-converter-x64.exe" --help
```

If the crate embeds VERSIONINFO, check the Details tab in File Explorer.

### Linux binary

From WSL, check that it is statically linked and runs:

```powershell
wsl -d Ubuntu -- bash -c 'file /mnt/c/Users/lap1user/ocdev/html-specsheet-converters-dir/html-md-txt-stylespec-converter-proj/target/x86_64-unknown-linux-musl/release/foton-md-html-converter-x64 && /mnt/c/Users/lap1user/ocdev/html-specsheet-converters-dir/html-md-txt-stylespec-converter-proj/target/x86_64-unknown-linux-musl/release/foton-md-html-converter-x64 --help'
```

Expected `file` output includes `statically linked`.

## 7. Functional smoke test

Create a small markdown file and convert it:

```powershell
$testDir = "C:\Users\lap1user\ocdev\html-specsheet-converters-dir\html-md-txt-stylespec-converter-proj\_test-smoke"
New-Item -ItemType Directory -Path $testDir -Force
"# Smoke Test`n`nThis is a paragraph." | Set-Content -LiteralPath "$testDir\smoke.md" -Encoding UTF8
& "C:\Users\lap1user\ocdev\html-specsheet-converters-dir\html-md-txt-stylespec-converter-proj\target\x86_64-pc-windows-gnu\release\foton-md-html-converter-x64.exe" --target "$testDir\smoke.md"
Test-Path -LiteralPath "$testDir\smoke.html"
```

Open `smoke.html` in a browser and confirm:

- Toolbar appears at the top.
- Light/dark mode toggle works.
- Font toggle works.
- Palette selector switches Regular / Mono / Solarized.
- Brightness and saturation sliders adjust colors.
- The page uses OS-aware fonts.

Clean up the smoke directory when done:

```powershell
Remove-Item -Recurse -Force -LiteralPath $testDir
```

## 8. Adapt for other repos

For any other local repo after reinitialization:

1. Replace the project path in the commands above.
2. Check the repo's `Cargo.toml` for the binary name and any special targets.
3. If the project has a `.github/workflows/build.yml`, run the same commands it uses (usually `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, and `cargo build --release`).
4. If the repo contains C++ dependencies, add the `which cmake g++` check.

## 9. Common issues

| Symptom | Cause | Fix |
|---|---|---|
| `cargo: command not found` | Env not sourced | Always run `source /home/user1linux/.cargo/env` first. |
| `linker x86_64-w64-mingw32-gcc not found` | MinGW not installed in WSL | Install `mingw-w64` in Ubuntu. |
| `rustup target list` missing `x86_64-pc-windows-gnu` | Target not installed | `rustup target add x86_64-pc-windows-gnu`. |
| Clippy warnings on pre-existing code | Code not cleaned for clippy | Fix the warnings; do not suppress with `#[allow(...)]` unless the project explicitly chooses that. |
| `cargo fmt --check` fails | Files not formatted | Run `cargo fmt` and commit the formatting changes. |

## 10. Quick reference one-liner

```powershell
wsl -d Ubuntu -- bash -c 'source /home/user1linux/.cargo/env && cd /mnt/c/Users/lap1user/ocdev/html-specsheet-converters-dir/html-md-txt-stylespec-converter-proj && cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test && cargo build --release --target x86_64-pc-windows-gnu && cargo build --release --target x86_64-unknown-linux-musl'
```

This is the complete local toolchain verification in one command.
