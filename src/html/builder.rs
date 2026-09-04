use super::toolbar::TOOLBAR_HTML;
use crate::css;
use crate::js;
use crate::parser::inline::escape_html;

pub fn build_page(title: &str, lede: &str, body_html: &str, kicker: Option<&str>) -> String {
    let title = escape_html(title);
    let kicker_html = kicker
        .map(|k| format!("<p class=\"kicker\">{}</p>\n", escape_html(k)))
        .unwrap_or_default();
    let lede_html = if lede.is_empty() {
        String::new()
    } else {
        format!("<p class=\"lede\">{}</p>\n", escape_html(lede))
    };

    let header_html = format!("{}{}<h1>{}</h1>", kicker_html, lede_html, title);

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{}</title>
<style>
{}
{}
{}
</style>
<script>{}</script>
</head>
<body>
{}
<div class="wrap">
  <header>
    {}  </header>
  <main>
{}
  </main>
</div>
<script>{}</script>
<script>{}</script>
<script>{}</script>
</body>
</html>"#,
        title,
        css::TOOLBAR_CSS,
        css::THEME_CSS,
        css::CONTENT_CSS,
        js::OS_DETECT_JS,
        TOOLBAR_HTML,
        header_html,
        body_html,
        js::THEME_ENGINE_JS,
        js::MENUBAR_JS,
        js::INIT_JS
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn title_and_kicker_are_escaped() {
        let page = build_page("a&b<title>", "", "", Some("k<i>cker & co"));
        assert!(page.contains("<title>a&amp;b&lt;title&gt;</title>"));
        assert!(page.contains("<h1>a&amp;b&lt;title&gt;</h1>"));
        assert!(page.contains("<p class=\"kicker\">k&lt;i&gt;cker &amp; co</p>"));
    }

    #[test]
    fn empty_kicker_and_lede_are_omitted() {
        let page = build_page("T", "", "<p>x</p>", None);
        assert!(!page.contains("class=\"kicker\""));
        assert!(!page.contains("class=\"lede\""));
        assert!(page.contains("<h1>T</h1>"));
    }
}
