fn main() {
    let target = std::env::var("TARGET").unwrap_or_default();
    if !target.contains("windows") {
        return;
    }

    let mut res = winresource::WindowsResource::new();
    res.set("FileDescription", "Markdown to styled HTML converter");
    res.set("ProductName", "foton-md-html-converter-x64");
    res.set("CompanyName", "westercoder");
    res.set("FileVersion", "1.5.0.0");
    res.set("ProductVersion", "1.5.0.0");
    res.set("LegalCopyright", "Copyright 2026 westercoder");
    res.set("OriginalFilename", "foton-md-html-converter-x64.exe");
    res.set_icon("src/icon.ico");
    res.set_language(0x0409);
    res.compile().expect("Failed to compile Windows resources");
}
