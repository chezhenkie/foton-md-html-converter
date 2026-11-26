pub const TOOLBAR_HTML: &str = r##"<div id="tb" role="group" aria-label="Page controls">
  <button class="tb-btn back" id="back" type="button" disabled aria-label="Go back" title="Go back">
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
      <path d="M20 12H5M11 6l-6 6 6 6"/>
    </svg>
    <span>go back</span>
  </button>
  <button class="tb-btn" id="print" type="button" aria-pressed="false" aria-label="Print friendly" title="Print friendly">
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
      <path d="M6 9V3h12v6"/>
      <path d="M6 18H4a2 2 0 0 1-2-2v-5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v5a2 2 0 0 1-2 2h-2"/>
      <rect x="6" y="14" width="12" height="8" rx="1"/>
    </svg>
    <span>print friendly</span>
  </button>
  <span class="s" aria-hidden="true"></span>
  <button class="toggle on" id="mode" type="button" role="switch" aria-checked="true" aria-label="Light mode" title="Light or dark mode">
    <span class="toggle-thumb" aria-hidden="true"></span>
    <svg class="ico-l" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" aria-hidden="true">
      <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>
    </svg>
    <svg class="ico-r" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" aria-hidden="true">
      <circle cx="12" cy="12" r="5"/>
      <line x1="12" y1="1" x2="12" y2="3"/><line x1="12" y1="21" x2="12" y2="23"/>
      <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"/><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"/>
      <line x1="1" y1="12" x2="3" y2="12"/><line x1="21" y1="12" x2="23" y2="12"/>
      <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"/><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"/>
    </svg>
  </button>
  <button class="toggle on" id="font" type="button" role="switch" aria-checked="true" aria-label="Sans-serif" title="Serif or sans-serif">
    <span class="toggle-thumb" aria-hidden="true"></span>
    <svg class="ico-l" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="1.2" aria-hidden="true">
      <line x1="3" y1="3" x2="11" y2="3"/>
      <line x1="7" y1="3" x2="7" y2="11"/>
      <line x1="3" y1="11" x2="11" y2="11"/>
    </svg>
    <svg class="ico-r" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="1.2" aria-hidden="true">
      <line x1="7" y1="3" x2="7" y2="11"/>
    </svg>
  </button>
  <span class="s" aria-hidden="true"></span>
  <div class="group">
    <button class="tb-btn fold" id="dark-theme" type="button" aria-expanded="false" aria-haspopup="true" aria-controls="dark-menu" title="Dark theme">dark theme</button>
    <div class="menu" id="dark-menu" role="menu" aria-label="Dark theme"></div>
  </div>
  <div class="group">
    <button class="tb-btn fold" id="light-theme" type="button" aria-expanded="false" aria-haspopup="true" aria-controls="light-menu" title="Light theme">light theme</button>
    <div class="menu" id="light-menu" role="menu" aria-label="Light theme"></div>
  </div>
  <span class="s" aria-hidden="true"></span>
  <span class="brk" aria-hidden="true"></span>
  <div class="sl">
    <svg viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1" aria-hidden="true">
      <rect x="1" y="1" width="10" height="10"/>
      <polygon points="1,1 11,1 1,11" fill="currentColor"/>
    </svg>
    <input type="range" id="brightness" min="-3" max="3" step="1" value="0" aria-label="Brightness">
    <span class="v" id="brightness-value" aria-hidden="true">0</span>
  </div>
  <div class="sl">
    <svg viewBox="0 0 12 12" fill="currentColor" aria-hidden="true">
      <circle cx="6" cy="3" r="1.8"/>
      <circle cx="3.5" cy="7.5" r="1.8"/>
      <circle cx="8.5" cy="7.5" r="1.8"/>
    </svg>
    <input type="range" id="saturation" min="-3" max="3" step="1" value="0" aria-label="Saturation">
    <span class="v" id="saturation-value" aria-hidden="true">0</span>
  </div>
</div>"##;

#[cfg(test)]
mod tests {
    use super::TOOLBAR_HTML;

    #[test]
    fn toolbar_carries_every_control() {
        for id in [
            "id=\"back\"",
            "id=\"print\"",
            "id=\"mode\"",
            "id=\"font\"",
            "id=\"dark-theme\"",
            "id=\"light-theme\"",
            "id=\"brightness\"",
            "id=\"saturation\"",
        ] {
            assert!(TOOLBAR_HTML.contains(id), "missing {id}");
        }
    }

    #[test]
    fn back_button_precedes_print_and_starts_disabled() {
        assert!(TOOLBAR_HTML.contains("class=\"tb-btn back\" id=\"back\""));
        assert!(TOOLBAR_HTML.contains("type=\"button\" disabled aria-label=\"Go back\""));
        assert!(TOOLBAR_HTML.contains("stroke-width=\"1.4\""));
        assert!(TOOLBAR_HTML.contains("<path d=\"M20 12H5M11 6l-6 6 6 6\"/>"));
        assert!(TOOLBAR_HTML.contains("<span>go back</span>"));
        let back = TOOLBAR_HTML.find("id=\"back\"").unwrap();
        let print = TOOLBAR_HTML.find("id=\"print\"").unwrap();
        assert!(back < print);
    }
}
