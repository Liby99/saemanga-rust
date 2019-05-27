import $ from 'jquery';

import Controller from '../../library/controller';

type State = {
  page: number,
  percInPage: number,
};

export default class Anchorer extends Controller<State> {

  static MARGIN : number = 5;

  constructor(root: JQuery<HTMLElement>) {
    super(root);

    $(window).scroll(() => {
      let changed = false;
      const scrollTop = $(window).scrollTop() || 0;
      this.root.children().each((index, elem) => {
        const off = $(elem).offset(), top = off ? off.top : 0, height = $(elem).height() || 0;
        if (scrollTop >= top && scrollTop <= top + height + Anchorer.MARGIN) {
          changed = true;
          this.setState({ page: index, percInPage: (scrollTop - top) / height });
        }
      });
      if (!changed) {
        this.setState(this.initialState());
      }
    });

    this.listen("setting.scale.update", () => {
      const { page, percInPage } = this.state;
      if (page !== -1) {
        const $elem = this.root.children().eq(page);
        const off = $elem.offset(), top = off ? off.top : 0, height = $elem.height() || 0;
        const scrollTop = top + height * percInPage;
        window.scrollTo(0, scrollTop);
      }
    });
  }

  initialState() : State {
    return {
      page: -1,
      percInPage: 0,
    };
  }
}