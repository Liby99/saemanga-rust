import EventPool from "./event_pool";

export default class Controller<State> {

  static observeConfig = { childList: true };

  root: JQuery<HTMLElement>;
  state: State;
  listeners: { [key: string]: number[] };
  observer: MutationObserver | undefined;
  parent: Node | undefined;

  constructor(root: JQuery<HTMLElement>) {

    // Initialize basic variables
    this.root = root;
    this.state = this.initialState(root);
    this.listeners = {};

    // Detect mutation
    if (this.root[0].parentNode) {
      this.parent = this.root[0].parentNode;
      this.observer = new MutationObserver((ms) => this.onMutation(ms));
      this.observer.observe(this.parent, Controller.observeConfig);
    }
  }

  protected initialState(_: JQuery<HTMLElement>) : State {
    throw new Error("Component must have an initial state");
  }

  protected listen(evt: string, callback: Function) {
    const id = EventPool.listen(evt, callback);
    if (this.listeners[evt]) {
      this.listeners[evt].push(id);
    } else {
      this.listeners[evt] = [id];
    }
  }

  protected setState(newState: Partial<State>) {
    this.state = { ...this.state, ...newState };
    this.update();
  }

  protected update() {
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
      for (const listenerId of this.listeners[evt]) {
        EventPool.unlisten(evt, listenerId);
      }
    }
  }
}