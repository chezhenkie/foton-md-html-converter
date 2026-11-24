pub const THEME_CSS: &str = r#":root{
  color-scheme:light;
  --page-bg:#F3F3F1;
  --page-ink:#20242A;
  --page-muted:#4E545C;
  --page-line:#C9CCD1;
  --card:#FFFFFF;
  --accent:#9A6400;
  --pick-ink:#000000;
  --tb-ink:#4E545C;
  --tb-line:#C9CCD1;
  --thumb:#CBCBCB;
  --thumb-line:#888888;
  --print-bg:#FFFFFF;
  --print-ink:#000000;
  --print-muted:#333333;
  --print-line:#9A9A9A;
  --font-serif:Georgia,'Times New Roman',serif;
  --font-ui:'Segoe UI',Helvetica,Arial,sans-serif;
  --font-mono:'Cascadia Code',Consolas,monospace;
}
html[data-theme="dark"]{
  color-scheme:dark;
  --page-bg:#0D1117;
  --page-ink:#C9D1D9;
  --page-muted:#8B949E;
  --page-line:#6E7681;
  --card:#010409;
  --accent:#3B82F6;
  --pick-ink:#000000;
  --tb-ink:#8B949E;
  --tb-line:#6E7681;
  --thumb:#2E2E2E;
  --thumb-line:#777777;
}
html[data-print="on"]{color-scheme:light}
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
  margin:0;
  background:var(--page-bg);
  color:var(--page-ink);
  font-family:var(--font-ui);
  font-size:16px;
  line-height:1.6;
  transition:background .25s,color .25s;
}
[data-font="serif"] body{font-family:var(--font-serif)}
h1,h2,h3,h4,h5,h6{
  color:var(--page-ink);
  font-family:inherit;
  font-weight:400;
  line-height:1.3;
  margin:0 0 .5em;
  transition:color .25s;
}
a{color:var(--accent);text-decoration:none;transition:color .2s}
a:hover{text-decoration:underline}
hr{border:none;border-top:1px solid var(--line);margin:2em 0}
@media(prefers-reduced-motion:reduce){*{transition:none!important}}
@media print{
  @page{margin:12mm}
  :root{
    color-scheme:light!important;
    --page-bg:var(--print-bg)!important;
    --page-ink:var(--print-ink)!important;
    --page-muted:var(--print-muted)!important;
    --page-line:var(--print-line)!important;
    --card:var(--print-bg)!important;
    --accent:var(--print-ink)!important;
    --pick-ink:var(--print-bg)!important;
    --tb-ink:var(--print-muted)!important;
    --tb-line:var(--print-line)!important;
  }
  #tb{display:none}
  .wrap{max-width:none;padding:0}
  body{background:var(--page-bg)!important;color:var(--page-ink)!important;-webkit-print-color-adjust:exact;print-color-adjust:exact}
  header,table,pre,.callout,.kicker,.lede,.rule{break-inside:avoid}
}"#;
