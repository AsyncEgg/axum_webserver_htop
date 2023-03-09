import { h, Component, render } from 'https://esm.sh/preact';
import htm from 'https://esm.sh/htm';

const html = htm.bind(h);

function App(props) {
    return html`
    <div>
    ${props.cpus.map((cpu) => {
        return html`
            <div>${cpu.toFixed(2)}% usage</div>
        `;
    })}
    </div>
    `;
}

setInterval(async () => {
    let responce = await fetch('/api/cpus');
    if (responce.status !== 200) {
        throw new Error(`HTTP error! status: ${responce.status}`);
    }

    let json = await responce.json();
    render(html`<${App} cpus=${json}></${App}>`, document.body)
}, 1000);