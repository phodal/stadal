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
        if ((<any>window).stadal) {
          (<any>window).stadal.send_multiple([
              {method: "send_host"},
              {method: "send_memory"}
            ])
        }
      }, 1000);
    }
  })

  ipcRenderer.on('window.blur', (event, arg) => {
    clearInterval(memoryInterval);
    memoryInterval = null;
  })
}

setTimeout(() => {
  startGetMemory();
}, 1000);

const demoButton = document.getElementById('exit-app');
demoButton.addEventListener('click', () => {
  ipcRenderer.send('stadal.exit');
})
