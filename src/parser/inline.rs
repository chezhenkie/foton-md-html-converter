use regex::Regex;
use std::sync::LazyLock;

static CODE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"`([^`]+)`").unwrap());
static BOLD_ITALIC_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\*\*\*(.+?)\*\*\*").unwrap());
static BOLD_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\*\*(.+?)\*\*").unwrap());
static ITALIC_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\*(.+?)\*").unwrap());
static LINK_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").unwrap());

pub fn escape_html(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for ch in text.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(ch),
        }
    }
    out
}

fn decorate(text: &str) -> String {
    let text = BOLD_ITALIC_RE.replace_all(text, "<strong><em>$1</em></strong>");
    let text = BOLD_RE.replace_all(&text, "<strong>$1</strong>");
    let text = ITALIC_RE.replace_all(&text, "<em>$1</em>");
    LINK_RE
        .replace_all(&text, r#"<a href="$2">$1</a>"#)
        .to_string()
}

pub fn inline(text: &str) -> String {
    let escaped = escape_html(text);
    let mut out = String::with_capacity(escaped.len());
    let mut last = 0;

    for caps in CODE_RE.captures_iter(&escaped) {
        let span = caps.get(0).unwrap();
        out.push_str(&decorate(&escaped[last..span.start()]));
        out.push_str("<code>");
        out.push_str(&caps[1]);
        out.push_str("</code>");
        last = span.end();
    }
    out.push_str(&decorate(&escaped[last..]));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_html_escapes_core_chars() {
        assert_eq!(
            escape_html("5 < 10 & 10 > 5 \"quoted\" 'x'"),
            "5 &lt; 10 &amp; 10 &gt; 5 &quot;quoted&quot; &#39;x&#39;"
        );
    }

    #[test]
    fn link_url_with_quote_cannot_break_href() {
        let out = inline("[x](a\"onmouseover=\"y)");
        assert!(
            !out.contains("\"onmouseover"),
            "attribute injection: {}",
            out
        );
        assert!(out.starts_with("<a href=\"a&quot;onmouseover=&quot;y\">x</a>"));
    }

    #[test]
    fn escape_html_leaves_plain_text_unchanged() {
        assert_eq!(escape_html("hello world"), "hello world");
    }

    #[test]
    fn inline_code() {
        assert_eq!(inline("use `code` here"), "use <code>code</code> here");
    }

    #[test]
    fn inline_bold() {
        assert_eq!(inline("this is **bold**"), "this is <strong>bold</strong>");
    }

    #[test]
    fn inline_italic() {
        assert_eq!(inline("this is *italic*"), "this is <em>italic</em>");
    }

    #[test]
    fn inline_bold_italic() {
        assert_eq!(
            inline("this is ***bold italic***"),
            "this is <strong><em>bold italic</em></strong>"
        );
    }

    #[test]
    fn inline_link() {
        assert_eq!(
            inline("[example](https://example.com)"),
            r#"<a href="https://example.com">example</a>"#
        );
    }

    #[test]
    fn inline_code_escapes_html_first() {
        assert_eq!(
            inline("`<div>foo & bar</div>`"),
            "<code>&lt;div&gt;foo &amp; bar&lt;/div&gt;</code>"
        );
    }

    #[test]
    fn inline_combined_formatting() {
        assert_eq!(
            inline("**bold** and *italic* and `code`"),
            "<strong>bold</strong> and <em>italic</em> and <code>code</code>"
        );
    }

    #[test]
    fn code_span_is_protected_from_later_transforms() {
        assert_eq!(
            inline("`**x** and [a](b)`"),
            "<code>**x** and [a](b)</code>"
        );
    }

    #[test]
    fn formatting_around_code_still_works() {
        assert_eq!(
            inline("**bold** `x` *em*"),
            "<strong>bold</strong> <code>x</code> <em>em</em>"
        );
    }
}
