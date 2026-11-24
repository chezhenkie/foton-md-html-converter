use super::toolbar::TOOLBAR_HTML;
use crate::css;
use crate::js;
use crate::parser::inline::{escape_html, inline};
use std::sync::LazyLock;

// Page shell: the constant ~21KB of CSS/JS is assembled ONCE per process here.
// build_page only splices title, header, and body into the prebuilt chunks,
// so per-file cost is a single sized allocation plus push_str copies.
const PAGE_PRE: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>"#;

const PAGE_MID: &str = r#"  </header>
  <main>
"#;

static PAGE_HEAD_REST: LazyLock<String> = LazyLock::new(|| {
    format!(
        r#"</title>
<style>
{}
{}
{}
</style>
<script>{}</script>
<script>{}</script>
</head>
<body>
{}
<div class="wrap">
  <header>
    "#,
        css::TOOLBAR_CSS,
        css::THEME_CSS,
        css::CONTENT_CSS,
        js::OS_DETECT_JS,
        js::THEME_BOOT_JS,
        TOOLBAR_HTML
    )
});

static PAGE_TAIL: LazyLock<String> = LazyLock::new(|| {
    format!(
        r#"
  </main>
</div>
<script>{}</script>
<script>{}</script>
<script>{}</script>
</body>
</html>"#,
        js::THEME_ENGINE_JS,
        js::MENUBAR_JS,
        js::INIT_JS
    )
});

pub fn build_page(
    title: &str,
    heading: Option<&str>,
    lede: &str,
    body_html: &str,
    kicker: Option<&str>,
) -> String {
    let title = escape_html(title);
    let kicker_html = kicker
        .map(|k| format!("<p class=\"kicker\">{}</p>\n", escape_html(k)))
        .unwrap_or_default();
    let lede_html = if lede.is_empty() {
        String::new()
    } else {
        format!("<p class=\"lede\">{}</p>\n", escape_html(lede))
    };
    let heading_html = heading
        .map(|h| format!("<h1>{}</h1>\n", inline(h)))
        .unwrap_or_default();

    let header_html = format!("{}{}{}", kicker_html, lede_html, heading_html);

    let mut page = String::with_capacity(
        PAGE_PRE.len()
            + title.len()
            + PAGE_HEAD_REST.len()
            + header_html.len()
            + PAGE_MID.len()
            + body_html.len()
            + PAGE_TAIL.len(),
    );
    page.push_str(PAGE_PRE);
    page.push_str(&title);
    page.push_str(&PAGE_HEAD_REST);
    page.push_str(&header_html);
    page.push_str(PAGE_MID);
    page.push_str(body_html);
    page.push_str(&PAGE_TAIL);
    page
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn title_tag_and_kicker_are_escaped() {
        let page = build_page("a&b<title>", None, "", "", Some("k<i>cker & co"));
        assert!(page.contains("<title>a&amp;b&lt;title&gt;</title>"));
        assert!(page.contains("<p class=\"kicker\">k&lt;i&gt;cker &amp; co</p>"));
    }

    #[test]
    fn heading_is_rendered_inline_and_escaped() {
        let page = build_page("T", Some("a <b> **bold**"), "", "", None);
        assert!(page.contains("<h1>a &lt;b&gt; <strong>bold</strong></h1>"));
    }

    #[test]
    fn empty_kicker_and_lede_are_omitted() {
        let page = build_page("T", None, "", "<p>x</p>", None);
        assert!(!page.contains("class=\"kicker\""));
        assert!(!page.contains("class=\"lede\""));
    }

    #[test]
    fn no_heading_is_injected_from_the_file_name() {
        let page = build_page("file-name", None, "", "<h1>Mine</h1>", Some("K"));
        assert_eq!(page.matches("<h1>").count(), 1);
        assert!(page.contains("<h1>Mine</h1>"));
        assert!(!page.contains("file-name</h1>"));
    }
}
