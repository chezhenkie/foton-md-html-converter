pub const DIAGRAMS_JS: &str = r#"const Diagrams=(function(){
  var NS='http://www.w3.org/2000/svg';
  var MARKER_ID='dg-arrow';

  function ensureMarker(svg){
    if(svg.getElementById(MARKER_ID))return;
    var defs=document.createElementNS(NS,'defs');
    var marker=document.createElementNS(NS,'marker');
    marker.setAttribute('id',MARKER_ID);
    marker.setAttribute('viewBox','0 0 10 10');
    marker.setAttribute('refX','9');
    marker.setAttribute('refY','5');
    marker.setAttribute('markerWidth','6');
    marker.setAttribute('markerHeight','6');
    marker.setAttribute('orient','auto-start-reverse');
    var path=document.createElementNS(NS,'path');
    path.setAttribute('d','M 0 0 L 10 5 L 0 10 z');
    path.setAttribute('fill','var(--accent)');
    marker.appendChild(path);
    defs.appendChild(marker);
    svg.appendChild(defs);
  }

  function center(graph,el){
    var a=el.getBoundingClientRect();
    var b=graph.getBoundingClientRect();
    return{
      x:a.left-b.left+a.width/2,
      y:a.top-b.top+a.height/2,
      h:a.height,
      w:a.width
    };
  }

  function bez(t,p0,c1,c2,p3){
    var mt=1-t,a=mt*mt*mt,b=3*mt*mt*t,c=3*mt*t*t,d=t*t*t;
    return{
      x:a*p0.x+b*c1.x+c*c2.x+d*p3.x,
      y:a*p0.y+b*c1.y+c*c2.y+d*p3.y
    };
  }

  function drawEdge(graph,svg,edge,sourceNode,targetNode){
    var S=center(graph,sourceNode);
    var T=center(graph,targetNode);
    var dx=T.x-S.x;
    var dy=T.y-S.y;
    var up=edge.loop||T.y+S.h<S.y;

    var p0,p3,c1,c2;
    if(Math.abs(dy)<40){
      var dir=dx>=0?1:-1;
      p0={x:S.x+dir*S.w/2,y:S.y};
      p3={x:T.x-dir*T.w/2,y:T.y};
      var midY=(p0.y+p3.y)/2;
      var spread=Math.abs(dx)*0.4;
      c1={x:p0.x+dir*spread,y:midY};
      c2={x:p3.x-dir*spread,y:midY};
    }else if(up){
      p0={x:S.x,y:S.y-S.h/2};
      p3={x:T.x,y:T.y+S.h/2};
      var bulge=90;
      c1={x:p0.x-bulge,y:p0.y-30};
      c2={x:p3.x-bulge,y:p3.y+30};
    }else{
      p0={x:S.x,y:S.y+S.h/2};
      p3={x:T.x,y:T.y-S.h/2};
      var my=(p0.y+p3.y)/2;
      c1={x:p0.x,y:my};
      c2={x:p3.x,y:my};
    }

    var g=document.createElementNS(NS,'g');
    g.dataset.from=edge.s;
    g.dataset.to=edge.t;

    var path=document.createElementNS(NS,'path');
    var d='M '+p0.x+' '+p0.y+' C '+c1.x+' '+c1.y+' '+c2.x+' '+c2.y+' '+p3.x+' '+p3.y;
    path.setAttribute('d',d);
    path.setAttribute('class','diagram-edge'+(edge.loop?' loop':''));
    if(edge.arrow){
      path.setAttribute('marker-end','url(#'+MARKER_ID+')');
    }
    g.appendChild(path);
    svg.appendChild(g);
  }

  function renderDiagram(data){
    var graph=document.getElementById(data.id);
    if(!graph)return;
    var svg=graph.querySelector('.diagram-edges');
    var layers=graph.querySelector('.diagram-layers');
    if(!svg||!layers)return;

    ensureMarker(svg);

    while(svg.firstChild)svg.removeChild(svg.firstChild);

    var nodeMap={};
    var nodes=layers.querySelectorAll('.diagram-node');
    for(var i=0;i<nodes.length;i++){
      var n=nodes[i];
      var key=n.getAttribute('data-dg-node');
      if(key)nodeMap[key]=n;
    }

    for(var i=0;i<data.edges.length;i++){
      var e=data.edges[i];
      var s=nodeMap[e.s];
      var t=nodeMap[e.t];
      if(s&&t)drawEdge(graph,svg,e,s,t);
    }
  }

  function initAll(){
    var q=window.__diagramQueue||[];
    for(var i=0;i<q.length;i++){
      renderDiagram(q[i]);
    }
  }

  function debounce(fn,ms){
    var t;
    return function(){
      clearTimeout(t);
      t=setTimeout(fn,ms);
    };
  }

  if(document.readyState==='loading'){
    document.addEventListener('DOMContentLoaded',initAll);
  }else{
    initAll();
  }
  window.addEventListener('resize',debounce(initAll,100));

  return{initAll,renderDiagram};
})();"#;
