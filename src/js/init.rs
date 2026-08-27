pub const INIT_JS: &str = r#"(function(){
  var prefersDark=window.matchMedia&&window.matchMedia('(prefers-color-scheme: dark)').matches;
  var initialMode=prefersDark?'dark':'light';
  MenuBar.init({
    mode:initialMode,
    palette:'regular',
    onChange:function(opts){
      ThemeEngine.apply(opts.palette,opts.mode,opts.brightness,opts.saturation,opts.font);
    }
  });
  var initial=MenuBar.getValues();
  ThemeEngine.apply(initial.palette,initial.mode,initial.brightness,initial.saturation,initial.font);
})();"#;
