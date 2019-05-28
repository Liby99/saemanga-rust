import Controller from '../../library/controller';

type State = {
  sticking: boolean,
};

export default class SidebarToggle extends Controller<State> {
  constructor(root: JQuery<HTMLElement>) {
    super(root);

    this.listen("following.header.release", () => {
      this.setState({ sticking: false });
    });

    this.listen("following.header.stick", () => {
      this.setState({ sticking: true });
    });
  }

  initialState() : State {
    return { sticking: false };
  }

  update() {
    const { sticking } = this.state;
    if (sticking) {
      this.root.addClass("stick");
    } else {
      this.root.removeClass("stick");
    }
  }
}