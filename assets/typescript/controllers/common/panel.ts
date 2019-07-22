import Controller from '../../library/controller';

type PanelState = {
  active: boolean,
};

export default class Panel extends Controller<PanelState> {

  name: string;
  noPanelMaskClose: boolean;
  noCloseButton: boolean;

  $mask: JQuery<HTMLElement>;
  $outer: JQuery<HTMLElement>;
  $closeButton: JQuery<HTMLElement>;

  constructor(root: JQuery<HTMLElement>) {
    super(root);

    const props = (root.attr("props") || "").split(" ");
    this.noPanelMaskClose = props.indexOf("no-panel-mask-close") >= 0;
    this.noCloseButton = props.indexOf("no-close-button") >= 0;

    // Constants
    this.name = root.attr("name") || "";

    // Elements
    this.$mask = root.find(".panel-mask");
    this.$outer = root.find(".panel-outer");
    this.$closeButton = root.find(".panel-head > a");

    // If no close button then hide the close button
    this.noCloseButton && this.$closeButton.attr("hidden", "hidden");

    // Events
    this.noPanelMaskClose || this.$mask.click(() => this.close());
    this.$outer.click((event) => event.stopPropagation());
    this.$closeButton.click(() => this.close());

    // Listeners
    this.listen(`panel.${this.name}.open`, () => this.open());
    this.listen(`panel.${this.name}.close`, () => this.close());
  }

  initialState() : PanelState {
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