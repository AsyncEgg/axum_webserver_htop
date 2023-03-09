import { h, Component, render } from 'https://esm.sh/preact';
    
setInterval(async () => {
    let responce = await fetch('/api/cpus');
    if ( responce.status !== 200 ){
        throw new Error(`HTTP error! status: ${responce.status}`);
    }

    let json = await responce.json();
    //document.body.textContent = JSON.stringify(json, null, 2);

    const app = h('pre', null, JSON.stringify(json, null, 2));

    render(app, document.body)
}, 1000);