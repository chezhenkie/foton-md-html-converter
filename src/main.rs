#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

use clap::Parser;
use std::path::{Path, PathBuf};
use std::process;

#[derive(Parser)]
#[command(
    name = "foton-md-html-converter-x64",
    about = "Convert markdown to styled HTML",
    long_about = "HTML always lands in the same directory as its source .md file."
)]
struct Args {
    #[arg(
        long,
        help = "Path to a single .md/.txt file OR a directory to convert. A directory (trailing slash optional) converts every .md/.txt inside it. Default: convert every .md/.txt in the current working directory."
    )]
    target: Option<PathBuf>,

    #[arg(long, help = "Kicker text shown above the title on each page")]
    kicker: Option<String>,

    /// Files and/or folders to convert (used by drag-and-drop onto the exe)
    #[arg(
        value_name = "PATH",
        help = "Files and/or folders to convert (drag-and-drop passes plain paths). Validated for .md/.txt; directories are scanned like --target."
    )]
    files: Vec<PathBuf>,
}

fn convert_directory(dir: &Path, kicker: Option<&str>) {
    let md_files = match foton_md_html_converter::io::files::collect_source_files(dir) {
        Ok(files) => files,
        Err(_) => process::exit(1),
    };
    if md_files.is_empty() {
        process::exit(1);
    }
    for md_file in md_files {
        if foton_md_html_converter::io::convert::convert_file(&md_file, kicker).is_err() {
            process::exit(1);
        }
    }
}

fn convert_paths(paths: &[PathBuf], kicker: Option<&str>) {
    let mut failed = 0usize;
    for raw in paths {
        let trimmed = raw
            .to_string_lossy()
            .trim_end_matches(['/', '\\'])
            .to_string();
        let path = PathBuf::from(&trimmed);

        if path.is_dir() {
            match foton_md_html_converter::io::files::collect_source_files(&path) {
                Ok(files) if !files.is_empty() => {
                    for md_file in files {
                        if foton_md_html_converter::io::convert::convert_file(&md_file, kicker)
                            .is_err()
                        {
                            failed += 1;
                        }
                    }
                }
                Ok(_) | Err(_) => failed += 1,
            }
        } else if path.is_file() {
            let ext = path
                .extension()
                .and_then(|e| e.to_str())
                .map(|s| s.to_lowercase())
                .unwrap_or_default();
            if ext != "md" && ext != "txt" {
                failed += 1;
                continue;
            }
            if foton_md_html_converter::io::convert::convert_file(&path, kicker).is_err() {
                failed += 1;
            }
        } else {
            failed += 1;
        }
    }

    if failed > 0 {
        process::exit(1);
    }
}

fn main() {
    let args = Args::parse();

    let kicker = args.kicker.as_deref();

    if !args.files.is_empty() {
        if args.target.is_some() {
            process::exit(1);
        }
        convert_paths(&args.files, kicker);
        return;
    }

    match args.target {
        Some(target) => {
            let trimmed = target
                .to_string_lossy()
                .trim_end_matches(['/', '\\'])
                .to_string();
            let target = PathBuf::from(&trimmed);

            if target.is_dir() {
                convert_directory(&target, kicker);
            } else if target.is_file() {
                let ext = target
                    .extension()
                    .and_then(|e| e.to_str())
                    .map(|s| s.to_lowercase())
                    .unwrap_or_default();
                if ext != "md" && ext != "txt" {
                    process::exit(1);
                }
                if foton_md_html_converter::io::convert::convert_file(&target, kicker).is_err() {
                    process::exit(1);
                }
            } else {
                process::exit(1);
            }
        }
        None => {
            let cwd = std::env::current_dir().expect("Failed to get current directory");
            convert_directory(&cwd, kicker);
        }
    }
}
