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
        Err(e) => {
            eprintln!("Error: cannot read directory {}: {}", dir.display(), e);
            process::exit(1);
        }
    };
    if md_files.is_empty() {
        eprintln!("No .md or .txt files found in {}", dir.display());
        process::exit(1);
    }
    println!(
        "Converting {} .md/.txt file(s) in {}",
        md_files.len(),
        dir.display()
    );
    for md_file in md_files {
        if let Err(e) = foton_md_html_converter::io::convert::convert_file(&md_file, kicker) {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
    println!("\nDone: {}", dir.display());
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
                    println!(
                        "Converting {} .md/.txt file(s) in {}",
                        files.len(),
                        path.display()
                    );
                    for md_file in files {
                        if let Err(e) =
                            foton_md_html_converter::io::convert::convert_file(&md_file, kicker)
                        {
                            eprintln!("{}", e);
                            failed += 1;
                        }
                    }
                }
                Ok(_) => {
                    eprintln!("No .md or .txt files found in {}", path.display());
                    failed += 1;
                }
                Err(e) => {
                    eprintln!("Error: cannot read directory {}: {}", path.display(), e);
                    failed += 1;
                }
            }
        } else if path.is_file() {
            let ext = path
                .extension()
                .and_then(|e| e.to_str())
                .map(|s| s.to_lowercase())
                .unwrap_or_default();
            if ext != "md" && ext != "txt" {
                eprintln!("Error: not a .md or .txt file: {}", path.display());
                failed += 1;
                continue;
            }
            println!("Converting single file");
            if let Err(e) = foton_md_html_converter::io::convert::convert_file(&path, kicker) {
                eprintln!("{}", e);
                failed += 1;
            }
        } else {
            eprintln!("Error: path not found: {}", path.display());
            failed += 1;
        }
    }

    if failed > 0 {
        eprintln!("\nDone with {} error(s)", failed);
        process::exit(1);
    }
    println!("\nDone: {} path(s)", paths.len());
}

fn main() {
    let args = Args::parse();

    let kicker = args.kicker.as_deref();

    if !args.files.is_empty() {
        if args.target.is_some() {
            eprintln!("Error: give either positional paths or --target, not both");
            process::exit(1);
        }
        convert_paths(&args.files, kicker);
        return;
    }

    match args.target {
        Some(target) => {
            let target_str = target.to_string_lossy();
            let trimmed = target_str.trim_end_matches(['/', '\\']);
            let target = PathBuf::from(trimmed);

            if target.is_dir() {
                convert_directory(&target, kicker);
            } else if target.is_file() {
                let ext = target
                    .extension()
                    .and_then(|e| e.to_str())
                    .map(|s| s.to_lowercase())
                    .unwrap_or_default();
                if ext != "md" && ext != "txt" {
                    eprintln!("Error: not a .md or .txt file: {}", target.display());
                    process::exit(1);
                }
                println!("Converting single file");
                if let Err(e) = foton_md_html_converter::io::convert::convert_file(&target, kicker)
                {
                    eprintln!("{}", e);
                    process::exit(1);
                }
                println!("\nDone: 1 file");
            } else {
                eprintln!("Error: path not found: {}", target.display());
                process::exit(1);
            }
        }
        None => {
            let cwd = std::env::current_dir().expect("Failed to get current directory");
            convert_directory(&cwd, kicker);
        }
    }
}
