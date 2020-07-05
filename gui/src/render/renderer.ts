import "reflect-metadata";
import * as path from "path";
const {ipcRenderer} = require('electron')

let Core = require('./core').default;
(<any>window).Core = Core;

const opts = {
    filePath: path.resolve(__dirname, '..', 'xi', 'plugins', 'xi_plugin', 'cache.py'),
    coreOptions: {
        env: Object.assign({RUST_BACKTRACE: 1}, process.env)
    },
    viewOptions: {}
};

(<any>window).stadal = new Core(opts.coreOptions);

function startGetMemory() {
    let memoryInterval: NodeJS.Timeout;

    ipcRenderer.on('window.focus', (event, arg) => {
        document.getElementById("info").innerText = "window.focus";
        if (!memoryInterval) {
            memoryInterval = setInterval(() => {
                (<any>window).stadal.send("send_memory")
            }, 1000);
            console.log(memoryInterval);
        }
    })

    ipcRenderer.on('window.blur', (event, arg) => {
        document.getElementById("info").innerText = "window.blur";
        clearInterval(memoryInterval);
        memoryInterval = null;
    })
}

// waiting for start stadal core process;
setTimeout(()=> {
    startGetMemory();
}, 1000);
