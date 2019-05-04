export default class EventPool {
  static events: { [key: string]: Function[] } = {};

  /**
   * Register an event listener
   * @param evt The event identifier. E.g. "event.is.triggered"
   * @param callback The callback function when the event is being dispatched
   * @returns The id of the callback inside the event listeners
   */
  static listen(evt: string, callback: Function) : number {
    if (this.events[evt]) {
      this.events[evt].push(callback);
      return this.events[evt].length - 1;
    } else {
      this.events[evt] = [callback];
      return 0;
    }
  }

  /**
   * Unlisten an event listener given the listener id previously registered
   * @param evt The event identifier. E.g. "event.is.triggered"
   * @param id The id of the listener
   */
  static unlisten(evt: string, fn: Function) {
    if (this.events[evt]) {
      const index = this.events[evt].indexOf(fn);
      if (index) this.events[evt].splice(index, 1);
    }
  }

  /**
   * Emit an event. This event along with its data will be delegated to any
   * event listener associated with this event name
   * @param evt The event identifier. E.g. "event.is.triggered"
   * @param data The data associated with the event
   */
  static emit(evt: string, data?: any) {
    if (this.events[evt]) {
      this.events[evt].forEach((callback) => {
        callback(data);
      });
    }
  }
}