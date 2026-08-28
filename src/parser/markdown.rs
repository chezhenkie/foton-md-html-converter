use super::inline::{escape_html, inline};
use regex::Regex;
use std::sync::LazyLock;

static HEADING_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(#{1,6})\s+(.+)$").unwrap());
static UL_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[-*]\s+").unwrap());
static OL_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(\d+)\.\s+(.+)$").unwrap());
static SEP_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[-:]+$").unwrap());

pub fn extract_lede(md: &str) -> Option<String> {
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
    let lines: Vec<&str> = md.split('\n').collect();
    let mut html_parts: Vec<String> = Vec::new();
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
    let mut in_svg = false;
    let mut in_table = false;
    let mut in_ul = false;
    let mut in_ol = false;
    let mut code_buf: Vec<String> = Vec::new();
    let mut code_lang = String::new();
    let mut section_num = 0u32;

    let close_lists = |in_ul: &mut bool, in_ol: &mut bool, parts: &mut Vec<String>| {
        if *in_ul {
            parts.push("</ul>".to_string());
            *in_ul = false;
        }
        if *in_ol {
            parts.push("</ol>".to_string());
            *in_ol = false;
        }
    };

    let close_table = |in_table: &mut bool, parts: &mut Vec<String>| {
        if *in_table {
            parts.push("</tbody></table>".to_string());
            *in_table = false;
        }
    };

    while i < lines.len() {
        let line = lines[i];

        if line.trim().starts_with("```") {
            if !in_code {
                close_lists(&mut in_ul, &mut in_ol, &mut html_parts);
                close_table(&mut in_table, &mut html_parts);
                in_code = true;
                code_lang = line.trim()[3..].trim().to_string();
                if code_lang == "svg" {
                    in_svg = true;
                }
                code_buf.clear();
            } else if in_svg {
                let raw = code_buf.join("\n");
                let body = if raw.trim_start().starts_with("<svg") {
                    format!("<div class=\"svg-diagram\">{}</div>", raw)
                } else {
                    super::ascii::ascii_to_html(&raw)
                };
                html_parts.push(body);
                in_code = false;
                in_svg = false;
                code_lang.clear();
            } else {
                let lang_attr = if code_lang.is_empty() {
                    String::new()
                } else {
                    format!(" class=\"language-{}\"", code_lang)
                };
                let joined = escape_html(&code_buf.join("\n"));
                html_parts.push(format!("<pre><code{}>{}</code></pre>", lang_attr, joined));
                in_code = false;
                code_lang.clear();
            }
            i += 1;
            continue;
        }

        if in_code {
            code_buf.push(line.to_string());
            i += 1;
            continue;
        }

        let stripped = line.trim();

        if stripped.is_empty() {
            close_lists(&mut in_ul, &mut in_ol, &mut html_parts);
            close_table(&mut in_table, &mut html_parts);
            i += 1;
            continue;
        }

        if stripped == "---" || stripped == "***" || stripped == "___" {
            close_lists(&mut in_ul, &mut in_ol, &mut html_parts);
            close_table(&mut in_table, &mut html_parts);
            i += 1;
            continue;
        }

        if let Some(caps) = HEADING_RE.captures(stripped) {
            close_lists(&mut in_ul, &mut in_ol, &mut html_parts);
            close_table(&mut in_table, &mut html_parts);
            let level = caps.get(1).unwrap().as_str().len();
            let content = inline(caps.get(2).unwrap().as_str());

            if level == 2 {
                section_num += 1;
                html_parts.push(format!(
                    "<h2><span class=\"num\">{}</span>{}</h2>",
                    section_num, content
                ));
                html_parts.push("<div class=\"rule\"></div>".to_string());
            } else if level == 3 {
                html_parts.push(format!("<h3>{}</h3>", content));
            } else {
                html_parts.push(format!("<h{}>{}</h{}>", level, content, level));
            }
            i += 1;
            continue;
        }

        if stripped.contains('|') && stripped.starts_with('|') {
            close_lists(&mut in_ul, &mut in_ol, &mut html_parts);
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
                in_table = true;
                let ths: String = cells
                    .iter()
                    .map(|c| format!("<th>{}</th>", inline(c)))
                    .collect();
                html_parts.push(format!("<table><thead><tr>{}</tr></thead><tbody>", ths));
            } else {
                let tds: String = cells
                    .iter()
                    .map(|c| format!("<td>{}</td>", inline(c)))
                    .collect();
                html_parts.push(format!("<tr>{}</tr>", tds));
            }
            i += 1;
            continue;
        } else {
            close_table(&mut in_table, &mut html_parts);
        }

        if stripped.starts_with('>') {
            close_lists(&mut in_ul, &mut in_ol, &mut html_parts);
            let content = inline(stripped.trim_start_matches('>').trim());
            html_parts.push(format!("<div class=\"callout\"><p>{}</p></div>", content));
            i += 1;
            continue;
        }

        if UL_RE.is_match(stripped) {
            if in_ol {
                html_parts.push("</ol>".to_string());
                in_ol = false;
            }
            if !in_ul {
                html_parts.push("<ul>".to_string());
                in_ul = true;
            }
            let content = inline(UL_RE.replace(stripped, "").as_ref());
            html_parts.push(format!("<li>{}</li>", content));
            i += 1;
            continue;
        }

        if let Some(caps) = OL_RE.captures(stripped) {
            if in_ul {
                html_parts.push("</ul>".to_string());
                in_ul = false;
            }
            if !in_ol {
                html_parts.push("<ol>".to_string());
                in_ol = true;
            }
            let content = inline(caps.get(2).unwrap().as_str());
            html_parts.push(format!("<li>{}</li>", content));
            i += 1;
            continue;
        }

        close_lists(&mut in_ul, &mut in_ol, &mut html_parts);

        let content = inline(stripped);
        html_parts.push(format!("<p>{}</p>", content));
        i += 1;
    }

    close_lists(&mut in_ul, &mut in_ol, &mut html_parts);
    close_table(&mut in_table, &mut html_parts);

    if in_code {
        if in_svg {
            let raw = code_buf.join("\n");
            let body = if raw.trim_start().starts_with("<svg") {
                format!("<div class=\"svg-diagram\">{}</div>", raw)
            } else {
                super::ascii::ascii_to_html(&raw)
            };
            html_parts.push(body);
        } else {
            let lang_attr = if code_lang.is_empty() {
                String::new()
            } else {
                format!(" class=\"language-{}\"", code_lang)
            };
            let joined = escape_html(&code_buf.join("\n"));
            html_parts.push(format!("<pre><code{}>{}</code></pre>", lang_attr, joined));
        }
    }

    html_parts.join("\n")
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
    fn svg_fenced_block_passes_through_verbatim() {
        let input = "```svg\n<svg viewBox=\"0 0 100 20\"><rect x=\"5\" y=\"5\" width=\"90\" height=\"10\"/></svg>\n```";
        let output = md_to_html(input);
        assert!(output.contains("<svg viewBox=\"0 0 100 20\">"));
        assert!(output.contains("</svg>"));
        assert!(!output.contains("&lt;svg"));
        assert!(!output.contains("&lt;/svg&gt;"));
    }

    #[test]
    fn fenced_code_block_still_escapes() {
        let input = "```html\n<div class=\"x\">5 < 10</div>\n```";
        let output = md_to_html(input);
        assert!(output.contains("&lt;div"));
        assert!(output.contains("&lt;"));
        assert!(!output.contains("<div class=\"svg-diagram\">"));
    }

    #[test]
    fn unclosed_svg_block_is_flushed_at_eof() {
        let input = "```svg\n<svg viewBox=\"0 0 100 20\"><rect/></svg>\n";
        let output = md_to_html(input);
        assert!(output.contains("<div class=\"svg-diagram\">"));
        assert!(output.contains("<svg viewBox=\"0 0 100 20\">"));
        assert!(!output.contains("&lt;svg"));
    }

    #[test]
    fn ascii_diagram_in_svg_fence_is_converted() {
        let input =
            "```svg\n+------+      +------+\n|  A   | ---> |  B   |\n+------+      +------+\n```";
        let output = md_to_html(input);
        assert!(output.contains("<div class=\"svg-diagram\">"));
        assert!(output.contains("diagram-graph"));
        assert!(output.contains("diagram-node"));
        assert!(output.contains("__diagramQueue"));
        assert!(!output.contains("<pre><code"));
    }
}
