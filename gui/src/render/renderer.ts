import "reflect-metadata";
import * as path from "path";

const {ipcRenderer} = require('electron')

let Core = require('./core').default;
(<any>window).Core = Core;

const opts = {
  filePath: path.resolve(__dirname, '..'),
  coreOptions: {
    env: Object.assign({RUST_BACKTRACE: 1}, process.env)
  },
  viewOptions: {}
};

(<any>window).stadal = new Core(opts.coreOptions);

function startGetMemory() {
  let memoryInterval: NodeJS.Timeout;

  ipcRenderer.on('window.focus', (event, arg) => {
    if (!memoryInterval) {
      memoryInterval = setInterval(() => {
        (<any>window).stadal.send("send_memory")
      }, 1000);
    }
  })

  ipcRenderer.on('window.blur', (event, arg) => {
    document.getElementById("info").innerText = "window.blur";
    clearInterval(memoryInterval);
    memoryInterval = null;
  })
}

setTimeout(() => {
  startGetMemory();
}, 1000);
