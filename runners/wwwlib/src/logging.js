export function log(target) {
  // const outputDiv = document.getElementById("output");
  const { outputDiv, pre, p, hr } = init();

  if ( target instanceof Error){
    console.warn(target);
    outputDiv.appendChild(hr());
    const error = target;
    // outputDiv.appendChild(pre(error.toString() + error.stack));
    outputDiv.appendChild(pre(error));
    return;
  }
  else{ // string or anything else
    console.log(target);
    outputDiv.appendChild(p(target));
  }
}


function init(){
  return {
    outputDiv : document.getElementById("output"),
    pre,
    p,
    hr,
  };

  function hr(){
    return document.createElement('hr');
  }

  function p(s){
    const el = document.createElement('p');
    el.textContent = s;
    return el;
  }

  function pre(s, css = { 'color' : 'red' }){
    const el = document.createElement('pre');
    // el.style = css;
    // el.style = `{ 'color': 'red' }`;
    Object.assign(el.style, css);
    el.textContent = s;
    return el;
  }
  
}
