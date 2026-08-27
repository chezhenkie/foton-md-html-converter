pub const MENUBAR_JS: &str = r#"const MenuBar=(function(){
  let tb,modeToggle,fontToggle,palCtrl,palBtns,iBri,vBri,iSat,vSat,gSat;
  let currentMode='light',currentPalette='regular',currentFont='sans-serif';
  let onChange=null;

  const FONT_DEFAULTS={
    regular:'sans-serif',
    monochrome:'sans-serif',
    solarized:'serif'
  };

  function init(opts){
    opts=opts||{};
    onChange=opts.onChange||null;
    currentMode=opts.mode||'light';
    currentPalette=opts.palette||'regular';
    currentFont=FONT_DEFAULTS[currentPalette]||'sans-serif';

    tb=document.getElementById('tb');
    modeToggle=document.getElementById('i-mode');
    fontToggle=document.getElementById('i-font');
    palCtrl=document.getElementById('i-palette');
    palBtns=palCtrl.querySelectorAll('.pal-btn');
    iBri=document.getElementById('i-bri');
    vBri=document.getElementById('v-bri');
    iSat=document.getElementById('i-sat');
    vSat=document.getElementById('v-sat');
    gSat=document.getElementById('g-sat');

    modeToggle.addEventListener('click',()=>setMode(currentMode==='dark'?'light':'dark'));
    modeToggle.addEventListener('keydown',e=>{
      if(e.key==='Enter'||e.key===' '){e.preventDefault();modeToggle.click();}
    });
    fontToggle.addEventListener('click',()=>setFont(currentFont==='sans-serif'?'serif':'sans-serif'));
    fontToggle.addEventListener('keydown',e=>{
      if(e.key==='Enter'||e.key===' '){e.preventDefault();fontToggle.click();}
    });
    palBtns.forEach(b=>b.addEventListener('click',()=>setPalette(b.dataset.palette)));
    iBri.addEventListener('input',()=>vBri.textContent=iBri.value);
    iBri.addEventListener('change',emit);
    iSat.addEventListener('input',()=>vSat.textContent=iSat.value);
    iSat.addEventListener('change',emit);

    iBri.value=0;
    iSat.value=0;
    vBri.textContent='0';
    vSat.textContent='0';

    syncUI();
  }

  function syncUI(){
    const isLight=currentMode==='light';
    const isSans=currentFont==='sans-serif';
    const isMono=currentPalette==='monochrome';
    modeToggle.classList.toggle('on',isLight);
    modeToggle.setAttribute('aria-checked',String(isLight));
    fontToggle.classList.toggle('on',isSans);
    fontToggle.setAttribute('aria-checked',String(isSans));
    tb.classList.toggle('lt',isLight);
    gSat.classList.toggle('disabled',isMono);
    palBtns.forEach(b=>{
      const pl=b.dataset.palette;
      b.classList.toggle('active',pl===currentPalette);
    });
  }

  function setMode(m){
    currentMode=m;
    syncUI();
    emit();
  }

  function setFont(f){
    currentFont=f;
    syncUI();
    emit();
  }

  function setPalette(pl){
    currentPalette=pl;
    currentFont=FONT_DEFAULTS[pl]||'sans-serif';
    syncUI();
    emit();
  }

  function emit(){
    if(onChange)onChange({
      mode:currentMode,
      palette:currentPalette,
      font:currentFont,
      brightness:parseInt(iBri.value),
      saturation:parseInt(iSat.value)
    });
  }

  function getValues(){
    return{
      mode:currentMode,
      palette:currentPalette,
      font:currentFont,
      brightness:parseInt(iBri.value),
      saturation:parseInt(iSat.value)
    };
  }

  function setValues(opts){
    if(opts.mode!==undefined)currentMode=opts.mode;
    if(opts.palette!==undefined)currentPalette=opts.palette;
    if(opts.font!==undefined)currentFont=opts.font;
    if(opts.brightness!==undefined){iBri.value=opts.brightness;vBri.textContent=opts.brightness;}
    if(opts.saturation!==undefined){iSat.value=opts.saturation;vSat.textContent=opts.saturation;}
    syncUI();
    emit();
  }

  return{init,getValues,setValues,setMode,setPalette,setFont};
})();"#;
