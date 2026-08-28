pub const CONTENT_CSS: &str = r#"*{box-sizing:border-box}
.wrap{max-width:990px;margin:0 auto;padding:64px 32px 96px}
header{border-top:6px solid var(--accent);padding-top:28px;margin-bottom:8px;transition:border-color .3s}
h1{font-size:38px;font-weight:400;line-height:1.2;color:var(--espresso);margin:0 0 12px;letter-spacing:-.01em;transition:color .3s}
h1 em{font-style:italic;color:var(--accent-deep);transition:color .3s}
.lede{font-size:17px;color:var(--ink-soft);margin:0 0 4px;font-style:italic;transition:color .3s}
.lede small{display:block;font-style:normal;font-family:var(--font-ui);font-size:12.5px;color:var(--bronze);margin-top:10px;transition:color .3s}
h2{font-size:26px;font-weight:400;color:var(--espresso);margin:56px 0 6px;letter-spacing:-.01em;transition:color .3s}
h2 .num{font-family:var(--font-ui);font-size:14px;color:var(--accent-deep);vertical-align:super;margin-right:8px;transition:color .3s}
h2+.rule{height:1px;background:var(--line);margin:0 0 22px;transition:background .3s}
h3{font-weight:600;color:var(--espresso);font-size:19px;margin:26px 0 6px;transition:color .3s}
p{margin:0 0 16px}
em{color:var(--accent-deep);transition:color .3s}
strong{color:var(--accent-deep);font-weight:600;transition:color .3s}
table{width:100%;border-collapse:collapse;margin:20px 0 28px;background:var(--card);border:1px solid var(--line);box-shadow:0 1px 0 var(--shadow-color);transition:background .3s,border-color .3s,box-shadow .3s}
th,td{text-align:left;padding:10px 14px;vertical-align:top}
th{font-family:var(--font-ui);font-size:12.5px;letter-spacing:.04em;text-transform:uppercase;color:var(--card);background:var(--espresso);border-bottom:2px solid var(--accent);font-weight:600;transition:background .3s,color .3s,border-color .3s}
td{font-size:14.5px;border-bottom:1px solid var(--line);color:var(--ink);transition:background .3s,border-color .3s,color .3s}
tr:last-child td{border-bottom:none}
tr:nth-child(even) td{background:var(--zebra)}
.callout{border-left:4px solid var(--accent);background:var(--card);padding:16px 20px;margin:22px 0;font-style:italic;color:var(--ink-soft);transition:background .3s,border-color .3s,color .3s}
.callout strong{font-style:normal}
ul,ol{margin:0 0 18px;padding-left:26px}
li{margin:0 0 8px}
li::marker{color:var(--accent-deep)}
code{font-family:var(--font-mono);font-size:13px;background:var(--code-bg);color:var(--espresso);padding:1px 6px;border-radius:3px;transition:background .3s,color .3s}
pre{background:var(--pre-bg);color:var(--pre-ink);padding:16px 20px;border-radius:4px;overflow-x:auto;margin:18px 0 24px;font-family:var(--font-mono);font-size:13px;line-height:1.6;transition:background .3s,color .3s}
pre code{background:none;color:inherit;padding:0}
.lede,em,.callout{font-family:var(--font-serif)!important}
.svg-diagram{display:block;text-align:center;margin:24px 0;padding:20px;background:var(--card);border:1px solid var(--line);border-radius:4px;transition:background .3s,border-color .3s}
.svg-diagram svg{max-width:100%;height:auto}
@media(max-width:620px){.wrap{padding:40px 18px 72px}h1{font-size:28px}}"#;
