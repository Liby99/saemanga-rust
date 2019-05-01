export default class EventPool {
  static events: { [key: string]: [Function] } = {};

  static listen(evt: string, callback: Function) {
    if (this.events[evt]) {
      this.events[evt].push(callback);
    } else {
      this.events[evt] = [callback];
    }
  }

  static dispatch(evt: string, data?: any) {
    if (this.events[evt]) {
      this.events[evt].forEach((callback) => {
        callback(data);
      });
    }
  }
}