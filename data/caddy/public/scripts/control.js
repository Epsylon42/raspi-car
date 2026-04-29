class ControlState {
    constructor() {
        this.keys = {
            forward: 'KeyW',
            backward: 'KeyS',
            left: 'KeyA',
            right: 'KeyD',
        };
        this.pressed = new Set();
        this.ws = null;
        this.interval = null;
    }

    connect() {
        const proto = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
        this.ws = new WebSocket(`${proto}//${window.location.host}/ws`);

        this.ws.onopen = () => {
            this.interval = setInterval(() => this.sendState(), 50);
        };

        this.ws.onclose = () => {
            clearInterval(this.interval);
            this.interval = null;
            setTimeout(() => this.connect(), 1000);
        };

        this.ws.onerror = () => {
            this.ws.close();
        };
    }

    driveCmd() {
        const fwd = this.pressed.has(this.keys.forward);
        const bwd = this.pressed.has(this.keys.backward);
        if (fwd && !bwd) return 'f';
        if (bwd && !fwd) return 'b';
        return 's';
    }

    turnCmd() {
        const left = this.pressed.has(this.keys.left);
        const right = this.pressed.has(this.keys.right);
        if (left && !right) return 'l';
        if (right && !left) return 'r';
        return 's';
    }

    sendState() {
        if (this.ws && this.ws.readyState === WebSocket.OPEN) {
            this.ws.send(`${this.driveCmd()},${this.turnCmd()}`);
        }
    }
}

var __controlState = new ControlState();
__controlState.connect();

document.body.onkeydown = ev => {
    __controlState.pressed.add(ev.code);

    const el = document.querySelector(`.key.${ev.code}`);
    if (el != null) {
        el.style.backgroundColor = 'green';
    }
}

document.body.onkeyup = ev => {
    __controlState.pressed.delete(ev.code);

    const el = document.querySelector(`.key.${ev.code}`);
    if (el != null) {
        el.style.backgroundColor = 'gray';
    }
}
