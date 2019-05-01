import Component from "../lib/component";
import EventPool from "../lib/event_pool";

type PanelState = {
  active: boolean,
};

export default class Panel extends Component<PanelState> {

  name: string;

  $mask: JQuery<HTMLElement>;
  $outer: JQuery<HTMLElement>;
  $closeButton: JQuery<HTMLElement>;

  constructor(root: JQuery<HTMLElement>) {
    super(root);

    // Constants
    this.name = root.attr("name") || "";

    // Elements
    this.$mask = root.find(".panel-mask");
    this.$outer = root.find(".panel-outer");
    this.$closeButton = root.find(".panel-head > a");

    // Events
    this.$mask.click(() => this.close());
    this.$outer.click((event) => event.stopPropagation());
    this.$closeButton.click(() => this.close());

    // Listeners
    EventPool.listen(`panel.${this.name}.open`, () => this.open());
    EventPool.listen(`panel.${this.name}.close`, () => this.close());
  }

  initialState(_: JQuery<HTMLElement>) : PanelState {
    return { active: false };
  }

  open() {
    this.setState({ active: true });
  }

  close() {
    this.setState({ active: false });
  }

  update() {
    const { active } = this.state;
    if (active) {
      this.$mask.fadeIn(300).addClass("active");
    } else {
      this.$mask.fadeOut(300).removeClass("active");
    }
  }
}