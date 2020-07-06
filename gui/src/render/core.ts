import * as execa from 'execa';
import {ChildProcess} from 'child_process';
import EventEmitter from '../utils/emitter';
import {XI_CORE_BIN, XI_CORE_DIR} from '../utils/environment';
import ViewProxy from './view-proxy';
import {CoreMethod, CoreResponse} from './types/core';
import {container} from "tsyringe";
import Actions from "./actions";

export type CoreOptions = {
  env?: { [key: string]: string | undefined },
  configDir?: string,
};

/**
 * This is a class that manages xi-core. It creates ViewProxies which are simple
 * emitters that link xi-core's internal views with out actual ViewControllers.
 * It is also responsible for encoding/decoding messages to and from xi-core, and
 * managing the spawned process.
 */
export default class Core extends EventEmitter {

  // The spawned child process.
  private child: ChildProcess;

  // References to our ViewProxy classes. Keyed by the view's id.
  private proxies: { [key: string]: ViewProxy };
  private action: Actions;

  /**
   * Create the class.
   * @param  {Object} env The environment map to use when spawning xi-core.
   */
  constructor(opts: CoreOptions) {
    super();

    this.proxies = {};

    // Spawn xi-core.
    this.child = execa(XI_CORE_BIN, [], {env: opts.env || {}});
    this.child.on('close', this.coreClosed.bind(this));

    // Receive messages from xi-core as text.
    this.stdout().setEncoding('utf8');
    this.stderr().setEncoding('utf8');

    // Listen to its streams.
    this.stdout().on('data', this.eventFromCore.bind(this));
    this.stderr().on('data', this.errorFromCore.bind(this));

    this.action = container.resolve(Actions);
    this.send(CoreMethod.CLIENT_STARTED, {
      client_extras_dir: XI_CORE_DIR,
      config_dir: opts.configDir || XI_CORE_DIR
    });
  }

  /**
   * Public API
   */

  /**
   * Serialise and send a message to xi-core.
   * @param  {CoreMethod} method The method to send.
   * @param  {Object} params The method's parameters.
   * @param  {Object} rest   An optional object to extend the top request.
   * @return {Boolean}       Whether or not the message successfully sent.
   */
  public send(method: CoreMethod, params: any = {}, rest: any = {}): boolean {
    const data = {method, params, ...rest};
    try {
      this.stdin().write(`${JSON.stringify(data)}\n`);
      return true;
    } catch (e) {
      console.error(e);
      return false;
    }
  }

  public close() {
    this.child.kill();
  }

  /**
   * Private API
   */

  // Getters for easier access to streams.
  private stdin() {
    return this.child.stdin;
  }

  private stdout() {
    return this.child.stdout;
  }

  private stderr() {
    return this.child.stderr;
  }

  /**
   * Called when we get events from xi-core's `stdout` stream.
   * @param {String} data Raw data emitted from xi-core's stdout.
   */
  private eventFromCore(raw: string) {
    // TODO: refactor - switch?
    // TODO: use message enum
    parseMessages(raw).forEach((msg) => {
      // A new view was created if `msg.result` is set.
      if ('result' in msg) {
        this.proxies[msg.result] = new ViewProxy(this.proxySend, msg.id, msg.result);
        this.emit('new_view', this.proxies[msg.result]);
        return;
      }
      // Otherwise respond to other messages.
      switch (msg.method) {
        case CoreResponse.CONFIG_STATUS: {
          return;
        }
        case CoreResponse.SEND_MEMORY: {
          this.action.display_memory(raw)
          return;
        }
        case CoreResponse.SEND_HOST: {
          this.action.display_host(raw)
          return;
        }
        default: {
          console.warn('Unhandled message from core: ', msg);
        }
      }
    });
  }

  /**
   * Called when we get events from xi-core's `stderr` stream.
   * @param {String} data Raw data emitted from xi-core's stderr.
   */
  private errorFromCore(data: Buffer) {
    console.log(`${data}`);
  }

  /**
   * Called when the xi-core process has closed.
   * @param {Number} code   The exit code of the process.
   * @param {String} signal The close signal (why the process closed).
   */
  private coreClosed(code: number, signal: string) {
    // TODO: if error attempt to reboot core process?
    // TODO: or alternatively just close the app with a dialog error?
    console.log('core proc closed: ', code, signal);
  }

  /**
   * This function is bound to this class and given to each ViewProxy so that
   * they may send messages back to the core process.
   * @param  {CoreMethod} method The method to send.
   * @param  {Object}     params The method's parameters.
   */
  private proxySend = (method: CoreMethod, params: any = {}): void => {
    this.send(method, params);
  }
}

// Helpers ---------------------------------------------------------------------

/**
 * Parses a message (from stdout/err) sent from xi-core. Xi sends multiple
 * messages as serialised JSON objects separated by newlines.
 * @param  {String} string Raw data emitted from xi-core's stdout.
 * @return {Array}         An array containing JSON objects of xi's messages.
 */
function parseMessages(raw: string): Array<any> {
  const parsed = [];
  const lines = raw.split('\n');

  for (let i = 0; i < lines.length; ++i) {
    if (typeof lines[i] !== 'string' || lines[i] === '') {
      continue;
    }
    try {
      parsed.push(JSON.parse(lines[i]));
    } catch (err) {
      console.warn('Error parsing message from core!');
      console.error(err);
    }
  }

  return parsed;
}
