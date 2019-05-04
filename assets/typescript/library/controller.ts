import EventPool from "./event_pool";

export default class Controller<State> {

  static observeConfig = { childList: true };

  root: JQuery<HTMLElement>;
  state: State;
  listeners: { [key: string]: Function[] };
  observer: MutationObserver | undefined;
  parent: Node | undefined;

  constructor(root: JQuery<HTMLElement>) {

    // Initialize basic variables
    this.root = root;
    this.state = this.initialState();
    this.listeners = {};

    // Detect mutation
    if (this.root[0].parentNode) {
      this.parent = this.root[0].parentNode;
      this.observer = new MutationObserver((ms) => this.onMutation(ms));
      this.observer.observe(this.parent, Controller.observeConfig);
    }
  }

  protected initialState() : State {
    throw new Error("Component must have an initial state");
  }

  protected listen(evt: string, callback: Function) {
    EventPool.listen(evt, callback);
    if (this.listeners[evt]) {
      this.listeners[evt].push(callback);
    } else {
      this.listeners[evt] = [callback];
    }
  }

  protected setState(newState: Partial<State>, callback?: () => void, noUpdate?: boolean) {
    this.state = { ...this.state, ...newState };
    if (!noUpdate) this.update(callback);
  }

  protected update(callback?: () => void) {
    // Do nothing
  }

  private onMutation(mutations: MutationRecord[]) {
    for (const mutation of mutations) {
      if (mutation.type == "childList") {
        mutation.removedNodes.forEach((n) => {
          if (n == this.root[0]) {
            this.destroy();
          }
        });
      }
    }
  }

  private destroy() {

    // Disconnect observer
    this.observer && this.observer.disconnect();

    // Unlisten all listeners
    for (const evt in this.listeners) {
      for (const listener of this.listeners[evt]) {
        EventPool.unlisten(evt, listener);
      }
    }
  }
}