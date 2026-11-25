pub const INIT_JS: &str = r#"(function(){
  var params=LinkState.readParams()||{};
  var manual=!!(params.mode||params.dark||params.light);
  MenuBar.init({
    mode:params.mode||undefined,
    dark:params.dark||undefined,
    light:params.light||undefined,
    manual:manual,
    onChange:function(state){
      ThemeEngine.apply(state);
      LinkState.syncLinks(state);
    }
  });
  LinkState.syncLinks(MenuBar.getValues());
})();"#;
