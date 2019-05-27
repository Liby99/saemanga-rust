import $ from 'jquery';

import Controller from '../../library/controller';

type State = {
  fixed: boolean,
};

export default class FixBody extends Controller<State> {

  scrollTop : number = 0;

  constructor(root: JQuery<HTMLElement>) {
    super(root);

    this.listen("sidebar.opened", () => {
      this.setState({ fixed: true });
    });

    this.listen("sidebar.closed", () => {
      this.setState({ fixed: false });
    });

    this.listen("body.fix_at", (scrollTop: number) => {
      console.log(`fixing at ${scrollTop}`);
      this.scrollTop = scrollTop;
      this.root.css({ "margin-top": -this.scrollTop });
    });
  }

  initialState() : State {
    return { fixed: false };
  }

  update() {
    const { fixed } = this.state;
    if (fixed) {
      if (!this.root.hasClass("fixed")) {
        this.scrollTop = $(window).scrollTop() || 0;
        this.root.addClass("fixed").css({
          "margin-top": -this.scrollTop,
          "position": "fixed"
        });
      }
    } else {
      if (this.root.hasClass("fixed")) {
        this.root.removeClass("fixed").css({
          "margin-top": 0,
          "position": "relative"
        });
        window.scrollTo(0, this.scrollTop);
      }
    }
  }
}