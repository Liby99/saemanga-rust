import Controller from '../../library/controller';
import EventPool from '../../library/event_pool';

type State = {
  managing: boolean,
};

export default class FollowingManageToggle extends Controller<State> {

  $span: JQuery<HTMLElement>;
  $i: JQuery<HTMLElement>;

  constructor(root: JQuery<HTMLElement>) {
    super(root);

    this.$span = this.root.children("span");
    this.$i = this.root.children("i");

    this.listen("following.manage.finish", () => {
      this.setState({ managing: false });
    });

    this.root.click(() => {
      const { managing } = this.state;
      const newState = !managing;
      this.setState({ managing: newState });
      EventPool.emit('following.manage', newState);
    });
  }

  initialState() : State {
    return { managing: false };
  }

  update() {
    const { managing } = this.state;
    if (managing) {
      this.root.addClass("active");
      this.$span.text("完成");
      this.$i.addClass("active");
    } else {
      this.root.removeClass("active");
      this.$span.text("管理");
      this.$i.removeClass("active");
    }
  }
}