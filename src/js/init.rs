pub const INIT_JS: &str = r#"(function(){
  MenuBar.init({
    onChange:function(state){ThemeEngine.apply(state);}
  });
})();"#;
