pub const OS_DETECT_JS: &str = r#"(function(){
  var ua=navigator.userAgent;
  var os='unknown';
  if(/Android/i.test(ua))os='android';
  else if(/iPhone|iPad|iPod/i.test(ua))os='ios';
  else if(/Windows/i.test(ua))os='windows';
  else if(/Macintosh|Mac OS X/i.test(ua))os='mac';
  else if(/Linux/i.test(ua))os='linux';
  document.documentElement.dataset.os=os;
})();"#;
