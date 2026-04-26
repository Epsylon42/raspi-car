function fetchApi(addr, method) {
    // return fetch(`http://${window.location.hostname}:3000${addr}`, { method });
    return fetch(addr, { method });
}

class ControlState {
    constructor() {
        this.keys = {
            forward: 'KeyW',
            backward: 'KeyS',
            left: 'KeyA',
            right: 'KeyD',
        };
    }

    stop(motor) {
        fetchApi(`/api/motor/${motor}/s`, 'POST');
    }

    run(motor, dir) {
        fetchApi(`/api/motor/${motor}/${dir}`, 'POST');
    }
}

var __controlState = new ControlState();

document.body.onkeydown = ev => {
    switch (ev.code) {
    case __controlState.keys.forward:
        __controlState.run('drive', 'f');
        break;
    case __controlState.keys.backward:
        __controlState.run('drive', 'b');
        break;

    case __controlState.keys.left:
        __controlState.run('turn', 'l');
        break;
    case __controlState.keys.right:
        __controlState.run('turn', 'r');
        break;
    }

    const el = document.querySelector(`.key.${ev.code}`);
    if (el != null) {
        el.style.backgroundColor = 'green';
    }
}

document.body.onkeyup = ev => {
    switch (ev.code) {
    case __controlState.keys.forward:
    case __controlState.keys.backward:
        __controlState.stop('drive');
        break;

    case __controlState.keys.left:
    case __controlState.keys.right:
        __controlState.stop('turn');
        break;
    }

    const el = document.querySelector(`.key.${ev.code}`);
    if (el != null) {
        el.style.backgroundColor = 'gray';
    }
}
