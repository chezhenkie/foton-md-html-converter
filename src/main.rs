use clap::Parser;
use std::path::PathBuf;
use std::process;

#[derive(Parser)]
#[command(
    name = "foton-md-html-converter-x64",
    about = "Convert markdown to styled HTML (v7)",
    long_about = "HTML always lands in the same directory as its source .md file."
)]
struct Args {
    #[arg(
        long,
        help = "Path to a single .md file OR a directory to convert. A directory (trailing slash optional) converts every .md inside it. Default: convert every .md in the current working directory."
    )]
    target: Option<PathBuf>,

    #[arg(long, help = "Kicker text shown above the title on each page")]
    kicker: Option<String>,
}

fn main() {
    let args = Args::parse();

    let kicker = args.kicker.as_deref();

    match args.target {
        Some(target) => {
            let target_str = target.to_string_lossy();
            let trimmed = target_str.trim_end_matches(|c| c == '/' || c == '\\');
            let target = PathBuf::from(trimmed);

            if target.is_dir() {
                let md_files = foton_md_html_converter::io::files::collect_source_files(&target);
                if md_files.is_empty() {
                    eprintln!("No .md or .txt files found in {}", target.display());
                    process::exit(1);
                }
                println!(
                    "Converting {} .md/.txt file(s) in {}",
                    md_files.len(),
                    target.display()
                );
                for md_file in md_files {
                    if let Err(e) =
                        foton_md_html_converter::io::convert::convert_file(&md_file, kicker)
                    {
                        eprintln!("{}", e);
                        process::exit(1);
                    }
                }
                println!("\nDone: {}", target.display());
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
            let md_files = foton_md_html_converter::io::files::collect_source_files(&cwd);
            if md_files.is_empty() {
                eprintln!("No .md or .txt files found in {}", cwd.display());
                process::exit(1);
            }
            println!(
                "Converting {} .md/.txt file(s) in {}",
                md_files.len(),
                cwd.display()
            );
            for md_file in md_files {
                if let Err(e) = foton_md_html_converter::io::convert::convert_file(&md_file, kicker)
                {
                    eprintln!("{}", e);
                    process::exit(1);
                }
            }
            println!("\nDone: {}", cwd.display());
        }
    }
}
