pub const TOOLBAR_CSS: &str = r#"#tb{
  position:fixed;top:0;left:0;right:0;height:24px;z-index:10;
  display:flex;align-items:center;justify-content:flex-end;gap:7px;
  padding:0 28px;background:var(--page-bg);color:var(--tb-ink);
  font:10px/1 var(--font-ui);
  user-select:none;transition:background .25s,color .25s;
}
#tb .s{width:1px;height:12px;background:var(--tb-line)}
#tb .tb-btn{
  display:inline-flex;align-items:center;justify-content:center;gap:4px;
  height:18px;padding:0 6px 3px;cursor:pointer;
  color:inherit;background:transparent;
  border:1px solid var(--tb-line);border-radius:2px;
  white-space:nowrap;
}
#tb .tb-btn svg{width:13px;height:13px}
#tb .tb-btn.on{background:var(--tb-ink);color:var(--page-bg);border-color:var(--tb-ink)}
#tb .tb-btn.fold{min-width:68px}
#tb .tb-btn.fold::after{
  content:"";width:0;height:0;margin-left:4px;
  border-left:3px solid transparent;border-right:3px solid transparent;
  border-top:4px solid currentColor;
}
#tb .group{position:relative;display:inline-flex}
#tb .menu{
  display:none;position:absolute;top:21px;right:0;z-index:20;
  flex-direction:column;min-width:100%;padding:2px;
  background:var(--page-bg);border:1px solid var(--tb-line);border-radius:2px;
}
#tb .menu.open{display:flex}
#tb .menu button{
  height:16px;padding:0 6px;cursor:pointer;text-align:left;
  font:inherit;color:inherit;background:transparent;border:0;border-radius:1px;
  white-space:nowrap;
}
#tb .menu button:hover,#tb .menu button:focus-visible{background:var(--tb-line)}
#tb .menu button[aria-checked="true"]{background:var(--accent);color:var(--pick-ink)}
#tb .toggle{
  position:relative;width:44px;height:20px;padding:0;cursor:pointer;
  background:transparent;border:0;color:inherit;
}
#tb .toggle-thumb{
  position:absolute;top:2px;left:5px;width:16px;height:16px;
  border-radius:2px;background:var(--thumb);border:1px solid var(--thumb-line);
  transition:transform .25s cubic-bezier(.4,0,.2,1),background .25s,border-color .25s;
}
#tb .toggle.on .toggle-thumb{transform:translateX(17px)}
#tb .toggle svg{
  position:absolute;top:50%;width:12px;height:12px;
  transform:translateY(-50%);pointer-events:none;
  transition:opacity .25s;
}
#tb .toggle .ico-l{left:7px;opacity:1}
#tb .toggle .ico-r{right:8px;opacity:.45}
#tb .toggle.on .ico-l{opacity:.45}
#tb .toggle.on .ico-r{opacity:1}
#tb .sl{display:inline-flex;align-items:center;gap:4px}
#tb .sl svg{width:10px;height:10px;flex-shrink:0}
#tb .sl input[type=range]{
  -webkit-appearance:none;appearance:none;width:48px;height:3px;
  padding:0;margin:0;cursor:pointer;outline:none;
  background:var(--tb-line);border:1px solid var(--tb-line);border-radius:2px;
}
#tb .sl input[type=range]::-webkit-slider-thumb{
  -webkit-appearance:none;appearance:none;width:10px;height:10px;
  background:var(--page-bg);border:1px solid var(--tb-ink);border-radius:2px;
  cursor:pointer;
}
#tb .sl input[type=range]::-moz-range-thumb{
  width:10px;height:10px;background:var(--page-bg);
  border:1px solid var(--tb-ink);border-radius:2px;cursor:pointer;
}
#tb .sl .v{min-width:14px;text-align:center;font-variant-numeric:tabular-nums}
#tb .tb-btn:focus-visible,#tb .toggle:focus-visible,#tb .sl input:focus-visible,#tb .menu button:focus-visible{
  outline:2px solid var(--tb-ink);outline-offset:1px;
}
@media(prefers-reduced-motion:reduce){
  #tb *{transition:none!important}
}"#;
