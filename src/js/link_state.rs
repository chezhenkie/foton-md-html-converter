pub const LINK_STATE_JS: &str = r#"const LinkState=(function(){
  function readParams(){
    var s=(window.location&&window.location.search)||'';
    if(s.charAt(0)==='?')s=s.slice(1);
    if(!s)return null;
    var out={};
    s.split('&').forEach(function(p){
      var i=p.indexOf('=');
      if(i>0)out[p.slice(0,i)]=p.slice(i+1);
    });
    var mode=(out.mode==='dark'||out.mode==='light')?out.mode:null;
    var dark=ThemeEngine.THEMES.dark[out.dark]?out.dark:null;
    var light=ThemeEngine.THEMES.light[out.light]?out.light:null;
    if(!mode&&!dark&&!light)return null;
    return{mode:mode,dark:dark,light:light};
  }

  function pickQuery(state){
    return '?mode='+(state.mode==='dark'?'dark':'light')+
           '&dark='+state.dark+'&light='+state.light;
  }

  function syncLinks(state){
    var nodes=document.querySelectorAll?document.querySelectorAll('a[href]'):[];
    for(var i=0;i<nodes.length;i++){
      var a=nodes[i];
      var href=a.getAttribute('href')||'';
      if(href.charAt(0)==='#'||href.slice(0,2)==='//')continue;
      if(/^[a-z][a-z0-9+.-]*:/i.test(href))continue;
      var cut=href.search(/[?#]/);
      var path=cut<0?href:href.slice(0,cut);
      var hash=href.indexOf('#')<0?'':href.slice(href.indexOf('#'));
      if(!/\.html$/.test(path))continue;
      a.setAttribute('href',path+pickQuery(state)+'&via=1'+hash);
    }
  }

  function syncUrl(state){
    try{
      if(window.history&&window.history.replaceState){
        var via=/[?&]via=1\b/.test(window.location.search)?'&via=1':'';
        window.history.replaceState(null,'',pickQuery(state)+via+window.location.hash);
      }
    }catch(e){}
  }

  return{readParams:readParams,syncLinks:syncLinks,syncUrl:syncUrl};
})();"#;
