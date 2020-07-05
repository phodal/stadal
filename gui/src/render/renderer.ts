import "reflect-metadata";
import * as path from "path";

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


setInterval(() => {
    (<any>window).stadal.send("send_memory")
}, 1000);
