pub const THEME_BOOT_JS: &str = r#"(function(){
  var p=/[?&]mode=(dark|light)\b/.exec(window.location.search||'');
  var mq=window.matchMedia&&window.matchMedia('(prefers-color-scheme: dark)');
  var root=document.documentElement;
  root.setAttribute('data-theme',p?p[1]:(mq&&mq.matches?'dark':'light'));
  root.setAttribute('data-font','sans-serif');
  root.setAttribute('data-print','off');
})();"#;
