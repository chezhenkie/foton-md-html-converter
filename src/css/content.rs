pub const CONTENT_CSS: &str = r#"*{box-sizing:border-box}
.wrap{max-width:990px;margin:0 auto;padding:64px 32px 96px}
header{border-top:6px solid var(--accent);padding-top:28px;margin-bottom:8px;transition:border-color .25s}
.kicker{font-family:var(--font-ui);font-size:12px;letter-spacing:.22em;text-transform:uppercase;color:var(--page-muted);margin:0 0 14px;transition:color .25s}
h1{font-size:38px;font-weight:400;line-height:1.2;color:var(--page-ink);margin:0 0 12px;letter-spacing:-.01em;transition:color .25s}
h1 em{font-style:italic;color:var(--accent);transition:color .25s}
.lede{font-family:var(--font-serif);font-size:18px;line-height:1.5;color:var(--accent);margin:0 0 4px;font-style:italic;transition:color .25s}
.lede small{display:block;font-style:normal;font-family:var(--font-ui);font-size:12.5px;color:var(--page-muted);margin-top:10px;transition:color .25s}
h2{font-size:26px;font-weight:400;color:var(--page-ink);margin:56px 0 6px;letter-spacing:-.01em;transition:color .25s}
h2 .num{font-family:var(--font-ui);font-size:14px;color:var(--accent);vertical-align:super;margin-right:8px;transition:color .25s}
h2+.rule{height:1px;background:var(--page-line);margin:0 0 22px;transition:background .25s}
h3{font-weight:600;color:var(--page-ink);font-size:19px;margin:26px 0 6px;transition:color .25s}
p{margin:0 0 16px}
em{font-family:var(--font-serif);color:var(--accent);transition:color .25s}
strong{color:var(--accent);font-weight:600;transition:color .25s}
table{width:100%;border-collapse:collapse;margin:20px 0 28px;background:var(--card);border:1px solid var(--page-line);transition:background .25s,border-color .25s}
th,td{text-align:left;padding:10px 14px;vertical-align:top}
th{font-family:var(--font-ui);font-size:12.5px;letter-spacing:.04em;text-transform:uppercase;color:var(--page-muted);background:var(--card);border-bottom:2px solid var(--accent);font-weight:600;transition:background .25s,color .25s,border-color .25s}
td{font-size:14.5px;border-bottom:1px solid var(--page-line);color:var(--page-ink);transition:background .25s,border-color .25s,color .25s}
tr:last-child td{border-bottom:none}
.callout{border-left:4px solid var(--accent);background:var(--card);padding:16px 20px;margin:22px 0;font-family:var(--font-serif);font-style:italic;color:var(--page-muted);transition:background .25s,border-color .25s,color .25s}
.callout strong{font-family:var(--font-ui);font-style:normal}
ul,ol{margin:0 0 18px;padding-left:26px}
li{margin:0 0 8px}
li::marker{color:var(--accent)}
code{font-family:var(--font-mono);font-size:13px;background:transparent;color:var(--page-ink);border:1px solid var(--page-line);padding:1px 5px;border-radius:3px;transition:background .25s,color .25s,border-color .25s}
pre{background:var(--card);color:var(--page-ink);border:1px solid var(--page-line);padding:16px 20px;border-radius:4px;overflow-x:auto;margin:18px 0 24px;font-family:var(--font-mono);font-size:13px;line-height:1.6;transition:background .25s,color .25s,border-color .25s}
pre code{background:none;color:inherit;border:0;padding:0}
@media(max-width:620px){.wrap{padding:40px 18px 72px}h1{font-size:28px}}"#;
