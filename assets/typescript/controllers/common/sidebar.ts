import Controller from "../../library/controller";
import EventPool from "../../library/event_pool";

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
    this.listen("sidebar.open", () => this.open());
    this.listen("sidebar.close", () => this.close());
  }

  initialState() : SidebarState {
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
      this.$mask.fadeIn(200, () => {
        EventPool.emit("sidebar.opened");
      });
    } else {
      this.root.removeClass("active");
      this.$rotationToggle.removeClass("active");
      this.$mask.fadeOut(200, () => {
        EventPool.emit("sidebar.closed");
      });
    }
  }
}