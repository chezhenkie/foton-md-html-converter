pub const THEME_BOOT_JS: &str = r#"(function(){
  var mq=window.matchMedia&&window.matchMedia('(prefers-color-scheme: dark)');
  var root=document.documentElement;
  root.setAttribute('data-theme',mq&&mq.matches?'dark':'light');
  root.setAttribute('data-font','sans-serif');
  root.setAttribute('data-print','off');
})();"#;
