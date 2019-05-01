import Controller from '../lib/controller';

type SelectState = {
  leftSelected: boolean,
};

export default class Select extends Controller<SelectState> {

  $left: JQuery<HTMLElement>;
  $right: JQuery<HTMLElement>;

  constructor(root: JQuery<HTMLElement>) {
    super(root);

    // Elements
    this.$left = root.children().first();
    this.$right = root.children().last();

    // Events
    this.$left.click(() => this.selectLeft());
    this.$right.click(() => this.selectRight());
  }

  initialState(root: JQuery<HTMLElement>) : SelectState {
    return {
      leftSelected: root.children().first().hasClass("active")
    };
  }

  selectLeft() {
    this.setState({ leftSelected: true });
  }

  selectRight() {
    this.setState({ leftSelected: false });
  }

  update() {
    const { leftSelected } = this.state;
    if (leftSelected) {
      this.$left.addClass("active");
      this.$right.removeClass("active");
    } else {
      this.$left.removeClass("active");
      this.$right.addClass("active");
    }
  }
}