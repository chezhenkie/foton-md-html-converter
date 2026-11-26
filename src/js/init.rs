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
      if(MenuBar.isManual())LinkState.syncUrl(state);
    }
  });
  var backBtn=document.getElementById('back');
  if(backBtn){
    var syncBack=function(){
      var nav=window.navigation;
      var can;
      if(nav&&typeof nav.canGoBack==='boolean'){
        can=nav.canGoBack;
        // Firefox treats each file:// page as its own origin for the
        // Navigation API, so a page opened from a local link reports no
        // back entry. The via marker proves the link and history.length
        // proves it was not opened in a new tab.
        if(!can&&window.location.protocol==='file:'&&window.history.length>1&&/[?&]via=1\b/.test(window.location.search))can=true;
      }else{
        can=window.history.length>1;
      }
      backBtn.disabled=!can;
      backBtn.title=can?'Go back':'No earlier page';
      backBtn.setAttribute('aria-label',backBtn.title);
    };
    syncBack();
    if(window.addEventListener)window.addEventListener('pageshow',syncBack);
    backBtn.addEventListener('click',function(){window.history.back();});
  }
  LinkState.syncLinks(MenuBar.getValues());
})();"#;
