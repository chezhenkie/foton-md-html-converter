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
            _ => out.push(ch),
        }
    }
    out
}

pub fn inline(text: &str) -> String {
    let mut text = escape_html(text);

    text = CODE_RE.replace_all(&text, "<code>$1</code>").to_string();
    text = BOLD_ITALIC_RE
        .replace_all(&text, "<strong><em>$1</em></strong>")
        .to_string();
    text = BOLD_RE
        .replace_all(&text, "<strong>$1</strong>")
        .to_string();
    text = ITALIC_RE.replace_all(&text, "<em>$1</em>").to_string();
    text = LINK_RE
        .replace_all(&text, r#"<a href="$2">$1</a>"#)
        .to_string();

    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_html_escapes_core_chars() {
        assert_eq!(escape_html("5 < 10 & 10 > 5"), "5 &lt; 10 &amp; 10 &gt; 5");
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
}
