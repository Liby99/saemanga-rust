import Controller from "../lib/controller";
import EventPool from "../lib/event_pool";

type SidebarState = {
  active: boolean,
};

export default class Sidebar extends Controller<SidebarState> {

  $toggle: JQuery<HTMLElement>;
  $rotationToggle: JQuery<HTMLElement>;
  $mask: JQuery<HTMLElement>;
  $sidebar: JQuery<HTMLElement>;

  constructor(root: JQuery<HTMLElement>) {
    super(root);

    // Initialize elements
    this.$toggle = root.children("#sidebar-toggle");
    this.$rotationToggle = this.$toggle.children("i");
    this.$mask = root.children("#sidebar-mask");
    this.$sidebar = root.children("#sidebar");

    // Initialize callbacks
    this.$toggle.click(() => this.toggle());
    this.$mask.click(() => this.close());

    // Global listeners
    EventPool.listen("sidebar.open", () => this.open());
    EventPool.listen("sidebar.close", () => this.close());
  }

  initialState(_: JQuery<HTMLElement>) : SidebarState {
    return { active: false };
  }

  close() {
    this.setState({ active: false });
  }

  open() {
    this.setState({ active: true });
  }

  toggle() {
    const { active } = this.state;
    this.setState({ active: !active });
  }

  update() {
    const { active } = this.state;
    if (active) {
      this.root.addClass("active");
      this.$rotationToggle.addClass("active");
      this.$mask.fadeIn(200);
    } else {
      this.root.removeClass("active");
      this.$rotationToggle.removeClass("active");
      this.$mask.fadeOut(200);
    }
  }
}