module.exports = class EventPool {
  constructor() {
    this.listeners = undefined;
  }

  opened() {
    return this.listeners !== undefined;
  }

  open() {
    this.listeners = [];
  }

  close() {
    this.listeners = undefined;
  }

  wait(fn) {
    if (this.opened()) this.listeners.push(fn);
  }

  emit() {
    for (const fn of this.listeners) fn();
    this.listeners = undefined;
  }
}