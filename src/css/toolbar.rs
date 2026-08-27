pub const TOOLBAR_CSS: &str = r#"#tb{
  position:fixed;top:0;left:0;right:0;height:24px;z-index:10;
  display:flex;align-items:center;gap:5px;padding:0 8px;justify-content:flex-end;
  background:var(--paper);transition:background .3s;
  border-bottom:1px solid rgba(255,255,255,0.06);
  font:10px/1 var(--font-ui);color:#888;
  user-select:none;
}
#tb.lt{border-bottom-color:rgba(0,0,0,0.08);color:#666}
#tb .g{display:flex;align-items:center;gap:3px}
#tb .s{width:1px;height:12px;background:rgba(255,255,255,0.08)}
#tb.lt .s{background:rgba(0,0,0,0.1)}
.toggle{position:relative;width:44px;height:20px;cursor:pointer;background:transparent}
.toggle-thumb{position:absolute;top:2px;left:6px;width:16px;height:16px;border-radius:2px;background:#2e2e2e;border:1px solid #777;transition:transform .3s cubic-bezier(.4,0,.2,1),background .3s ease}
#tb.lt .toggle-thumb{background:#cbcbcb;border-color:#888}
.toggle.on .toggle-thumb{transform:translateX(16px)}
.toggle svg{position:absolute;top:50%;transform:translateY(-50%);transition:opacity .3s ease;pointer-events:none;width:12px;height:12px}
.toggle .ico-l{left:8px;opacity:1}
.toggle .ico-r{right:8px;opacity:0.5}
.toggle.on .ico-l{opacity:0.5}
.toggle.on .ico-r{opacity:1}
.pal-ctrl{display:flex;border:1px solid rgba(255,255,255,0.15);border-radius:2px;overflow:hidden}
#tb.lt .pal-ctrl{border-color:rgba(0,0,0,0.15)}
.pal-btn{
  height:18px;padding:0 5px;background:transparent;color:inherit;
  border:none;border-right:1px solid rgba(255,255,255,0.15);cursor:pointer;
  font:8px/1 var(--font-ui);text-transform:uppercase;
  letter-spacing:0.4px;transition:background .2s ease,color .2s ease;
}
.pal-btn:last-child{border-right:none}
.pal-btn.active{background:#2e2e2e;color:#ccc}
#tb.lt .pal-btn.active{background:#cbcbcb;color:#333}
.ico-slider{width:10px;height:10px;flex-shrink:0}
#tb input[type=range]{-webkit-appearance:none;appearance:none;width:48px;height:3px;background:#4e4e4e;border-radius:2px;outline:none;cursor:pointer;border:1px solid rgba(255,255,255,0.12)}
#tb input[type=range]::-webkit-slider-thumb{-webkit-appearance:none;appearance:none;width:10px;height:10px;background:#2e2e2e;border:1px solid #777;border-radius:2px;cursor:pointer}
#tb input[type=range]::-moz-range-thumb{width:10px;height:10px;background:#2e2e2e;border:none;border-radius:2px;cursor:pointer}
#tb.lt input[type=range]{background:#cbcbcb;border-color:rgba(0,0,0,0.15)}
#tb.lt input[type=range]::-webkit-slider-thumb{background:#cbcbcb}
#tb.lt input[type=range]::-moz-range-thumb{background:#cbcbcb}
#tb .v{min-width:14px;text-align:center;font-variant-numeric:tabular-nums;font-size:10px}
#g-sat.disabled{opacity:0.3;pointer-events:none}
#g-sat.disabled .ico-slider{opacity:0.5}"#;
