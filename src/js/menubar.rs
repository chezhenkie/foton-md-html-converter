pub const MENUBAR_JS: &str = r#"const MenuBar=(function(){
  let printBtn,modeBtn,fontBtn,bri,sat,briOut,satOut;
  let state={mode:'light',dark:'github',light:'vercel',font:'sans-serif',print:false};
  let onChange=null,manual=false,mq=null;

  function init(opts){
    opts=opts||{};
    onChange=opts.onChange||null;
    mq=window.matchMedia?window.matchMedia('(prefers-color-scheme: dark)'):null;

    state.mode=opts.mode==='dark'||opts.mode==='light'
      ?opts.mode
      :(mq&&mq.matches?'dark':'light');
    if(opts.dark&&ThemeEngine.THEMES.dark[opts.dark])state.dark=opts.dark;
    if(opts.light&&ThemeEngine.THEMES.light[opts.light])state.light=opts.light;
    if(opts.manual)manual=true;
    if(opts.font)state.font=opts.font;

    printBtn=document.getElementById('print');
    modeBtn=document.getElementById('mode');
    fontBtn=document.getElementById('font');
    bri=document.getElementById('brightness');
    briOut=document.getElementById('brightness-value');
    sat=document.getElementById('saturation');
    satOut=document.getElementById('saturation-value');

    printBtn.addEventListener('click',()=>setPrint(!state.print));
    modeBtn.addEventListener('click',()=>{
      manual=true;
      setPrint(false,true);
      setMode(state.mode==='dark'?'light':'dark');
    });
    fontBtn.addEventListener('click',()=>setFont(state.font==='serif'?'sans-serif':'serif'));
    bri.addEventListener('input',()=>{syncSliders();emit();});
    sat.addEventListener('input',()=>{syncSliders();emit();});

    buildMenu('dark-menu','dark');
    buildMenu('light-menu','light');
    document.getElementById('dark-theme').addEventListener('click',e=>{
      e.stopPropagation();
      toggleMenu('dark-theme','dark-menu');
    });
    document.getElementById('light-theme').addEventListener('click',e=>{
      e.stopPropagation();
      toggleMenu('light-theme','light-menu');
    });
    document.getElementById('dark-menu').addEventListener('keydown',menuKey);
    document.getElementById('light-menu').addEventListener('keydown',menuKey);
    document.addEventListener('click',e=>{
      if(!e.target.closest||!e.target.closest('.group'))closeMenus();
    });
    document.addEventListener('keydown',e=>{
      if(e.key!=='Escape')return;
      let openBtn=null;
      [['dark-theme','dark-menu'],['light-theme','light-menu']].forEach(pair=>{
        if(document.getElementById(pair[1]).classList.contains('open')){
          openBtn=document.getElementById(pair[0]);
        }
      });
      closeMenus();
      if(openBtn)openBtn.focus();
    });

    if(mq){
      const follow=e=>{if(!manual)setMode(e.matches?'dark':'light');};
      if(mq.addEventListener)mq.addEventListener('change',follow);
      else if(mq.addListener)mq.addListener(follow);
    }

    bri.value='0';
    sat.value='0';
    syncUI();
    emit();
  }

  function syncSliders(){
    const b=stepOf(bri),s=stepOf(sat);
    bri.value=String(b);
    sat.value=String(s);
    briOut.textContent=String(b);
    satOut.textContent=String(s);
  }

  function syncMenus(){
    [['dark','dark-menu'],['light','light-menu']].forEach(pair=>{
      const menu=document.getElementById(pair[1]);
      Array.prototype.forEach.call(menu.children,row=>{
        row.setAttribute('aria-checked',String(row.textContent===state[pair[0]]));
      });
    });
  }

  function syncUI(){
    const light=state.mode==='light';
    const sans=state.font==='sans-serif';
    modeBtn.classList.toggle('on',light);
    modeBtn.setAttribute('aria-checked',String(light));
    modeBtn.setAttribute('aria-label',light?'Light mode':'Dark mode');
    fontBtn.classList.toggle('on',sans);
    fontBtn.setAttribute('aria-checked',String(sans));
    fontBtn.setAttribute('aria-label',sans?'Sans-serif':'Serif');
    printBtn.classList.toggle('on',state.print);
    printBtn.setAttribute('aria-pressed',String(state.print));
    syncSliders();
    syncMenus();
  }

  function stepOf(el){
    const n=parseInt(el.value,10);
    return isNaN(n)?0:Math.max(-3,Math.min(3,n));
  }

  function copy(){
    return{
      mode:state.mode,
      dark:state.dark,
      light:state.light,
      font:state.font,
      print:state.print,
      brightness:stepOf(bri),
      saturation:stepOf(sat)
    };
  }

  function emit(){
    if(onChange)onChange(copy());
  }

  function setMode(mode){
    state.mode=mode==='dark'?'dark':'light';
    syncUI();
    emit();
  }

  function setTheme(mode,name){
    if(!ThemeEngine.THEMES[mode]||!ThemeEngine.THEMES[mode][name])return;
    manual=true;
    setPrint(false,true);
    state[mode]=name;
    state.mode=mode;
    syncUI();
    emit();
  }

  function setFont(font){
    state.font=font==='serif'?'serif':'sans-serif';
    syncUI();
    emit();
  }

  function setPrint(on,quiet){
    state.print=!!on;
    if(!quiet)syncUI();
    if(!quiet)emit();
  }

  function buildMenu(id,mode){
    const menu=document.getElementById(id);
    Object.keys(ThemeEngine.THEMES[mode]).forEach(name=>{
      const row=document.createElement('button');
      row.type='button';
      row.setAttribute('role','menuitemradio');
      row.setAttribute('aria-checked',String(state[mode]===name));
      row.textContent=name;
      row.addEventListener('click',()=>{
        setTheme(mode,name);
        closeMenus();
        document.getElementById(mode==='dark'?'dark-theme':'light-theme').focus();
      });
      menu.appendChild(row);
    });
  }

  function closeMenus(){
    [['dark-theme','dark-menu'],['light-theme','light-menu']].forEach(pair=>{
      document.getElementById(pair[1]).classList.remove('open');
      document.getElementById(pair[0]).setAttribute('aria-expanded','false');
    });
  }

  function toggleMenu(btnId,menuId){
    const btn=document.getElementById(btnId);
    const menu=document.getElementById(menuId);
    const open=!menu.classList.contains('open');
    closeMenus();
    if(open){
      menu.classList.add('open');
      btn.setAttribute('aria-expanded','true');
      const rows=Array.prototype.slice.call(menu.children);
      const pick=rows.filter(row=>row.getAttribute('aria-checked')==='true')[0]||rows[0];
      if(pick)pick.focus();
    }
  }

  function menuKey(e){
    const rows=Array.prototype.slice.call(this.children);
    const n=rows.length;
    if(!n)return;
    const i=rows.indexOf(document.activeElement);
    if(e.key==='ArrowDown'||e.key==='ArrowUp'){
      e.preventDefault();
      rows[(i+(e.key==='ArrowDown'?1:-1)+n)%n].focus();
    }else if(e.key==='Home'||e.key==='End'){
      e.preventDefault();
      rows[e.key==='Home'?0:n-1].focus();
    }
  }

  function getValues(){
    return copy();
  }

  function setValues(opts){
    opts=opts||{};
    if(opts.mode==='dark'||opts.mode==='light')state.mode=opts.mode;
    if(opts.dark&&ThemeEngine.THEMES.dark[opts.dark])state.dark=opts.dark;
    if(opts.light&&ThemeEngine.THEMES.light[opts.light])state.light=opts.light;
    if(opts.font)state.font=opts.font;
    if(opts.print!==undefined)state.print=!!opts.print;
    if(opts.brightness!==undefined)bri.value=String(Math.max(-3,Math.min(3,Number(opts.brightness)||0)));
    if(opts.saturation!==undefined)sat.value=String(Math.max(-3,Math.min(3,Number(opts.saturation)||0)));
    syncUI();
    emit();
  }

  function isManual(){
    return manual;
  }

  return{init,getValues,setValues,setMode,setTheme,setFont,setPrint,isManual};
})();"#;
