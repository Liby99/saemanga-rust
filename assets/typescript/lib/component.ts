export default class Component<State> {
  root: JQuery<HTMLElement>;
  state: State;

  constructor(root: JQuery<HTMLElement>) {
    this.root = root;
    this.state = this.initialState(root);
  }

  initialState(root: JQuery<HTMLElement>) : State {
    throw new Error("Component must have an initial state");
  }

  setState(newState: Partial<State>) {
    this.state = { ...this.state, ...newState };
    this.update();
  }

  update() {
    // Do nothing
  }
}