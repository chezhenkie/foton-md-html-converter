pub const THEME_CSS: &str = r#":root{
  --paper:#f4f3ef;
  --ink:#2b2b2a;
  --ink-soft:#575756;
  --espresso:#383837;
  --accent:#b08a2e;
  --accent-deep:#8a6a24;
  --bronze:#7a746b;
  --blue:#4a6d8c;
  --brick:#a84a3a;
  --line:#d9d8d3;
  --card:#fafaf8;
  --zebra:#f7f7f4;
  --code-bg:#eceae4;
  --pre-bg:#343433;
  --shadow-color:rgba(45,45,45,0.06);
  --pre-ink:#f2f2ef;
  --font-serif:Georgia,'Times New Roman',serif;
  --font-ui:'Segoe UI',Helvetica,Arial,sans-serif;
  --font-mono:'Cascadia Code',Consolas,monospace;
}
[data-os="windows"]{
  --font-ui:"Segoe UI Variable Text","Segoe UI",sans-serif;
  --font-serif:Georgia,"Times New Roman",serif;
  --font-mono:"Cascadia Mono","Cascadia Code",Consolas,monospace;
}
[data-os="mac"]{
  --font-ui:-apple-system,"SF Pro Text","Helvetica Neue",sans-serif;
  --font-serif:"New York","Times New Roman",Times,serif;
  --font-mono:"SF Mono",Menlo,Monaco,monospace;
}
[data-os="ios"]{
  --font-ui:-apple-system,"SF Pro Text","Helvetica Neue",sans-serif;
  --font-serif:"New York",Georgia,"Times New Roman",serif;
  --font-mono:"SF Mono",Menlo,monospace;
}
[data-os="android"]{
  --font-ui:Roboto,"Noto Sans",sans-serif;
  --font-serif:"Noto Serif",serif;
  --font-mono:"Roboto Mono","Droid Sans Mono","Noto Sans Mono",monospace;
}
[data-os="linux"]{
  --font-ui:Ubuntu,"Cantarell","Noto Sans",sans-serif;
  --font-serif:"Liberation Serif","Noto Serif",serif;
  --font-mono:"Ubuntu Mono","DejaVu Sans Mono",monospace;
}
body{
  background:var(--paper);
  color:var(--ink);
  font-family:var(--font-serif);
  font-size:16px;
  line-height:1.6;
  transition:background .3s,color .3s;
}
h1,h2,h3,h4,h5,h6{
  color:var(--espresso);
  font-family:var(--font-serif);
  font-weight:400;
  line-height:1.3;
  margin-bottom:0.5em;
}
h1{font-size:2.5rem;letter-spacing:0.02em}
h2{font-size:1.8rem}
h3{font-size:1.4rem}
p{margin-bottom:1em;color:var(--ink)}
p.soft{color:var(--ink-soft)}
a{color:var(--accent);text-decoration:none;transition:color .2s}
a:hover{color:var(--accent-deep)}
em{color:var(--accent-deep);font-style:italic;font-family:var(--font-serif)}
hr{border:none;border-top:1px solid var(--line);margin:2em 0}
.card{background:var(--card);border:1px solid var(--line);border-radius:4px;padding:1.5rem;margin-bottom:1.5rem;box-shadow:0 1px 3px var(--shadow-color)}
.card h3{margin-bottom:0.75rem}
table{width:100%;border-collapse:collapse;margin-bottom:1.5rem;font-family:var(--font-ui);font-size:1rem}
table th{background:var(--espresso);color:var(--paper);padding:0.75rem 1rem;text-align:left;font-weight:500}
table td{padding:0.75rem 1rem;border-bottom:1px solid var(--line)}
table tr:nth-child(even){background:var(--zebra)}
table tr:hover{background:var(--line)}
code{background:var(--code-bg);padding:0.15em 0.4em;border-radius:3px;font-family:var(--font-mono);font-size:0.95em;color:var(--ink)}
pre{background:var(--pre-bg);color:var(--pre-ink);padding:1rem 1.25rem;border-radius:4px;overflow-x:auto;margin-bottom:1.5rem;font-family:var(--font-mono);font-size:0.95rem;line-height:1.5}
pre code{background:transparent;padding:0;color:inherit}
.kicker{font-family:var(--font-ui);font-size:0.75rem;text-transform:uppercase;letter-spacing:0.15em;color:var(--bronze);margin-bottom:0.5rem}
.header-rule{width:60px;height:2px;background:var(--accent);margin:1rem 0 2rem}
[data-font="sans-serif"] body{font-family:var(--font-ui)}
[data-font="sans-serif"] h1,
[data-font="sans-serif"] h2,
[data-font="sans-serif"] h3,
[data-font="sans-serif"] h4,
[data-font="sans-serif"] h5,
[data-font="sans-serif"] h6{font-family:var(--font-ui)}"#;
