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

  function adjustRgba(rgba,brightness,saturation){
    const match=rgba.match(/rgba?\((\d+),\s*(\d+),\s*(\d+)(?:,\s*([\d.]+))?\)/);
    if(!match)return rgba;
    let rgb=[parseInt(match[1]),parseInt(match[2]),parseInt(match[3])];
    const alpha=match[4]!==undefined?match[4]:'1';
    rgb=adjustBrightness(rgb,brightness);
    rgb=adjustSaturation(rgb,saturation);
    return'rgba('+rgb[0]+','+rgb[1]+','+rgb[2]+','+alpha+')';
  }

  const PALETTES={
    regular:{
      light:{
        '--paper':'#f4f3ef',
        '--ink':'#2b2b2a',
        '--ink-soft':'#575756',
        '--espresso':'#383837',
        '--accent':'#b08a2e',
        '--accent-deep':'#8a6a24',
        '--bronze':'#7a746b',
        '--blue':'#4a6d8c',
        '--brick':'#a84a3a',
        '--line':'#d9d8d3',
        '--card':'#fafaf8',
        '--zebra':'#f7f7f4',
        '--code-bg':'#eceae4',
        '--pre-bg':'#343433',
        '--shadow-color':'rgba(45,45,45,0.06)',
        '--pre-ink':'#f2f2ef',
        '--edge':'#b5b3ab',
        '--node-bg':'#efeee8'
      },
      dark:{
        '--paper':'#0a0c10',
        '--ink':'#d0ccc0',
        '--ink-soft':'#9a9688',
        '--espresso':'#a8b4c8',
        '--accent':'#4a6d8c',
        '--accent-deep':'#5a7d9c',
        '--bronze':'#7a8a9c',
        '--blue':'#4a6d8c',
        '--brick':'#8c6a6a',
        '--line':'#1a2030',
        '--card':'#141820',
        '--zebra':'#1c2230',
        '--code-bg':'#1a2030',
        '--pre-bg':'#0a0c10',
        '--shadow-color':'rgba(0,0,0,0.4)',
        '--pre-ink':'#d0ccc0',
        '--edge':'#8b97ab',
        '--node-bg':'#222a3a'
      }
    },
    solarized:{
      light:{
        '--paper':'#fdf6e3',
        '--ink':'#657b83',
        '--ink-soft':'#93a1a1',
        '--espresso':'#586e75',
        '--accent':'#b58900',
        '--accent-deep':'#cb4b16',
        '--bronze':'#93a1a1',
        '--blue':'#268bd2',
        '--cyan':'#2aa198',
        '--green':'#859900',
        '--red':'#dc322f',
        '--magenta':'#d33682',
        '--violet':'#6c71c4',
        '--line':'#eee8d5',
        '--card':'#eee8d5',
        '--zebra':'#eee8d5',
        '--code-bg':'#eee8d5',
        '--pre-bg':'#eee8d5',
        '--shadow-color':'rgba(0,0,0,0.05)',
        '--pre-ink':'#586e75',
        '--edge':'#93a1a1',
        '--node-bg':'#fdf6e3'
      },
      dark:{
        '--paper':'#002b36',
        '--ink':'#839496',
        '--ink-soft':'#586e75',
        '--espresso':'#93a1a1',
        '--accent':'#b58900',
        '--accent-deep':'#cb4b16',
        '--bronze':'#586e75',
        '--blue':'#268bd2',
        '--cyan':'#2aa198',
        '--green':'#859900',
        '--red':'#dc322f',
        '--magenta':'#d33682',
        '--violet':'#6c71c4',
        '--line':'#073642',
        '--card':'#073642',
        '--zebra':'#073642',
        '--code-bg':'#073642',
        '--pre-bg':'#073642',
        '--shadow-color':'rgba(0,0,0,0.3)',
        '--pre-ink':'#93a1a1',
        '--edge':'#839496',
        '--node-bg':'#0b3f4e'
      }
    },
    monochrome:{
      light:{
        '--paper':'#f5f5f5',
        '--ink':'#333333',
        '--ink-soft':'#666666',
        '--espresso':'#222222',
        '--accent':'#555555',
        '--accent-deep':'#222222',
        '--bronze':'#777777',
        '--blue':'#555555',
        '--brick':'#555555',
        '--line':'#dddddd',
        '--card':'#fafafa',
        '--zebra':'#f0f0f0',
        '--code-bg':'#e8e8e8',
        '--pre-bg':'#333333',
        '--shadow-color':'rgba(0,0,0,0.08)',
        '--pre-ink':'#f0f0f0',
        '--edge':'#c2c2c2',
        '--node-bg':'#efefef'
      },
      dark:{
        '--paper':'#1a1a1a',
        '--ink':'#cccccc',
        '--ink-soft':'#999999',
        '--espresso':'#dddddd',
        '--accent':'#aaaaaa',
        '--accent-deep':'#e0e0e0',
        '--bronze':'#888888',
        '--blue':'#aaaaaa',
        '--brick':'#aaaaaa',
        '--line':'#444444',
        '--card':'#222222',
        '--zebra':'#2a2a2a',
        '--code-bg':'#333333',
        '--pre-bg':'#111111',
        '--shadow-color':'rgba(0,0,0,0.3)',
        '--pre-ink':'#cccccc',
        '--edge':'#cccccc',
        '--node-bg':'#2e2e2e'
      }
    }
  };

  function apply(palette,mode,brightness,saturation,font){
    palette=palette||'regular';
    mode=mode||'light';
    brightness=brightness||0;
    saturation=saturation||0;
    font=font||'sans-serif';

    const theme=PALETTES[palette]&&PALETTES[palette][mode];
    if(!theme){
      console.error('ThemeEngine: unknown palette/mode combination:',palette,mode);
      return;
    }

    const root=document.documentElement;
    root.setAttribute('data-theme',mode);
    root.setAttribute('data-palette',palette);
    root.setAttribute('data-font',font);

    for(const[prop,value]of Object.entries(theme)){
      let adjusted;
      if(value.startsWith('rgba')){
        adjusted=adjustRgba(value,brightness,saturation);
      }else if(value.startsWith('#')){
        adjusted=adjustColor(value,brightness,saturation);
      }else{
        adjusted=value;
      }
      root.style.setProperty(prop,adjusted);
    }
  }

  return{apply,PALETTES};
})();"##;
