import Component from "../lib/component";

type SidebarState = {
  active: boolean,
};

export default class Sidebar extends Component<SidebarState> {

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
  }

  initialState(root: JQuery<HTMLElement>) : SidebarState {
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
      this.$rotationToggle.addClass("active");
      this.$sidebar.addClass("active");
      this.$mask.fadeIn(200);
    } else {
      this.$rotationToggle.removeClass("active");
      this.$sidebar.removeClass("active");
      this.$mask.fadeOut(200);
    }
  }
}