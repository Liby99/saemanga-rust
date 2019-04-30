export default class Component<State> {
  root: JQuery<HTMLElement>;
  state: State;

  constructor(root: JQuery<HTMLElement>, state: State) {
    this.root = root;
    this.state = state;
  }

  setState(newState: Partial<State>) {
    this.state = { ...this.state, ...newState };
    this.update();
  }

  update() {
    // Do nothing
  }
}