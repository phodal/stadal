import "reflect-metadata";
import * as path from "path";
import {CoreMethod} from "./types/core";

const {ipcRenderer} = require('electron')

let Core = require('./core').default;
(<any>window).Core = Core;

const opts = {
  filePath: path.resolve(__dirname, '..'),
  coreOptions: {
    env: Object.assign({RUST_BACKTRACE: "full"}, process.env)
  },
  viewOptions: {}
};

(<any>window).stadal = new Core(opts.coreOptions);

function sendMessage() {
  (<any>window).stadal.send_multiple([
    {method: "send_memory"},
    {method: "send_processes"},
  ])
}

function sendFirstMessages() {
  (<any>window).stadal.send_multiple([
    {method: "send_host"},
    {method: "send_memory"},
    {method: "send_languages"},
    {method: "send_sizes"},
    {method: "send_cpu"},
    {method: "send_disks"},
    {method: "send_processes"},
  ])
}

function startGetData() {
  let memoryInterval = setInterval(() => {
    if ((<any>window).stadal) {
      sendFirstMessages();
    }
  }, 1000);

  ipcRenderer.on('window.focus', (event, arg) => {
    if (!memoryInterval) {
      memoryInterval = setInterval(() => {
        if ((<any>window).stadal) {
          sendMessage();
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
  startGetData();
}, 5000);

const demoButton = document.getElementById('exit-app');
demoButton.addEventListener('click', () => {
  ipcRenderer.send('stadal.exit');
})
