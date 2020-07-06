import {app, BrowserWindow, ipcMain, Menu, nativeImage, NativeImage, Tray} from "electron";
import * as path from "path";

app.allowRendererProcessReuse = false;

const assetsDirectory = path.join(__dirname, '../assets')

let tray: Tray = undefined
let win: BrowserWindow = undefined

function createWindow() {
  win = new BrowserWindow({
    width: 300,
    height: 250,
    frame: false,
    show: false,
    fullscreenable: false,
    resizable: false,
    transparent: true,
    backgroundColor: '#fff',
    webPreferences: {
      backgroundThrottling: false,
      preload: path.join(__dirname, "preload.js"),
      nodeIntegration: true
    },
  });

  win.loadFile(path.join(__dirname, "../views/index.html"));
  win.on('blur', () => {
    if (!win.webContents.isDevToolsOpened()) {
      win.hide()
      win.webContents.send('window.blur')
    }
  })
}

app.dock.hide();
app.on("ready", () => {
  createTray();
  createWindow();

  app.on("activate", function () {
    if (BrowserWindow.getAllWindows().length === 0) createWindow();
  });
});

app.on("window-all-closed", () => {
  if (process.platform !== "darwin") {
    app.quit();
  }
});

const getWindowPosition = () => {
  const windowBounds = win.getBounds()
  const trayBounds = tray.getBounds()

  const x = Math.round(trayBounds.x + (trayBounds.width / 2) - (windowBounds.width / 2))
  const y = Math.round(trayBounds.y + trayBounds.height + 4)

  return {x: x, y: y}
}

const createTray = () => {
  let image_path = path.join(assetsDirectory, 'images/sunTemplate.png');
  tray = new Tray(nativeImage.createFromPath(image_path))
  const contextMenu = Menu.buildFromTemplate([
    {
      label: "Exit", type: "normal", click() {
        app.quit()
      }
    }
  ])
  tray.setContextMenu(contextMenu);

  tray.on('mouse-enter', function (event) {
    toggleWindow()

    if (win.isVisible() && process.defaultApp && event.metaKey) {
      win.webContents.openDevTools();
    }
  })
}

const toggleWindow = () => {
  if (win.isVisible()) {
    win.hide()
    win.webContents.send('window.blur')
  } else {
    showWindow()
    win.webContents.send('window.focus')
  }
}

const showWindow = () => {
  const position = getWindowPosition()
  win.setPosition(position.x, position.y, false)
  win.show()
  win.focus()
}

ipcMain.on('show-window', () => {
  showWindow()
})
