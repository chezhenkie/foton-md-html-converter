use std::fs;
use std::path::Path;

fn title_for(filename: &str) -> Option<&str> {
    match filename {
        "01-containers.md" => Some("Container Operations"),
        "02-images.md" => Some("Image Operations"),
        "03-pods.md" => Some("Pod Operations"),
        "04-networks-volumes-secrets.md" => Some("Networks, Volumes, Secrets"),
        "05-system-manifests-events.md" => Some("System, Manifests, Events"),
        "06-client-utilities.md" => Some("Client, Utilities & Patterns"),
        _ => None,
    }
}

pub fn convert_file(md_file: &Path, kicker: Option<&str>) -> Result<(), String> {
    let md = fs::read_to_string(md_file)
        .map_err(|e| format!("Failed to read {}: {}", md_file.display(), e))?;

    let filename = md_file.file_name().and_then(|s| s.to_str()).unwrap_or("");

    let title = title_for(filename).unwrap_or_else(|| {
        md_file
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Untitled")
    });

    let lede = crate::parser::markdown::extract_lede(&md).unwrap_or_default();

    let body_html = crate::parser::markdown::md_to_html(&md);
    let page = crate::html::builder::build_page(title, &lede, &body_html, kicker);

    let out_file = md_file.with_extension("html");
    fs::write(&out_file, page)
        .map_err(|e| format!("Failed to write {}: {}", out_file.display(), e))?;

    let out_name = out_file
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");
    println!("  {}  ->  {}", out_name, out_file.display());

    Ok(())
}
