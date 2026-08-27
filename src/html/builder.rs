use super::toolbar::TOOLBAR_HTML;
use crate::css;
use crate::js;

pub fn build_page(title: &str, _lede: &str, body_html: &str, kicker: Option<&str>) -> String {
    let kicker_html = kicker
        .map(|k| format!("<p class=\"kicker\">{}</p>\n", k))
        .unwrap_or_default();

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
        kicker_html,
        body_html,
        js::THEME_ENGINE_JS,
        js::MENUBAR_JS,
        js::INIT_JS
    )
}
