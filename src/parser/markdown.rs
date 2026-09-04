use super::inline::{escape_html, inline};
use regex::Regex;
use std::sync::LazyLock;

static HEADING_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(#{1,6})\s+(.+)$").unwrap());
static UL_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[-*]\s+").unwrap());
static OL_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(\d+)\.\s+(.+)$").unwrap());
static SEP_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[-:]+$").unwrap());

fn strip_bom(md: &str) -> &str {
    md.strip_prefix('\u{feff}').unwrap_or(md)
}

fn is_hr(line: &str) -> bool {
    let Some(c) = line.chars().next() else {
        return false;
    };
    if !matches!(c, '-' | '*' | '_') {
        return false;
    }
    let mut count = 1;
    for ch in line.chars().skip(1) {
        match ch {
            _ if ch == c => count += 1,
            ' ' | '\t' => {}
            _ => return false,
        }
    }
    count >= 3
}

// Buffer helpers: the old Vec<String> + join("\n") is replaced by one String;
// begin_line reproduces the join separator between document parts.
fn begin_line(html: &mut String) {
    if !html.is_empty() {
        html.push('\n');
    }
}

fn close_lists(html: &mut String, in_ul: &mut bool, in_ol: &mut bool) {
    if *in_ul {
        begin_line(html);
        html.push_str("</ul>");
        *in_ul = false;
    }
    if *in_ol {
        begin_line(html);
        html.push_str("</ol>");
        *in_ol = false;
    }
}

fn close_table(html: &mut String, in_table: &mut bool) {
    if *in_table {
        begin_line(html);
        html.push_str("</tbody></table>");
        *in_table = false;
    }
}

fn flush_code_block(html: &mut String, code_buf: &[&str], code_lang: &str) {
    begin_line(html);
    html.push_str("<pre><code");
    if !code_lang.is_empty() {
        html.push_str(" class=\"language-");
        html.push_str(code_lang);
        html.push('"');
    }
    html.push('>');
    html.push_str(&escape_html(&code_buf.join("\n")));
    html.push_str("</code></pre>");
}

pub fn extract_lede(md: &str) -> Option<String> {
    let md = strip_bom(md);
    let lines: Vec<&str> = md.split('\n').collect();
    if lines.first() != Some(&"---") {
        return None;
    }
    for line in lines.iter().skip(1) {
        let line = line.trim();
        if line == "---" {
            break;
        }
        if let Some(value) = line.strip_prefix("lede:") {
            return Some(value.trim().to_string());
        }
    }
    None
}

pub fn md_to_html(md: &str) -> String {
    let md = strip_bom(md);
    let lines: Vec<&str> = md.split('\n').collect();
    let mut html = String::with_capacity(md.len());
    let mut i = 0;

    if lines.first() == Some(&"---") {
        for (j, line) in lines.iter().enumerate().skip(1) {
            if line.trim() == "---" {
                i = j + 1;
                break;
            }
        }
    }

    let mut in_code = false;
    let mut in_table = false;
    let mut in_ul = false;
    let mut in_ol = false;
    let mut code_buf: Vec<&str> = Vec::new();
    let mut code_lang = String::new();
    let mut section_num = 0u32;

    while i < lines.len() {
        let line = lines[i];

        if line.trim().starts_with("```") {
            if !in_code {
                close_lists(&mut html, &mut in_ul, &mut in_ol);
                close_table(&mut html, &mut in_table);
                in_code = true;
                code_lang = line.trim()[3..].trim().to_string();
                code_buf.clear();
            } else {
                flush_code_block(&mut html, &code_buf, &code_lang);
                in_code = false;
                code_lang.clear();
            }
            i += 1;
            continue;
        }

        if in_code {
            code_buf.push(line);
            i += 1;
            continue;
        }

        let stripped = line.trim();

        if stripped.is_empty() {
            close_lists(&mut html, &mut in_ul, &mut in_ol);
            close_table(&mut html, &mut in_table);
            i += 1;
            continue;
        }

        if is_hr(stripped) {
            close_lists(&mut html, &mut in_ul, &mut in_ol);
            close_table(&mut html, &mut in_table);
            i += 1;
            continue;
        }

        if let Some(caps) = HEADING_RE.captures(stripped) {
            close_lists(&mut html, &mut in_ul, &mut in_ol);
            close_table(&mut html, &mut in_table);
            let level = caps.get(1).unwrap().as_str().len();
            let content = inline(caps.get(2).unwrap().as_str());

            begin_line(&mut html);
            if level == 2 {
                section_num += 1;
                html.push_str("<h2><span class=\"num\">");
                html.push_str(section_num.to_string().as_str());
                html.push_str("</span>");
                html.push_str(&content);
                html.push_str("</h2>");
                begin_line(&mut html);
                html.push_str("<div class=\"rule\"></div>");
            } else if level == 3 {
                html.push_str("<h3>");
                html.push_str(&content);
                html.push_str("</h3>");
            } else {
                let lvl = level.to_string();
                html.push_str("<h");
                html.push_str(&lvl);
                html.push('>');
                html.push_str(&content);
                html.push_str("</h");
                html.push_str(&lvl);
                html.push('>');
            }
            i += 1;
            continue;
        }

        if stripped.contains('|') && stripped.starts_with('|') {
            close_lists(&mut html, &mut in_ul, &mut in_ol);
            let cells: Vec<&str> = stripped
                .split('|')
                .skip(1)
                .take(stripped.matches('|').count() - 1)
                .map(|s| s.trim())
                .collect();

            if in_table && cells.iter().all(|c| SEP_RE.is_match(c)) {
                i += 1;
                continue;
            }

            if !in_table {
                if cells.iter().all(|c| SEP_RE.is_match(c)) {
                    i += 1;
                    continue;
                }
                in_table = true;
                begin_line(&mut html);
                html.push_str("<table><thead><tr>");
                for c in &cells {
                    html.push_str("<th>");
                    html.push_str(&inline(c));
                    html.push_str("</th>");
                }
                html.push_str("</tr></thead><tbody>");
            } else {
                begin_line(&mut html);
                html.push_str("<tr>");
                for c in &cells {
                    html.push_str("<td>");
                    html.push_str(&inline(c));
                    html.push_str("</td>");
                }
                html.push_str("</tr>");
            }
            i += 1;
            continue;
        } else {
            close_table(&mut html, &mut in_table);
        }

        if stripped.starts_with('>') {
            close_lists(&mut html, &mut in_ul, &mut in_ol);
            let content = inline(stripped.trim_start_matches('>').trim());
            begin_line(&mut html);
            html.push_str("<div class=\"callout\"><p>");
            html.push_str(&content);
            html.push_str("</p></div>");
            i += 1;
            continue;
        }

        if UL_RE.is_match(stripped) {
            if in_ol {
                begin_line(&mut html);
                html.push_str("</ol>");
                in_ol = false;
            }
            if !in_ul {
                begin_line(&mut html);
                html.push_str("<ul>");
                in_ul = true;
            }
            let content = inline(UL_RE.replace(stripped, "").as_ref());
            begin_line(&mut html);
            html.push_str("<li>");
            html.push_str(&content);
            html.push_str("</li>");
            i += 1;
            continue;
        }

        if let Some(caps) = OL_RE.captures(stripped) {
            if in_ul {
                begin_line(&mut html);
                html.push_str("</ul>");
                in_ul = false;
            }
            if !in_ol {
                begin_line(&mut html);
                html.push_str("<ol>");
                in_ol = true;
            }
            let content = inline(caps.get(2).unwrap().as_str());
            begin_line(&mut html);
            html.push_str("<li>");
            html.push_str(&content);
            html.push_str("</li>");
            i += 1;
            continue;
        }

        close_lists(&mut html, &mut in_ul, &mut in_ol);

        let content = inline(stripped);
        begin_line(&mut html);
        html.push_str("<p>");
        html.push_str(&content);
        html.push_str("</p>");
        i += 1;
    }

    close_lists(&mut html, &mut in_ul, &mut in_ol);
    close_table(&mut html, &mut in_table);

    if in_code {
        flush_code_block(&mut html, &code_buf, &code_lang);
    }

    html
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input_produces_empty_output() {
        assert_eq!(md_to_html(""), "");
    }

    #[test]
    fn paragraph() {
        assert_eq!(md_to_html("Hello world."), "<p>Hello world.</p>");
    }

    #[test]
    fn h1_and_h2_with_section_number() {
        let input = "# Title\n\n## Section\n";
        assert_eq!(
            md_to_html(input),
            "<h1>Title</h1>\n<h2><span class=\"num\">1</span>Section</h2>\n<div class=\"rule\"></div>"
        );
    }

    #[test]
    fn unordered_list() {
        let input = "- a\n- b\n- c\n";
        assert_eq!(
            md_to_html(input),
            "<ul>\n<li>a</li>\n<li>b</li>\n<li>c</li>\n</ul>"
        );
    }

    #[test]
    fn ordered_list() {
        let input = "1. first\n2. second\n";
        assert_eq!(
            md_to_html(input),
            "<ol>\n<li>first</li>\n<li>second</li>\n</ol>"
        );
    }

    #[test]
    fn fenced_code_block() {
        let input = "```python\ndef f():\n    pass\n```\n";
        assert_eq!(
            md_to_html(input),
            "<pre><code class=\"language-python\">def f():\n    pass</code></pre>"
        );
    }

    #[test]
    fn table_with_separator_row() {
        let input = "| A | B |\n|---|---|\n| 1 | 2 |\n";
        assert_eq!(
            md_to_html(input),
            "<table><thead><tr><th>A</th><th>B</th></tr></thead><tbody>\n<tr><td>1</td><td>2</td></tr>\n</tbody></table>"
        );
    }

    #[test]
    fn complex_table_with_inline_formatting() {
        let input = "| Name | Value |\n|------|-------|\n| **bold** | `code` |\n| [link](https://example.com) | *italic* |\n";
        assert_eq!(
            md_to_html(input),
            "<table><thead><tr><th>Name</th><th>Value</th></tr></thead><tbody>\n<tr><td><strong>bold</strong></td><td><code>code</code></td></tr>\n<tr><td><a href=\"https://example.com\">link</a></td><td><em>italic</em></td></tr>\n</tbody></table>"
        );
    }

    #[test]
    fn blockquote_becomes_callout() {
        assert_eq!(
            md_to_html("> note"),
            "<div class=\"callout\"><p>note</p></div>"
        );
    }

    #[test]
    fn yaml_front_matter_is_skipped() {
        let input = "---\ntitle: Test\n---\n# Hello\n";
        assert_eq!(md_to_html(input), "<h1>Hello</h1>");
    }

    #[test]
    fn horizontal_rule_is_ignored() {
        assert_eq!(md_to_html("---"), "");
    }

    #[test]
    fn horizontal_rule_variants_are_ignored() {
        assert_eq!(md_to_html("----"), "");
        assert_eq!(md_to_html("- - -"), "");
        assert_eq!(md_to_html("*****"), "");
        assert_eq!(md_to_html("____"), "");
        assert_eq!(md_to_html("--"), "<p>--</p>");
    }

    #[test]
    fn separator_only_first_table_line_is_skipped() {
        let input = "|---|---|\n| a | b |";
        assert_eq!(
            md_to_html(input),
            "<table><thead><tr><th>a</th><th>b</th></tr></thead><tbody>\n</tbody></table>"
        );
    }

    #[test]
    fn utf8_bom_is_ignored() {
        let input = "\u{feff}---\nlede: Hi\n---\n# Hello\n";
        assert_eq!(md_to_html(input), "<h1>Hello</h1>");
        assert_eq!(extract_lede(input), Some("Hi".to_string()));
    }

    #[test]
    fn special_characters_are_escaped() {
        assert_eq!(
            md_to_html("5 < 10 and 10 > 5"),
            "<p>5 &lt; 10 and 10 &gt; 5</p>"
        );
    }

    #[test]
    fn unclosed_code_block_is_flushed_at_eof() {
        let input = "```\nline\n";
        assert_eq!(md_to_html(input), "<pre><code>line\n</code></pre>");
    }

    #[test]
    fn mixed_list_switching() {
        let input = "- a\n1. b\n- c\n";
        assert_eq!(
            md_to_html(input),
            "<ul>\n<li>a</li>\n</ul>\n<ol>\n<li>b</li>\n</ol>\n<ul>\n<li>c</li>\n</ul>"
        );
    }

    #[test]
    fn h3_heading() {
        assert_eq!(md_to_html("### Subsection"), "<h3>Subsection</h3>");
    }

    #[test]
    fn extract_lede_from_front_matter() {
        let input = "---\nlede: A short summary\n---\n# Hello\n";
        assert_eq!(extract_lede(input), Some("A short summary".to_string()));
    }

    #[test]
    fn no_lede_when_no_front_matter() {
        assert_eq!(extract_lede("# Hello\n"), None);
    }

    #[test]
    fn svg_fence_is_treated_as_plain_code_block() {
        let input = "```svg\n<svg viewBox=\"0 0 100 20\"><rect x=\"5\" y=\"5\" width=\"90\" height=\"10\"/></svg>\n```";
        let output = md_to_html(input);
        assert!(output.contains("&lt;svg viewBox=&quot;0 0 100 20&quot;&gt;"));
        assert!(output.contains("&lt;/svg&gt;"));
        assert!(!output.contains("<svg"));
        assert!(!output.contains("diagram"));
    }

    #[test]
    fn fenced_code_block_still_escapes() {
        let input = "```html\n<div class=\"x\">5 < 10</div>\n```";
        let output = md_to_html(input);
        assert!(output.contains("&lt;div"));
        assert!(output.contains("&lt;"));
    }
}
