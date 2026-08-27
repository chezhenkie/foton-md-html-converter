pub const TOOLBAR_HTML: &str = r##"<div id="tb">
  <div class="g">
    <div class="toggle" id="i-mode" tabindex="0" role="switch" aria-checked="false" aria-label="Toggle dark/light mode">
      <span class="toggle-thumb"></span>
      <svg class="ico-l" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#888" stroke-width="2" stroke-linecap="round">
        <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>
      </svg>
      <svg class="ico-r" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#888" stroke-width="2" stroke-linecap="round">
        <circle cx="12" cy="12" r="5"/>
        <line x1="12" y1="1" x2="12" y2="3"/><line x1="12" y1="21" x2="12" y2="23"/>
        <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"/><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"/>
        <line x1="1" y1="12" x2="3" y2="12"/><line x1="21" y1="12" x2="23" y2="12"/>
        <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"/><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"/>
      </svg>
    </div>
  </div>
  <div class="g">
    <div class="toggle on" id="i-font" tabindex="0" role="switch" aria-checked="true" aria-label="Toggle serif/sans-serif">
      <span class="toggle-thumb"></span>
      <svg class="ico-l" width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="#888" stroke-width="1.2">
        <line x1="3" y1="3" x2="11" y2="3"/>
        <line x1="7" y1="3" x2="7" y2="11"/>
        <line x1="3" y1="11" x2="11" y2="11"/>
      </svg>
      <svg class="ico-r" width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="#888" stroke-width="1.2">
        <line x1="7" y1="3" x2="7" y2="11"/>
      </svg>
    </div>
  </div>
  <div class="s"></div>
  <div class="g">
    <div class="pal-ctrl" id="i-palette">
      <button class="pal-btn active" data-palette="regular">Regular</button>
      <button class="pal-btn" data-palette="monochrome">Mono</button>
      <button class="pal-btn" data-palette="solarized">Solarized</button>
    </div>
  </div>
  <div class="s"></div>
  <div class="g">
    <svg class="ico-slider" viewBox="0 0 12 12" fill="none" stroke="#888" stroke-width="1">
      <rect x="1" y="1" width="10" height="10"/>
      <polygon points="1,1 11,1 1,11" fill="#888"/>
    </svg>
    <input type="range" id="i-bri" min="-3" max="3" step="1" value="0">
    <span class="v" id="v-bri">0</span>
  </div>
  <div class="s"></div>
  <div class="g" id="g-sat">
    <svg class="ico-slider" viewBox="0 0 12 12" fill="#888">
      <circle cx="6" cy="3" r="1.8"/>
      <circle cx="3.5" cy="7.5" r="1.8"/>
      <circle cx="8.5" cy="7.5" r="1.8"/>
    </svg>
    <input type="range" id="i-sat" min="-3" max="3" step="1" value="0">
    <span class="v" id="v-sat">0</span>
  </div>
</div>"##;
