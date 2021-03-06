import EventEmitter from '../utils/emitter';
import {CoreMethod} from './types/core';

/**
 * A proxy that listens/emits to events in regards to one xi view.
 * This is a simple emitter that channels core events directly to its
 * ViewController.
 */
export default class ViewProxy extends EventEmitter {

  // Our unique view instance id.
  id: number;

  // The view's id generated by xi-core.
  viewId: string;

  // A function given that sends a message to the Core.
  sendToCore: (method: CoreMethod, params: any) => void;

  /**
   * Create the ViewProxy.
   * @param  {Function}     sendToCore Sends a message to the Core.
   * @param  {Number}       id         This classes unique id.
   * @param  {CoreMethod}   viewId     The id of xi-core's corresponding view.
   */
  constructor(sendToCore: (method: CoreMethod, params: any) => void, id: number, viewId: string) {
    super();

    this.id = id;
    this.viewId = viewId;
    this.sendToCore = sendToCore;
  }

  /**
   * Send a message back to xi-core's corresponding view.
   * @param  {CoreMethod} method The method to send.
   * @param  {Object}     params Method parameters.
   */
  send(method: CoreMethod, params: any = {}) {
    params.view_id = this.viewId;
    this.sendToCore(method, params);
  }
}
