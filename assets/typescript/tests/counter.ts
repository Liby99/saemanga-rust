import Controller from '../library/controller';

type State = {
  count: number
};

export default class Counter extends Controller<State> {
  $count: JQuery<HTMLElement>;

  constructor(root: JQuery<HTMLElement>) {
    super(root);
    this.$count = this.root.find(".count");
    this.root.click(() => {
      this.addOne()
    });
  }

  initialState() : State {
    return { count: 0 };
  }

  addOne() {
    const { count } = this.state;
    this.setState({ count: count + 1 });
  }

  update() {
    const { count } = this.state;
    this.$count.text(count);
  }
}