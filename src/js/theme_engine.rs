pub const THEME_ENGINE_JS: &str = r##"const ThemeEngine=(function(){

  function hexToRgb(hex){
    if(typeof hex!=='string'||hex[0]!=='#'){
      throw new Error('hexToRgb: expected a hex color string starting with "#"');
    }
    const h=hex.substring(1);
    if(h.length!==6||!/^[0-9a-fA-F]{6}$/.test(h)){
      throw new Error('hexToRgb: expected 6 hex digits after "#" (got "'+hex+'")');
    }
    return[
      parseInt(h.substring(0,2),16),
      parseInt(h.substring(2,4),16),
      parseInt(h.substring(4,6),16)
    ];
  }

  function rgbToHex(r,g,b){
    const c=v=>Math.max(0,Math.min(255,Math.round(v)));
    return'#'+[c(r),c(g),c(b)].map(v=>v.toString(16).padStart(2,'0')).join('');
  }

  function rgbToHsl(r,g,b){
    r/=255;g/=255;b/=255;
    const max=Math.max(r,g,b),min=Math.min(r,g,b);
    let h,s;
    const l=(max+min)/2;
    if(max===min){
      h=s=0;
    }else{
      const d=max-min;
      s=l>0.5?d/(2-max-min):d/(max+min);
      switch(max){
        case r:h=((g-b)/d+(g<b?6:0))/6;break;
        case g:h=((b-r)/d+2)/6;break;
        case b:h=((r-g)/d+4)/6;break;
      }
    }
    return[h*360,s,l];
  }

  function hslToRgb(h,s,l){
    h/=360;
    let r,g,b;
    if(s===0){
      r=g=b=l;
    }else{
      const hue2rgb=(p,q,t)=>{
        if(t<0)t+=1;
        if(t>1)t-=1;
        if(t<1/6)return p+(q-p)*6*t;
        if(t<1/2)return q;
        if(t<2/3)return p+(q-p)*(2/3-t)*6;
        return p;
      };
      const q=l<0.5?l*(1+s):l+s-l*s;
      const p=2*l-q;
      r=hue2rgb(p,q,h+1/3);
      g=hue2rgb(p,q,h);
      b=hue2rgb(p,q,h-1/3);
    }
    return[Math.round(r*255),Math.round(g*255),Math.round(b*255)];
  }

  function clampStep(step){
    return Math.max(-3,Math.min(3,Number(step)||0));
  }

  function adjustBrightness(rgb,step){
    step=clampStep(step);
    if(!step)return rgb.slice();
    const factor=1+step*0.08;
    return rgb.map(v=>Math.max(0,Math.min(255,Math.round(v*factor))));
  }

  function adjustSaturation(rgb,step){
    step=clampStep(step);
    if(!step)return rgb.slice();
    const[h,s,l]=rgbToHsl(rgb[0],rgb[1],rgb[2]);
    const newS=Math.max(0,Math.min(1,s*(1+step*0.15)));
    return hslToRgb(h,newS,l);
  }

  function adjustColor(hex,brightness,saturation){
    let rgb=hexToRgb(hex);
    rgb=adjustBrightness(rgb,brightness);
    rgb=adjustSaturation(rgb,saturation);
    return rgbToHex(rgb[0],rgb[1],rgb[2]);
  }

  function inkOn(hex){
    const rgb=hexToRgb(hex);
    const c=rgb.map(v=>{
      const p=v/255;
      return p<=0.04045?p/12.92:Math.pow((p+0.055)/1.055,2.4);
    });
    const l=0.2126*c[0]+0.7152*c[1]+0.0722*c[2];
    return l>0.179?'#000000':'#FFFFFF';
  }

  const THEMES={
    dark:{
      github:{bg:'#0D1117',card:'#010409',ink:'#C9D1D9',muted:'#8B949E',line:'#6E7681',accent:'#3B82F6'},
      harbor:{bg:'#0F1A2B',card:'#212121',ink:'#C9C9C9',muted:'#999999',line:'#737373',accent:'#4FD1C5'},
      solarized:{bg:'#002B36',card:'#073642',ink:'#9DB0B0',muted:'#8FA5AA',line:'#93A1A1',accent:'#D96F3C'},
      oledblack:{bg:'#000000',card:'#1A1A1A',ink:'#EDEDED',muted:'#A8A8A8',line:'#6E6E6E',accent:'#D4D4D4'},
      ayu:{bg:'#0B0E14',card:'#0F131A',ink:'#BFBDB6',muted:'#59C2FF',line:'#6C7380',accent:'#E6B450'}
    },
    light:{
      vercel:{bg:'#F3F3F1',card:'#FFFFFF',ink:'#20242A',muted:'#4E545C',line:'#C9CCD1',accent:'#9A6400'},
      tokyonight:{bg:'#F3F3F1',card:'#F0F4FC',ink:'#232A3D',muted:'#4B5468',line:'#C6CEDF',accent:'#2F5BB7'},
      cursor:{bg:'#F3F3F1',card:'#F0F7F1',ink:'#212A24',muted:'#4C5850',line:'#C5D2C7',accent:'#2A6350'},
      orng:{bg:'#F3F3F1',card:'#FFF6EE',ink:'#2A2420',muted:'#5C4F46',line:'#DBC8B9',accent:'#B4471E'}
    }
  };

  const THUMB={
    dark:{bg:'#2E2E2E',line:'#777777'},
    light:{bg:'#CBCBCB',line:'#888888'}
  };

  function apply(state){
    state=state||{};
    const mode=state.mode==='dark'?'dark':'light';
    const dark=THEMES.dark[state.dark]?state.dark:'github';
    const light=THEMES.light[state.light]?state.light:'vercel';
    const theme=THEMES[mode][mode==='dark'?dark:light];
    const print=!!state.print;
    const font=state.font==='serif'?'serif':'sans-serif';
    const b=clampStep(state.brightness),s=clampStep(state.saturation);

    const root=document.documentElement;
    root.setAttribute('data-theme',mode);
    root.setAttribute('data-font',font);
    root.setAttribute('data-print',print?'on':'off');
    root.setAttribute('data-dark',dark);
    root.setAttribute('data-light',light);

    const vars={};
    if(print){
      vars['--page-bg']='var(--print-bg)';
      vars['--page-ink']='var(--print-ink)';
      vars['--page-muted']='var(--print-muted)';
      vars['--page-line']='var(--print-line)';
      vars['--card']='var(--print-bg)';
      vars['--accent']='var(--print-ink)';
      vars['--pick-ink']='var(--print-bg)';
      vars['--tb-ink']='var(--print-muted)';
      vars['--tb-line']='var(--print-line)';
      vars['--thumb']=THUMB.light.bg;
      vars['--thumb-line']=THUMB.light.line;
    }else{
      const bg=adjustColor(theme.bg,b,s);
      const card=adjustColor(theme.card,b,s);
      const ink=adjustColor(theme.ink,b,s);
      const muted=adjustColor(theme.muted,b,s);
      const line=adjustColor(theme.line,b,s);
      const accent=adjustColor(theme.accent,b,s);
      vars['--page-bg']=bg;
      vars['--page-ink']=ink;
      vars['--page-muted']=muted;
      vars['--page-line']=line;
      vars['--card']=card;
      vars['--accent']=accent;
      vars['--pick-ink']=inkOn(accent);
      vars['--tb-ink']=muted;
      vars['--tb-line']=line;
      vars['--thumb']=THUMB[mode].bg;
      vars['--thumb-line']=THUMB[mode].line;
    }
    for(const[prop,value]of Object.entries(vars)){
      root.style.setProperty(prop,value);
    }
  }

  return{apply,THEMES,inkOn};
})();"##;
